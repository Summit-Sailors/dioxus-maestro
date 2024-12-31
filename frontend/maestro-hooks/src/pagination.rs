use dioxus::prelude::*;

pub struct UsePagination {
	pub idx: Signal<i32>,
	pub page: Signal<i32>,
	pub page_size: Signal<i32>,

	pub counter_label: Memo<String>,
	pub can_next_idx: Memo<bool>,
	pub can_prev_idx: Memo<bool>,
	pub can_next_page: Memo<bool>,
	pub can_prev_page: Memo<bool>,
}

impl Clone for UsePagination {
	fn clone(&self) -> Self {
		*self
	}
}

impl Copy for UsePagination {}

pub fn use_pagination(total: Memo<i32>) -> UsePagination {
	let idx = use_signal(|| 0);
	let mut page = use_signal(|| 1);
	let page_size = use_signal(|| 10);
	let mut touched = use_signal(|| false);
	let last_page = use_memo(move || ((total() as f64) / (page_size() as f64)).ceil() as i32);
	let can_next_idx = use_memo(move || idx() != total());
	let can_prev_idx = use_memo(move || idx() != 0);
	let can_next_page = use_memo(move || page() != last_page());
	let can_prev_page = use_memo(move || page() != 1);
	let counter_label = use_memo(move || format!("{} of {}", page(), total()));

	use_effect(move || {
		if idx() != 0 {
			touched.set(true);
		}
	});
	use_effect(move || {
		if touched() && (idx() % page_size() == 0) {
			page += 1;
		}
		if idx() > page_size() && idx() % page_size() == page_size() - 1 {
			page -= 1;
		}
	});
	UsePagination { idx, page, page_size, can_next_idx, can_prev_idx, can_next_page, can_prev_page, counter_label }
}
