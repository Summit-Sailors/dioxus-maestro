use serde::{Deserialize, Serialize};

#[derive(Clone, Copy, Debug, Default, Deserialize, Eq, PartialEq, Serialize)]
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
