use {
	super::{enums::feed::Feed, last_quotes_dtos::QuoteDTO},
	chrono::{DateTime, Utc},
	serde::{Deserialize, Serialize},
};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct QuotesRequestDTO {
	pub start: DateTime<Utc>,
	pub end: DateTime<Utc>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub limit: Option<usize>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub feed: Option<Feed>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub page_token: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct QuotesDTO {
	// #[serde(deserialize_with = "vec_from_str")]
	pub quotes: Vec<QuoteDTO>,
	pub symbol: String,
	pub next_page_token: Option<String>,
}
