use dioxus::prelude::*;

pub struct UsePagination {
	pub idx: Signal<i32>,
	pub page: Signal<i32>,
	pub page_size: Signal<i32>,

	pub counter_label: Memo<String>,
	pub next_idx_disabled: Memo<bool>,
	pub prev_idx_disabled: Memo<bool>,
	pub next_page_disabled: Memo<bool>,
	pub prev_page_disabled: Memo<bool>,
}

impl Clone for UsePagination {
	fn clone(&self) -> Self {
		*self
	}
}

impl Copy for UsePagination {}

pub fn use_pagination(total: Memo<i32>) -> (UsePagination, (impl FnMut(), impl FnMut(), impl FnMut(), impl FnMut())) {
	let mut idx = use_signal(|| 0);
	let mut page = use_signal(|| 0);
	let page_size = use_signal(|| 10);
	let mut touched = use_signal(|| false);

	let last_page = use_memo(move || ((total() as f64) / (page_size() as f64)).ceil() as i32 - 1);
	let next_idx_disabled = use_memo(move || idx() == total() - 1);
	let prev_idx_disabled = use_memo(move || idx() == 0);
	let next_page_disabled = use_memo(move || page() == last_page());
	let prev_page_disabled = use_memo(move || page() == 0);
	let counter_label = use_memo(move || format!("idx {} of {} - page {} of {}", idx(), total() - 1, page(), last_page()));
	let next_idx = move || {
		if !touched() {
			touched.set(true);
		}
		if !next_idx_disabled() {
			let mut idx = idx.write();
			*idx += 1;
			if !next_page_disabled() && *idx % page_size() == 0 {
				page += 1;
			}
		}
	};
	let next_page = move || {
		if !touched() {
			touched.set(true);
		}
		if !next_page_disabled() {
			let mut page = page.write();
			*page += 1;
			idx.set(*page * page_size());
		}
	};
	let prev_page = move || {
		if !touched() {
			touched.set(true);
		}
		if !next_page_disabled() {
			let mut page = page.write();
			*page -= 1;
			idx.set(*page * page_size());
		}
	};

	let prev_idx = move || {
		if !touched() {
			touched.set(true);
		}
		if !prev_idx_disabled() {
			let mut idx = idx.write();
			*idx -= 1;
			if !prev_page_disabled() && *idx % page_size() == page_size() - 1 {
				page += 1;
			}
		}
	};

	(
		UsePagination { idx, page, page_size, next_idx_disabled, prev_idx_disabled, next_page_disabled, prev_page_disabled, counter_label },
		(next_idx, prev_idx, next_page, prev_page),
	)
}
