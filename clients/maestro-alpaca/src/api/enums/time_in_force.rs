use serde::{Deserialize, Serialize};

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum TimeInForce {
	#[serde(rename = "day")]
	#[default]
	Day,
	#[serde(rename = "fok")]
	FillOrKill,
	#[serde(rename = "ioc")]
	ImmediateOrCancel,
	#[serde(rename = "gtc")]
	UntilCanceled,
	#[serde(rename = "opg")]
	UntilMarketOpen,
	#[serde(rename = "cls")]
	UntilMarketClose,
}
