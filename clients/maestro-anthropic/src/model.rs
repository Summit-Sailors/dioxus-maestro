use serde::{Deserialize, Serialize};

#[derive(Clone, Copy, Debug, Default, Deserialize, strum_macros::EnumIter, Eq, Ord, PartialEq, PartialOrd, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum Model {
	#[serde(rename = "claude-3-7-sonnet-20250219")]
	Sonnet37,
	#[serde(rename = "claude-3-5-sonnet-latest")]
	Sonnet35,
	/// 2024-06-20
	#[serde(rename = "claude-3-5-sonnet-20240620")]
	Sonnet35_20240620,
	/// 2024-10-22
	#[serde(rename = "claude-3-5-sonnet-20241022")]
	Sonnet35_20241022,
	#[serde(rename = "claude-3-opus-latest")]
	Opus30,
	#[serde(rename = "claude-3-opus-20240229")]
	Opus30_20240229,
	#[serde(rename = "claude-3-sonnet-20240229")]
	Sonnet30,
	#[serde(rename = "claude-3-5-haiku-latest")]
	Haiku35,
	#[serde(rename = "claude-3-5-haiku-20241022")]
	Haiku35_20241022,
	#[default]
	#[serde(rename = "claude-3-haiku-20240307", alias = "claude-3-haiku-latest")]
	Haiku30,
}
