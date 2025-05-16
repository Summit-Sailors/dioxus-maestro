use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

use super::enums::market_data::{AssetClass, MarketDataClass};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Bar {
	#[serde(alias = "t")]
	pub timestamp: DateTime<Utc>,
	#[serde(alias = "o")]
	pub open: f32,
	#[serde(alias = "h")]
	pub high: f32,
	#[serde(alias = "l")]
	pub low: f32,
	#[serde(alias = "c")]
	pub close: f32,
	#[serde(alias = "v")]
	pub volume: i32,
	#[serde(alias = "n")]
	pub trade_count: i32,
	#[serde(alias = "vw")]
	pub volume_weighted_avg_price: f32,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, bon::Builder, Default)]
pub struct MarketData {
	#[builder(default)]
	pub asset_class: AssetClass,
	#[builder(default)]
	pub asset_class_data_type: MarketDataClass,
}
