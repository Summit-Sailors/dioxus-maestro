use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case", tag = "type")]
pub enum Choice {
	Auto,
	Any,
	Tool { name: String },
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, bon::Builder)]
pub struct Tool {
	pub name: String,
	pub description: String,
	pub input_schema: serde_json::Value,
}
