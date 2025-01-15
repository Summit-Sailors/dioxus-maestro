use serde::{Deserialize, Serialize};

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum Type {
	#[serde(rename = "market")]
	#[default]
	Market,
	#[serde(rename = "limit")]
	Limit,
	#[serde(rename = "stop")]
	Stop,
	#[serde(rename = "stop_limit")]
	StopLimit,
	#[serde(rename = "trailing_stop")]
	TrailingStop,
}
