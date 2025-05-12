use std::collections::HashMap;

use chrono::{DateTime, Duration, NaiveDate, Timelike, Utc};
use serde::{Deserialize, Serialize};

use crate::data::{enums::feed::Feed, last_quotes::last_quotes_dtos::serialize_vec_to_csv};

// Single symbol auction request
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, bon::Builder)]
pub struct AuctionsSingleRequestDTO {
	// The inclusive start of the interval. Format: RFC-3339 or YYYY-MM-DD.
	// API Default: the beginning of the current day.
	#[builder(default = (Utc::now().with_hour(0).unwrap().with_minute(0).unwrap().with_second(0).unwrap().with_nanosecond(0).unwrap() - Duration::days(30)).to_rfc3339())]
	pub start: String,

	// The inclusive end of the interval. Format: RFC-3339 or YYYY-MM-DD.
	// Default: current time if feed is not sip or if the user has a subscription, otherwise 15 minutes before the current time.
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

// Multi symbol auction request
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, bon::Builder)]
pub struct AuctionsMultiRequestDTO {
	// Required parameter, must be first
	#[serde(serialize_with = "serialize_vec_to_csv")]
	pub symbols: Vec<String>,

	// The inclusive start of the interval. Format: RFC-3339 or YYYY-MM-DD.
	// API Default: the beginning of the current day.
	#[builder(default = (Utc::now().with_hour(0).unwrap().with_minute(0).unwrap().with_second(0).unwrap().with_nanosecond(0).unwrap() - Duration::days(30)).to_rfc3339())]
	pub start: String,

	// The inclusive end of the interval. Format: RFC-3339 or YYYY-MM-DD.
	// Default: current time if feed is not sip or if the user has a subscription, otherwise 15 minutes before the current time.
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

// Common structure for auction properties
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct AuctionProps {
	#[serde(alias = "t")]
	pub timestamp: DateTime<Utc>,

	#[serde(alias = "x")]
	pub exchange_code: String,

	#[serde(alias = "p")]
	pub auction_price: f64,

	#[serde(alias = "s")]
	pub auction_trade_size: Option<i64>,

	#[serde(alias = "c")]
	pub condition_flag: String, // condition flag indicating that this is an auction
}

// Individual auction data point
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Auction {
	#[serde(alias = "d")]
	pub date: NaiveDate,

	#[serde(alias = "o")]
	pub opening_auctions: Vec<AuctionProps>,

	#[serde(alias = "c")]
	pub closing_auctions: Vec<AuctionProps>,
}

// Response for single symbol auction
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct AuctionsSingleResponseDTO {
	pub symbol: String,
	pub auctions: Vec<Auction>,
	pub currency: Option<String>,
	pub next_page_token: Option<String>,
}

// Response for multi-symbol auction
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuctionsMultiResponseDTO {
	pub auctions: HashMap<String, Vec<Auction>>,
	pub currency: Option<String>,
	pub next_page_token: Option<String>,
}
