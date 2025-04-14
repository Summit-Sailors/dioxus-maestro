use std::collections::HashMap;

use chrono::{DateTime, Duration, Timelike, Utc};
use serde::{Deserialize, Serialize};

use crate::data::{
	enums::{adjustment::Adjustment, feed::Feed, timeframe::TimeFrame},
	last_quotes::last_quotes_dtos::serialize_vec_to_csv,
};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, bon::Builder)]
pub struct BarsSingleRequestDTO {
	#[builder(default)]
	pub timeframe: TimeFrame,
	#[builder(default)]
	pub feed: Feed,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub limit: Option<usize>,
	// The inclusive start of the interval. Format: RFC-3339 or YYYY-MM-DD.
	// API Default: the beginning of the current day.
	#[builder(default = (Utc::now().with_hour(0).unwrap().with_minute(0).unwrap().with_second(0).unwrap().with_nanosecond(0).unwrap() - Duration::days(30)).to_rfc3339())]
	pub start: String,
	// The inclusive end of the interval. Format: RFC-3339 or YYYY-MM-DD.
	// Default: current time if feed is not sip or if the user has a subscription, otherwise 15 minutes before the current time.
	#[serde(skip_serializing_if = "Option::is_none")]
	pub end: Option<String>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub adjustment: Option<Adjustment>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub page_token: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, bon::Builder)]
pub struct BarsMultiRequestDTO {
	#[serde(flatten)]
	params: BarsSingleRequestDTO,
	#[serde(serialize_with = "serialize_vec_to_csv")]
	symbols: Vec<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct NewBar {
	#[serde(alias = "t")]
	pub time: DateTime<Utc>,
	#[serde(alias = "o")]
	pub open: f32,
	#[serde(alias = "c")]
	pub close: f32,
	#[serde(alias = "h")]
	pub high: f32,
	#[serde(alias = "l")]
	pub low: f32,
	#[serde(alias = "v")]
	pub volume: i32,
	#[serde(alias = "n")]
	pub trade_count: i32,
	#[serde(alias = "vw")]
	pub weighted_average: f32,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct BarsDTO {
	pub bars: Vec<NewBar>,
	pub symbol: String,
	pub next_page_token: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BarsMultiApiDTO {
	pub bars: HashMap<String, Vec<NewBar>>,
	pub next_page_token: Option<String>,
}
