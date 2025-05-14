use serde::{Deserialize, Serialize};
use strum_macros::{Display, EnumIter, EnumString};

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum TimeFrameOne {
	#[serde(rename = "1Min")]
	Minute,
	#[serde(rename = "1Hour")]
	Hour,
	#[serde(rename = "1Day")]
	#[default]
	Day,
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, EnumIter, Display, EnumString)]
pub enum TimeFrame {
	#[serde(rename = "1Min")]
	#[strum(to_string = "1 Minute")]
	Minute,
	#[serde(rename = "5Min")]
	#[strum(to_string = "5 Minutes")]
	Minute5,
	#[serde(rename = "15Min")]
	#[strum(to_string = "15 Minutes")]
	Minute15,
	#[serde(rename = "30Min")]
	#[strum(to_string = "30 Minutes")]
	Minute30,
	#[serde(rename = "1Hour")]
	#[strum(to_string = "1 Hour")]
	Hour,
	#[serde(rename = "2Hour")]
	#[strum(to_string = "2 Hours")]
	Hour2,
	#[serde(rename = "6Hour")]
	#[strum(to_string = "6 Hours")]
	Hour6,
	#[serde(rename = "12Hour")]
	#[strum(to_string = "12 Hours")]
	Hour12,
	#[serde(rename = "1Day")]
	#[strum(to_string = "1 Day")]
	#[default]
	Day,
	#[serde(rename = "1Month")]
	#[strum(to_string = "1 Month")]
	Month,
	#[serde(rename = "2Month")]
	#[strum(to_string = "2 Months")]
	Month2,
	#[serde(rename = "3Month")]
	#[strum(to_string = "3 Months")]
	Month3,
	#[serde(rename = "4Month")]
	#[strum(to_string = "4 Months")]
	Month4,
	#[serde(rename = "6Month")]
	#[strum(to_string = "6 Months")]
	Month6,
	#[serde(rename = "12Month")]
	#[strum(to_string = "12 Months")]
	Month12,
}
