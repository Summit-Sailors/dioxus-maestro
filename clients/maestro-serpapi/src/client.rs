use {
	crate::{
		request_type::{Device, ETimeFrame, Engine, OutputFormat, SafeSearch, SearchRequest, SearchType},
		response_type::SearchResponse,
		routes::SerpApiRoute,
	},
	reqwest::Client,
};

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
	#[builder(default = Client::default())] client: Client,
) -> Result<SearchResponse, reqwest::Error> {
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
