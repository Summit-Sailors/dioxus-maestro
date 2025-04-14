use serde::{Deserialize, Serialize};
use validator::Validate;

#[derive(Debug, Deserialize, Validate)]
pub struct PaginationRequestDTO<T> {
	#[validate(range(min = 1))]
	pub page: i32,
	#[validate(range(min = 1, max = 100))]
	pub page_size: i32,
	#[serde(flatten)]
	pub query: T,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PaginatedResultDTO<T> {
	pub records: Vec<T>,
	pub total_pages: i64,
	pub has_more: bool,
	pub current_page: i64,
}

impl<T> PaginatedResultDTO<T> {
	pub fn new(records: Vec<T>, total_pages: i64, current_page: i64) -> Self {
		Self { records, total_pages, has_more: current_page < total_pages, current_page }
	}
}
