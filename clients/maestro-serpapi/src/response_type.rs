use {
	serde::{Deserialize, Serialize},
	std::collections::HashMap,
};

#[derive(Debug, Serialize, Deserialize)]
pub struct SearchResponse {
	#[serde(flatten)]
	pub metadata: HashMap<String, serde_json::Value>,
	#[serde(default)]
	pub organic_results: Vec<OrganicResult>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OrganicResult {
	pub position: u32,
	pub title: String,
	pub link: String,
	pub redirect_link: String,
	pub displayed_link: String,
	pub snippet: Option<String>,
	#[serde(flatten)]
	pub additional_fields: HashMap<String, serde_json::Value>,
}
