use dioxus::prelude::*;

pub struct UsePagination {
  pub idx: Signal<i32>,
  pub page: Signal<i32>,
  pub page_size: Signal<i32>,
  pub total: Memo<i32>,
  pub counter_label: Memo<String>,
  pub next_idx_disabled: Memo<bool>,
  pub prev_idx_disabled: Memo<bool>,
  pub next_page_disabled: Memo<bool>,
  pub prev_page_disabled: Memo<bool>,
}

impl Clone for UsePagination {
  fn clone(&self) -> Self {
    Self {
      idx: self.idx.clone(),
      page: self.page.clone(),
      page_size: self.page_size.clone(),
      total: self.total.clone(),
      counter_label: self.counter_label.clone(),
      next_idx_disabled: self.next_idx_disabled.clone(),
      prev_idx_disabled: self.prev_idx_disabled.clone(),
      next_page_disabled: self.next_page_disabled.clone(),
      prev_page_disabled: self.prev_page_disabled.clone(),
    }
  }
}


pub fn use_pagination(total: Memo<i32>, page_size: i32) -> (UsePagination, (impl FnMut(), impl FnMut(), impl FnMut(), impl FnMut())) {
  let mut idx = use_signal(|| 0);
  let mut page = use_signal(|| 0);
  let page_size_signal = use_signal(|| page_size);

  let last_page = use_memo(move || {
    let total_val = total();
    if total_val == 0 {
      0
    } else {
      ((total_val as f64) / (page_size_signal() as f64)).ceil() as i32 - 1
    }
  });

  let next_idx_disabled = use_memo(move || idx() >= total() - 1);
  let prev_idx_disabled = use_memo(move || idx() == 0);
  let next_page_disabled = use_memo(move || page() >= last_page());
  let prev_page_disabled = use_memo(move || page() == 0);
  let counter_label = use_memo(move || format!("idx {} of {} - page {} of {}", idx(), total() - 1, page() + 1, last_page() + 1));

  let next_idx = move || {
    if !next_idx_disabled() {
      idx.set(idx() + 1);
      if (idx() + 1) % page_size_signal() == 0 && page() < last_page() {
        page.set(page() + 1);
      }
    }
  };

  let prev_idx = move || {
    if !prev_idx_disabled() {
      idx.set(idx() - 1);
      if idx() % page_size_signal() == page_size_signal() - 1 && page() > 0 {
        page.set(page() - 1);
      }
    }
  };

  let next_page = move || {
    if !next_page_disabled() {
      page.set(page() + 1);
      idx.set(page() * page_size_signal());
    }
  };

  let prev_page = move || {
    if !prev_page_disabled() {
      page.set(page() - 1);
      idx.set(page() * page_size_signal());
    }
  };

  (
    UsePagination { idx, page, page_size: page_size_signal, total, next_idx_disabled, prev_idx_disabled, next_page_disabled, prev_page_disabled, counter_label },
    (next_idx, prev_idx, next_page, prev_page),
  )
}
