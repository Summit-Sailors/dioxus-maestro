use serde::{Deserialize, Serialize};

#[derive(Debug, Default, Clone, PartialEq, Eq, Serialize)]
pub enum Engine {
	#[default]
	#[serde(rename = "google")]
	Google,
}

#[derive(Debug, Default, Clone, PartialEq, Eq, Serialize)]
#[serde(rename_all = "lowercase")]
pub enum Device {
	#[default]
	Desktop,
	Mobile,
	Tablet,
}

#[derive(Debug, Default, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum SearchType {
	#[serde(rename = "")]
	#[default]
	Regular,
	#[serde(rename = "isch")]
	Images,
	#[serde(rename = "lcl")]
	Local,
	#[serde(rename = "vid")]
	Videos,
	#[serde(rename = "nws")]
	News,
	#[serde(rename = "shop")]
	Shopping,
	#[serde(rename = "pts")]
	Patents,
}

#[derive(Debug, Default, Clone, PartialEq, Eq, Serialize)]
#[serde(rename_all = "lowercase")]
pub enum OutputFormat {
	#[default]
	Json,
	Html,
}

#[derive(Debug, Default, Clone, PartialEq, Eq, Serialize)]
pub enum SafeSearch {
	#[serde(rename = "active")]
	Active,
	#[default]
	#[serde(rename = "off")]
	Off,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, strum_macros::EnumIter, strum_macros::Display, strum_macros::EnumString)]
pub enum ETimeFrame {
	#[strum(to_string = "qdr:h")]
	Hour,
	#[strum(to_string = "qdr:d")]
	Day,
	#[strum(to_string = "qdr:w")]
	Week,
	#[strum(to_string = "qdr:m")]
	Month,
	#[strum(to_string = "qdr:y")]
	Year,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct SearchRequest {
	pub engine: Engine,
	pub q: String,
	pub api_key: String,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub location: Option<String>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub device: Option<Device>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub tbs: Option<ETimeFrame>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub tbm: Option<SearchType>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub start: Option<u32>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub num: Option<u32>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub safe: Option<SafeSearch>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub nfpr: Option<bool>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub filter: Option<u8>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub output: Option<OutputFormat>,
}
