use {
	reqwest_middleware::ClientWithMiddleware,
	serde::{Deserialize, Serialize},
	std::collections::HashMap,
};

#[derive(Clone, Debug, Default, Eq, PartialEq, Serialize)]
pub enum Engine {
	#[default]
	#[serde(rename = "google")]
	Google,
}

#[derive(Clone, Debug, Default, Eq, PartialEq, Serialize)]
#[serde(rename_all = "lowercase")]
pub enum Device {
	#[default]
	Desktop,
	Mobile,
	Tablet,
}

#[derive(Clone, Debug, Default, Eq, PartialEq, Serialize)]
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

#[derive(Clone, Debug, Default, Eq, PartialEq, Serialize)]
#[serde(rename_all = "lowercase")]
pub enum OutputFormat {
	#[default]
	Json,
	Html,
}

#[derive(Clone, Debug, Default, Eq, PartialEq, Serialize)]
pub enum SafeSearch {
	#[serde(rename = "active")]
	Active,
	#[default]
	#[serde(rename = "off")]
	Off,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
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

#[derive(Debug, Serialize, Deserialize)]
pub struct SearchResponse {
	#[serde(flatten)]
	pub metadata: HashMap<String, serde_json::Value>,
	#[serde(default)]
	pub organic_results: Vec<OrganicResult>,
}

#[derive(Debug, Serialize, Deserialize)]
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

#[derive(Debug)]
pub enum SerpApiRoute {
	Search,
	Locations,
	Account,
}

impl SerpApiRoute {
	pub fn url_path(&self) -> String {
		match self {
			SerpApiRoute::Search => "https://serpapi.com/search".to_string(),
			SerpApiRoute::Locations => "https://serpapi.com/locations.json".to_string(),
			SerpApiRoute::Account => "https://serpapi.com/account".to_string(),
		}
	}
}

#[derive(Debug, Clone, Eq, strum_macros::Display, PartialEq, Serialize, Deserialize, strum_macros::EnumIter, strum_macros::EnumString)]
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

#[bon::builder]
pub async fn serpapi_request(
	q: String,
	location: Option<String>,
	tbs: Option<ETimeFrame>,
	safe: Option<SafeSearch>,
	nfpr: Option<bool>,
	filter: Option<u8>,
	start: Option<u32>,
	num: Option<u32>,
	output: Option<OutputFormat>,
	#[builder(default = Engine::Google)] engine: Engine,
	#[builder(default = Device::Desktop)] device: Device,
	#[builder(default = SearchType::Regular)] search_type: SearchType,
	#[builder(default = ClientWithMiddleware::default())] client: ClientWithMiddleware,
) -> Result<SearchResponse, reqwest_middleware::Error> {
	let search_resp = client
		.get(SerpApiRoute::Search.url_path())
		.query(&SearchRequest {
			engine,
			q,
			api_key: std::env::var("SERPAPI_API_KEY").expect("SERPAPI_API_KEY env var is missing"),
			location,
			tbs,
			tbm: Some(search_type),
			start,
			num,
			safe,
			nfpr,
			filter,
			device: Some(device),
			output,
		})
		.send()
		.await?
		.json::<SearchResponse>()
		.await?;

	Ok(search_resp)
}
