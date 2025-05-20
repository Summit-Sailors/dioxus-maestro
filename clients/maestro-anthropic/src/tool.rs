use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
#[serde(rename_all = "snake_case", tag = "type")]
pub enum Choice {
	Auto,
	Any,
	Tool { name: String },
}

#[derive(bon::Builder, Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct Tool {
	pub name: String,
	pub description: String,
	pub input_schema: serde_json::Value,
}
