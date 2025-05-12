use std::collections::HashMap;

use chrono::{DateTime, Duration, Timelike, Utc};
use serde::{Deserialize, Serialize};

use crate::data::{enums::feed::Feed, last_quotes::last_quotes_dtos::serialize_vec_to_csv};

// Historical trades - Single symbol request
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, bon::Builder)]
pub struct TradesSingleRequestDTO {
	// The inclusive start of the interval. Format: RFC-3339 or YYYY-MM-DD.
	// API Default: the beginning of the current day.
	#[builder(default = (Utc::now().with_hour(0).unwrap().with_minute(0).unwrap().with_second(0).unwrap().with_nanosecond(0).unwrap() - Duration::days(30)).to_rfc3339())]
	pub start: String,

	// The inclusive end of the interval. Format: RFC-3339 or YYYY-MM-DD.
	// Default: current time if feed is not sip or if the user has a subscription,
	// otherwise 15 minutes before the current time.
	#[serde(skip_serializing_if = "Option::is_none")]
	pub end: Option<String>,

	#[serde(skip_serializing_if = "Option::is_none")]
	pub asof: Option<String>,

	#[serde(skip_serializing_if = "Option::is_none")]
	pub currency: Option<String>,

	#[builder(default = Feed::Sip)]
	pub feed: Feed,

	#[serde(skip_serializing_if = "Option::is_none")]
	pub limit: Option<usize>,

	#[serde(skip_serializing_if = "Option::is_none")]
	pub sort: Option<String>,

	#[serde(skip_serializing_if = "Option::is_none")]
	pub page_token: Option<String>,
}

// Historical trades - Multi-symbol request
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, bon::Builder)]
pub struct TradesMultiRequestDTO {
	// Required parameter, must be first
	#[serde(serialize_with = "serialize_vec_to_csv")]
	pub symbols: Vec<String>,

	// The inclusive start of the interval. Format: RFC-3339 or YYYY-MM-DD.
	// API Default: the beginning of the current day.
	#[builder(default = (Utc::now().with_hour(0).unwrap().with_minute(0).unwrap().with_second(0).unwrap().with_nanosecond(0).unwrap() - Duration::days(30)).to_rfc3339())]
	pub start: String,

	// The inclusive end of the interval. Format: RFC-3339 or YYYY-MM-DD.
	// Default: current time if feed is not sip or if the user has a subscription,
	// otherwise 15 minutes before the current time.
	#[serde(skip_serializing_if = "Option::is_none")]
	pub end: Option<String>,

	#[serde(skip_serializing_if = "Option::is_none")]
	pub asof: Option<String>,

	#[serde(skip_serializing_if = "Option::is_none")]
	pub currency: Option<String>,

	#[builder(default = Feed::Sip)]
	pub feed: Feed,

	#[serde(skip_serializing_if = "Option::is_none")]
	pub limit: Option<usize>,

	#[serde(skip_serializing_if = "Option::is_none")]
	pub sort: Option<String>,

	#[serde(skip_serializing_if = "Option::is_none")]
	pub page_token: Option<String>,
}

// Latest trades - Request DTO
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, bon::Builder)]
pub struct TradesLatestRequestDTO {
	// Required parameter, must be first
	#[serde(serialize_with = "serialize_vec_to_csv")]
	pub symbols: Vec<String>,

	#[builder(default = Feed::Sip)]
	pub feed: Feed,

	#[serde(skip_serializing_if = "Option::is_none")]
	pub currency: Option<String>,
}

// Trade data structure
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TradeProps {
	#[serde(alias = "t")]
	pub timestamp: DateTime<Utc>,

	#[serde(alias = "x")]
	pub exchange_code: String,

	#[serde(alias = "p")]
	pub price: f64,

	#[serde(alias = "s")]
	pub size: u32,

	#[serde(alias = "i")]
	pub trade_id: u64,

	#[serde(alias = "c")]
	pub condition_flags: Vec<String>,

	#[serde(alias = "z")]
	pub tape: String,

	#[serde(alias = "u", skip_serializing_if = "Option::is_none")]
	pub update: Option<String>,
}

// Response for single symbol historical trades
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TradesSingleResponseDTO {
	pub symbol: String,
	pub trades: Vec<TradeProps>,
	pub currency: Option<String>,
	pub next_page_token: Option<String>,
}

// Response for multi-symbol historical trades
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TradesMultiResponseDTO {
	pub trades: HashMap<String, Vec<TradeProps>>,
	pub currency: Option<String>,
	pub next_page_token: Option<String>,
}

// Response for latest trades
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TradesLatestResponseDTO {
	pub trades: HashMap<String, TradeProps>,
	pub currency: Option<String>,
}
