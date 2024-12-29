use dioxus::prelude::*;

#[derive(Clone, Debug, PartialEq)]
pub enum PaginationError {
	FailedToRead,
	FailedToSet,
	NotAvailable,
}

pub struct UsePagination {
	pub idx: Signal<i32>,
	pub total: Memo<i32>,
	pub counter_label: Memo<String>,
	pub page: Signal<i32>,
	pub page_size: Signal<i32>,
	pub can_next: Signal<bool>,
	pub can_prev: Signal<bool>,
}

impl UsePagination {
	pub fn next_index(&mut self) {
		self.idx += 1;
		let idx = *self.idx.read();
		if idx != 0 && idx % *self.page_size.read() == 0 {
			self.next_page();
		}
	}

	pub fn prev_index(&mut self) {
		self.idx -= 1;
		let idx = *self.idx.read();
		let page_size = *self.page_size.read();
		if idx % page_size == page_size - 1 {
			self.prev_page();
		}
	}

	pub fn next_page(&mut self) {
		self.page += 1;
		self.can_next.set(*self.page.read() != *self.total.read());
	}

	pub fn prev_page(&mut self) {
		self.page -= 1;
		self.can_prev.set(*self.page.read() != 0);
	}
}

impl Clone for UsePagination {
	fn clone(&self) -> Self {
		*self
	}
}

impl Copy for UsePagination {}

#[bon::builder]
pub fn use_pagination(
	total: Memo<i32>,
	#[builder(with = |idx: i32| Signal::new(idx), default = Signal::new(0))] idx: Signal<i32>,
	#[builder(with = |page: i32| Signal::new(page), default = Signal::new(1))] page: Signal<i32>,
	#[builder(with = |page_size: i32| Signal::new(page_size), default = Signal::new(10))] page_size: Signal<i32>,
	#[builder(with = |can_next: bool| Signal::new(can_next), default = Signal::new(true))] can_next: Signal<bool>,
	#[builder(with = |can_prev: bool| Signal::new(can_prev), default = Signal::new(false))] can_prev: Signal<bool>,
) -> UsePagination {
	let counter_label = use_memo(move || format!("{} of {}", page, total));
	UsePagination { idx, total, counter_label, page, page_size, can_next, can_prev }
}
