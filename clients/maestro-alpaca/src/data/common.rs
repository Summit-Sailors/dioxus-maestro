use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

// Common Bar Structure for Daily, Minute, and Previous Daily Bars
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Bar {
	#[serde(alias = "t")]
	pub timestamp: DateTime<Utc>,
	#[serde(alias = "o")]
	pub open: f64,
	#[serde(alias = "h")]
	pub high: f64,
	#[serde(alias = "l")]
	pub low: f64,
	#[serde(alias = "c")]
	pub close: f64,
	#[serde(alias = "v")]
	pub volume: i64,
	#[serde(alias = "n")]
	pub trade_count: i64,
	#[serde(alias = "vw")]
	pub volume_weighted_avg_price: f64,
}
