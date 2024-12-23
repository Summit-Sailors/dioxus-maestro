use {
	reqwest::Client,
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
	pub uule: Option<String>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub ludocid: Option<String>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub lsig: Option<String>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub kgmid: Option<String>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub si: Option<String>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub ibp: Option<String>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub uds: Option<String>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub google_domain: Option<String>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub gl: Option<String>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub hl: Option<String>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub cr: Option<String>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub lr: Option<String>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub tbs: Option<String>,
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
	pub device: Option<Device>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub no_cache: Option<bool>,
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "async")]
	pub is_async: Option<bool>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub zero_trace: Option<bool>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub output: Option<OutputFormat>,
}

/// Main response struct containing all possible result types
#[derive(Debug, Serialize, Deserialize)]
pub struct SearchResponse {
	pub search_metadata: SearchMetadata,
	pub search_parameters: SearchParameters,
	pub search_information: SearchInformation,
	pub recipes_results: Option<Vec<RecipeResult>>,
	pub shopping_results: Option<Vec<ShoppingResult>>,
	pub local_results: Option<LocalResults>,
	pub knowledge_graph: Option<KnowledgeGraph>,
	pub organic_results: Vec<OrganicResult>,
	pub related_searches: Option<Vec<RelatedSearch>>,
	pub pagination: Pagination,
	pub serpapi_pagination: SerpapiPagination,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SearchMetadata {
	pub id: String,
	pub status: String,
	pub json_endpoint: String,
	pub created_at: String,
	pub processed_at: String,
	pub google_url: String,
	pub raw_html_file: String,
	pub total_time_taken: f64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SearchParameters {
	pub engine: String,
	pub q: String,
	pub location_requested: Option<String>,
	pub location_used: Option<String>,
	pub google_domain: String,
	pub hl: Option<String>,
	pub gl: Option<String>,
	pub device: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SearchInformation {
	pub organic_results_state: String,
	pub query_displayed: String,
	pub total_results: u64,
	pub time_taken_displayed: f64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct OrganicResult {
	pub position: u32,
	pub title: String,
	pub link: String,
	pub displayed_link: String,
	pub snippet: Option<String>,
	pub snippet_highlighted_words: Option<Vec<String>>,
	pub sitelinks: Option<Sitelinks>,
	pub cached_page_link: Option<String>,
	pub related_pages_link: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RecipeResult {
	pub title: String,
	pub link: String,
	pub source: String,
	pub rating: Option<f32>,
	pub reviews: Option<u32>,
	pub ingredients: Vec<String>,
	pub thumbnail: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ShoppingResult {
	pub position: u32,
	pub title: String,
	pub link: String,
	pub source: String,
	pub price: String,
	pub extracted_price: f64,
	pub thumbnail: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct LocalResults {
	pub more_locations_link: Option<String>,
	pub places: Vec<LocalPlace>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct LocalPlace {
	pub position: u32,
	pub title: String,
	pub place_id: String,
	pub lsig: String,
	pub place_id_search: String,
	pub rating: Option<f32>,
	pub reviews: Option<u32>,
	pub type_: Option<String>,
	pub address: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct KnowledgeGraph {
	pub title: String,
	pub type_: Option<String>,
	pub description: Option<String>,
	pub source: Option<KnowledgeGraphSource>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct KnowledgeGraphSource {
	pub name: String,
	pub link: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Sitelinks {
	pub inline: Vec<InlineLink>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct InlineLink {
	pub title: String,
	pub link: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RelatedSearch {
	pub query: String,
	pub link: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Pagination {
	pub current: u32,
	pub next: Option<String>,
	pub other_pages: HashMap<String, String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SerpapiPagination {
	pub current: u32,
	pub next_link: Option<String>,
	pub next: Option<String>,
	pub other_pages: HashMap<String, String>,
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

#[bon::builder]
pub async fn serpapi_request(
	q: String,
	location: Option<String>,
	uule: Option<String>,
	ludocid: Option<String>,
	lsig: Option<String>,
	kgmid: Option<String>,
	si: Option<String>,
	ibp: Option<String>,
	uds: Option<String>,
	google_domain: Option<String>,
	gl: Option<String>,
	hl: Option<String>,
	cr: Option<String>,
	lr: Option<String>,
	tbs: Option<String>,
	safe: Option<SafeSearch>,
	nfpr: Option<bool>,
	filter: Option<u8>,
	start: Option<u32>,
	num: Option<u32>,
	no_cache: Option<bool>,
	async_param: Option<bool>,
	zero_trace: Option<bool>,
	output: Option<OutputFormat>,
	#[builder(default = Engine::Google)] engine: Engine,
	#[builder(default = Device::Desktop)] device: Device,
	#[builder(default = SearchType::Regular)] search_type: SearchType,
	#[builder(default = Client::new().into())] client: ClientWithMiddleware,
) -> Result<SearchResponse, reqwest_middleware::Error> {
	let search_resp = client
		.get(SerpApiRoute::Search.url_path())
		.query(&SearchRequest {
			engine,
			q,
			api_key: std::env::var("SERPAPI_API_KEY").expect("SERPAPI_API_KEY env var is missing"),
			location,
			uule,
			ludocid,
			lsig,
			kgmid,
			si,
			ibp,
			uds,
			google_domain,
			gl,
			hl,
			cr,
			lr,
			tbs,
			tbm: Some(search_type),
			start,
			num,
			safe,
			nfpr,
			filter,
			device: Some(device),
			no_cache,
			is_async: async_param,
			zero_trace,
			output,
		})
		.send()
		.await?
		.json::<SearchResponse>()
		.await?;

	Ok(search_resp)
}
