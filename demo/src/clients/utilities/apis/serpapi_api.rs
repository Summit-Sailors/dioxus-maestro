use {
	dioxus::prelude::*,
	maestro_serpapi::{
		request_type::{ETimeFrame, Engine, SafeSearch, SearchType},
		response_type::{OrganicResult, SearchResponse},
	},
};

// server function to perform the search
#[server]
pub async fn search_google(query: String) -> Result<Vec<OrganicResult>, ServerFnError> {
	use maestro_serpapi::client::serpapi_request;

	// the SerpAPI request with various parameters to showcase the flexibility
	let search_response = serpapi_request()
		.q(query) // search query
		.engine(Engine::Google) // Google search engine
		.search_type(SearchType::Regular) // regular search (not images, news, etc.)
		.tbs(ETimeFrame::Year) // results from the past year
		.safe(SafeSearch::Active) // safe search enabled
		.num(10) // get 10 results
		.call() // execute the request
		.await?;

	Ok(search_response.organic_results)
}

// advanced search with additional parameters
#[server]
pub async fn advanced_search(query: String, search_type: SearchType, time_frame: ETimeFrame, location: String) -> Result<SearchResponse, ServerFnError> {
	use maestro_serpapi::client::serpapi_request;

	let search_response = serpapi_request().q(query).engine(Engine::Google).search_type(search_type).tbs(time_frame).location(location).call().await?;

	Ok(search_response)
}

// how to use the Dioxus server function for fetching content from URLs
#[server]
#[cfg(feature = "dioxus")]
pub async fn fetch_and_extract_content() -> Result<Vec<maestro_serpapi::functions::SerpapiDTO>, ServerFnError> {
	// uses functions.rs implementation to handle fetching and processing URLs
	let results = maestro_serpapi::functions::serpapi_server_request("Rust programming".to_string()).await?;
	Ok(results)
}
