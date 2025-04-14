use dioxus::prelude::*;

#[derive(Debug, Copy)]
pub struct Pagination {
	pub idx: Signal<i32>,
	pub page: Signal<i32>,
	pub page_size: Signal<i32>,
	pub total: Signal<i32>,
	pub counter_label: Memo<String>,
	pub next_idx_disabled: Memo<bool>,
	pub prev_idx_disabled: Memo<bool>,
	pub next_page_disabled: Memo<bool>,
	pub prev_page_disabled: Memo<bool>,
	pub touched: Signal<bool>,
	pub items_in_current_page: Memo<i32>,
}

impl Pagination {
	pub fn mark_touched(&mut self) {
		if !(self.touched)() {
			self.touched.set(true);
		}
	}

	pub fn next_idx(&mut self) {
		self.mark_touched();
		if !(self.next_idx_disabled)() {
			self.idx.set((self.idx)() + 1);
			let last_page = (((self.total)() as f64) / ((self.page_size)() as f64)).ceil() as i32 - 1;
			if ((self.idx)() + 1) % (self.page_size)() == 0 && (self.page)() < last_page {
				self.page.set((self.page)() + 1);
			}
		}
	}

	pub fn prev_idx(&mut self) {
		self.mark_touched();
		if !(self.prev_idx_disabled)() {
			self.idx.set((self.idx)() - 1);
			if (self.idx)() % (self.page_size)() == (self.page_size)() - 1 && (self.page)() > 0 {
				self.page.set((self.page)() - 1);
			}
		}
	}

	pub fn next_page(&mut self) {
		self.mark_touched();
		if !(self.next_page_disabled)() {
			self.page.set((self.page)() + 1);
			self.idx.set((self.page)() * (self.page_size)());
		}
	}

	pub fn prev_page(&mut self) {
		self.mark_touched();
		if !(self.prev_page_disabled)() {
			self.page.set((self.page)() - 1);
			self.idx.set((self.page)() * (self.page_size)());
		}
	}

	pub fn set_page_size(&mut self, new_size: i32) {
		if new_size > 0 {
			self.page_size.set(new_size);
			// recalculate current page and index to maintain position
			let current_idx = (self.idx)();
			self.page.set(current_idx / new_size);
			self.idx.set((self.page)() * new_size);
		}
	}
}

impl Clone for Pagination {
	fn clone(&self) -> Self {
		*self
	}
}

pub fn use_pagination(total: Signal<i32>, page_size: i32) -> Pagination {
	let idx = use_signal(|| 0);
	let page = use_signal(|| 0);
	let page_size_signal = use_signal(|| page_size);
	let touched = use_signal(|| false);

	let last_page = use_memo(move || {
		let total_val = total();
		if total_val == 0 { 0 } else { ((total_val as f64) / (page_size_signal() as f64)).ceil() as i32 - 1 }
	});

	let items_in_current_page = use_memo(move || {
		let total_val = total();
		let current_page = page();
		let page_size = page_size_signal();

		if current_page == last_page() { total_val - (current_page * page_size) } else { page_size }
	});

	let next_idx_disabled = use_memo(move || idx() >= total() - 1);
	let prev_idx_disabled = use_memo(move || idx() == 0);
	let next_page_disabled = use_memo(move || page() >= last_page());
	let prev_page_disabled = use_memo(move || page() == 0);

	let counter_label = use_memo(move || format!("Page {} of {}", page() + 1, last_page() + 1,));

	Pagination {
		idx,
		page,
		page_size: page_size_signal,
		total,
		next_idx_disabled,
		prev_idx_disabled,
		next_page_disabled,
		prev_page_disabled,
		counter_label,
		touched,
		items_in_current_page,
	}
}
