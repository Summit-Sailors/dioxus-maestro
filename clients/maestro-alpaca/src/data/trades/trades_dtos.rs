use {
	super::enums::feed::Feed,
	bigdecimal::BigDecimal,
	chrono::{DateTime, Utc},
	serde::{Deserialize, Serialize},
};

#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
pub struct TradesRequestDTO {
	#[serde(skip_serializing_if = "Option::is_none")]
	pub start: Option<DateTime<Utc>>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub end: Option<DateTime<Utc>>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub limit: Option<usize>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub feed: Option<Feed>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub page_token: Option<String>,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq)]
pub struct TradeDTO {
	#[serde(rename = "t")]
	pub timestamp: DateTime<Utc>,
	#[serde(rename = "p")]
	pub price: BigDecimal,
	#[serde(rename = "s")]
	pub size: usize,
}

#[derive(Debug, Deserialize, Eq, PartialEq)]
pub struct TradesDTO {
	pub trades: Vec<TradeDTO>,
	pub symbol: String,
	pub next_page_token: Option<String>,
}
