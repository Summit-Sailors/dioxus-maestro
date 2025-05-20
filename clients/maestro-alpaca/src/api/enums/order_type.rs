use serde::{Deserialize, Serialize};

#[derive(Clone, Copy, Debug, Default, Deserialize, Eq, PartialEq, Serialize)]
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
