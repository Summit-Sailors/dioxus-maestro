use {
	dioxus::prelude::*,
	maestro_serpapi::{
		request_type::{ETimeFrame, Engine, SafeSearch, SearchType},
		response_type::{OrganicResult, SearchResponse},
	},
};

#[derive(Clone, Debug)]
struct SearchState {
	query: String,
	results: Vec<T>,
	loading: bool,
	error: Option<String>,
}

#[component]
fn SerpApiDemo() -> Element {
	let state = use_signal(|| SearchState { query: String::new(), results: Vec::new(), loading: false, error: None });

	let on_search = move |query: String| {
		to_owned![state];
		async move {
			state.set(SearchState { query, results: Vec::new(), loading: true, error: None });

			match search_google(query).await {
				Ok(results) => {
					state.set(SearchState { query, results, loading: false, error: None });
				},
				Err(e) => {
					state.set(SearchState { query, results: Vec::new(), loading: false, error: Some(format!("Error: {}", e)) });
				},
			}
		}
	};

	let on_query_change = move |evt: Event<FormData>| {
		to_owned![state];
		state.set(SearchState { query: evt.value().clone(), results: state().results, loading: state().loading, error: state().error.clone() });
	};

	let on_submit = move |evt: Event<FormEvent>| {
		evt.prevent_default();
		to_owned![state, on_search];
		spawn(on_search(state().query.clone()));
	};

	rsx! {
		div { class: "w-4/5 mx-auto p-4",
			h1 { class: "text-2xl font-bold mb-4", "Maestro-SerpAPI Demo" }
			p { class: "mb-4", "Search for any topic using the power of SerpAPI!" }

			form { onsubmit: on_submit, class: "flex mb-6",
				input {
					r#type: "text",
					placeholder: "Enter search query...",
					value: "{state().query}",
					oninput: on_query_change,
					class: "flex-grow p-2 border rounded-l",
				}
				button {
					r#type: "submit",
					disabled: state().loading,
					class: "bg-blue-500 text-white p-2 rounded-r hover:bg-blue-700 disabled:bg-blue-300",
					if state().loading {
						"Searching..."
					} else {
						"Search"
					}
				}
			}

			if state().loading {
				div { class: "text-center p-4", "Loading results..." }
			} else if let Some(error) = &state().error {
				div { class: "bg-red-100 border border-red-400 text-red-700 px-4 py-3 rounded",
					"{error}"
				}
			} else if !state().results.is_empty() {
				div {
					h2 { class: "text-xl font-semibold mb-4", "Search Results" }
					div { class: "space-y-4",
						{
								state()
										.results
										.iter()
										.enumerate()
										.map(|(index, result)| {
												rsx! {
													div { key: "{index}", class: "border p-4 rounded",
														h3 { class: "text-lg font-medium text-blue-600",
															a { href: "{result.link}", target: "_blank", "{result.title}" }
														}
														p { class: "text-sm text-gray-600", "{result.displayed_link}" }
														if let Some(snippet) = &result.snippet {
															p { class: "mt-2", "{snippet}" }
														}
														p { class: "text-sm text-gray-500 mt-2", "Position: {result.position}" }
													}
												}
										})
						}
					}
				}
			}
		}
	}
}

// server function to perform the search
#[server]
async fn search_google(query: String) -> Result<Vec<OrganicResult>, ServerFnError> {
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
		.await
		.map_err(|e| ServerFnError::ServerError(format!("SerpAPI request failed: {}", e)))?;

	Ok(search_response.organic_results)
}

// advanced search with additional parameters
#[server]
async fn advanced_search(
	query: String,
	search_type: SearchType,
	time_frame: Option<ETimeFrame>,
	location: Option<String>,
) -> Result<SearchResponse, ServerFnError> {
	use maestro_serpapi::client::serpapi_request;

	let search_response = serpapi_request()
		.q(query)
		.engine(Engine::Google)
		.search_type(search_type)
		.tbs(time_frame?)
		.location(location?)
		.call()
		.await
		.map_err(|e| ServerFnError::ServerError(format!("Advanced search failed: {}", e)))?;

	Ok(search_response)
}

// how to use the Dioxus server function for fetching content from URLs
#[server]
async fn fetch_and_extract_content() -> Result<Vec<api::prompt_preset::models::SerpapiDTO>, ServerFnError> {
	// uses functions.rs implementation to handle fetching and processing URLs
	let results = maestro_serpapi::functions::serpapi_server_request("Rust programming".to_string()).await?;
	Ok(results)
}
