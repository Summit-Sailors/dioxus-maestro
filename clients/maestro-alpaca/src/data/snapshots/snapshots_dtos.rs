use std::collections::HashMap;

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

use crate::data::{common::Bar, enums::feed::Feed};

// Latest Quote Structure
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct LatestQuote {
	#[serde(alias = "t")]
	pub timestamp: DateTime<Utc>,
	#[serde(alias = "bx")]
	pub bid_exchange: String,
	#[serde(alias = "bp")]
	pub bid_price: f64,
	#[serde(alias = "bs")]
	pub bid_size: u32,
	#[serde(alias = "ax")]
	pub ask_exchange: String,
	#[serde(alias = "ap")]
	pub ask_price: f64,
	#[serde(alias = "as")]
	pub ask_size: u32,
	#[serde(alias = "c")]
	pub condition_flags: Vec<String>,
	#[serde(alias = "z")]
	pub tape: String,
}

// Latest Trade Structure
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct LatestTrade {
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
	#[serde(alias = "u")]
	pub update: Option<String>,
}

// Multi-Symbol Snapshots Request DTO
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, bon::Builder)]
pub struct SnapshotsMultiRequestDTO {
	// Required parameter, comma-separated list of symbols
	#[serde(serialize_with = "crate::data::last_quotes::last_quotes_dtos::serialize_vec_to_csv")]
	pub symbols: Vec<String>,

	#[builder(default = Feed::Sip)]
	pub feed: Feed,

	#[serde(skip_serializing_if = "Option::is_none")]
	pub currency: Option<String>,
}

// Single Symbol Snapshot Request DTO
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, bon::Builder)]
pub struct SnapshotSingleRequestDTO {
	#[builder(default = Feed::Sip)]
	pub feed: Feed,

	#[serde(skip_serializing_if = "Option::is_none")]
	pub currency: Option<String>,
}

// Multi-Symbol Snapshots Response DTO
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SnapshotsMultiResponseDTO {
	pub snapshots: HashMap<String, SnapshotData>,
}

// Single Symbol Snapshot Response DTO
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SnapshotSingleResponseDTO {
	pub symbol: String,
	pub currency: Option<String>,
	#[serde(flatten)]
	pub snapshot: SnapshotData,
}

// Snapshot Data Structure (common to both single and multi symbol responses)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SnapshotData {
	#[serde(alias = "dailyBar")]
	pub daily_bar: Option<Bar>,
	#[serde(alias = "latestQuote")]
	pub latest_quote: Option<LatestQuote>,
	#[serde(alias = "latestTrade")]
	pub latest_trade: Option<LatestTrade>,
	#[serde(alias = "minuteBar")]
	pub minute_bar: Option<Bar>,
	#[serde(alias = "prevDailyBar")]
	pub prev_daily_bar: Option<Bar>,
}
