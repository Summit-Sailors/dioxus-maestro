use {crate::components::ui::features::Features, dioxus::prelude::*};

#[derive(Clone, Debug)]
struct SearchState {
	query: String,
	results: Vec<maestro_serpapi::response_type::OrganicResult>,
	loading: bool,
	error: Option<String>,
}

#[component]
pub fn SerpApiDemo() -> Element {
	let mut state =
		use_signal(|| SearchState { query: String::new(), results: Vec::<maestro_serpapi::response_type::OrganicResult>::new(), loading: false, error: None });

	let mut on_search = move |query: String| {
		state.set(SearchState { query: query.clone(), results: Vec::new(), loading: true, error: None });

		async move {
			match crate::clients::utilities::apis::serpapi_api::search_google(state().query).await {
				Ok(results) => {
					state.set(SearchState { query, results, loading: false, error: None });
				},
				Err(e) => {
					state.set(SearchState {
						query,
						results: Vec::new(),
						loading: false,
						error: Some(format!("Error: An error occurred while running the search query: {}", e)),
					});
				},
			}
		}
	};

	let on_query_change = move |evt: Event<FormData>| {
		let current_state = state();
		state.set(SearchState { query: evt.value().clone(), results: current_state.results, loading: current_state.loading, error: current_state.error.clone() });
	};

	rsx! {
		div { class: "w-full mx-auto p-4",
			div { class: "flex flex-col gap-3",
				h1 { class: "text-slate-100 text-center text-2xl sm:text-3xl lg:text-4xl 2xl:text-5xl font-semibold",
					"Maestro SerpAPI"
				}
				p { class: "text-slate-300 text-center text-base lg:text-xl 2xl:text-2xl",
					"A serpapi utility designed to make your experience pleasant when integrating SerpAPI into your Dioxus app"
				}
			}

			div {
				id: "maestro-serpaip-features",
				class: "flex space-x-2 mt-4 mb-4",
				Features {
					title: "Features".to_string(),
					features: vec![
							"Ready to use Dioxus server function".to_string(),
							"Comprehensive DTOs".to_string(),
							"Simple integration with Dioxus".to_string(),
					],
				}
			}
			p { class: "mb-4", "Search for any topic using the power of SerpAPI!" }

			form {
				onsubmit: move |_| on_search(state().query),
				class: "flex mb-6",
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
						p { "Found: {state().results.len()} results" }
						for (index , result) in state.read().results.iter().enumerate() {
							div { key: "{index}", class: "border p-4 rounded",
								h3 { class: "text-lg font-medium text-blue-600",
									a {
										href: "{result.link}",
										target: "_blank",
										"{result.title}"
									}
								}
								p { class: "text-sm text-gray-600", "{result.displayed_link}" }
								if let Some(snippet) = &result.snippet {
									p { class: "mt-2", "{snippet}" }
								}
								p { class: "text-sm text-gray-500 mt-2",
									"Position: {result.position}"
								}
							}
						}
					}
				}
			} else {
				div {
					p { class: "text-center text-orange-400", "Try searching anything!" }
				}
			}
		}
	}
}
