use dioxus::prelude::*;

use crate::components::ui::features::Features;

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
		div { class: "w-full mx-auto p-4 bg-[color:var(--bg-color)] text-[color:var(--text-color)]",
			div { class: "flex flex-col gap-3",
				h1 { class: "text-[color:var(--text-color)] text-center text-3xl font-bold mb-2",
					"Maestro SerpAPI"
				}
				p { class: "text-[color:var(--muted-text)] text-center",
					"A serpapi utility designed to make your experience pleasant when integrating SerpAPI into your Dioxus app"
				}
			}

			div {
				id: "maestro-serpaip-features",
				class: "flex flex-wrap gap-2 mt-4 mb-4",
				Features {
					title: "Features".to_string(),
					features: vec![
							"Ready to use Dioxus server function".to_string(),
							"Comprehensive DTOs".to_string(),
							"Simple integration with Dioxus".to_string(),
					],
				}
			}
			p { class: "mb-4 text-[color:var(--text-color)]",
				"Search for any topic using the power of SerpAPI!"
			}

			form {
				onsubmit: move |_| on_search(state().query),
				class: "flex mb-6",
				input {
					r#type: "text",
					placeholder: "Enter search query...",
					value: "{state().query}",
					oninput: on_query_change,
					class: "flex-grow p-2 border border-[color:var(--border-color)] bg-[color:var(--input-bg)] text-[color:var(--text-color)] rounded-l focus:outline-none focus:ring-2 focus:ring-[color:var(--primary)]",
				}
				button {
					r#type: "submit",
					disabled: state().loading,
					class: "bg-[color:var(--primary)] text-white p-2 rounded-r hover:bg-[color:var(--primary-hover)] disabled:bg-[color:var(--primary-disabled)]",
					if state().loading {
						"Searching..."
					} else {
						"Search"
					}
				}
			}

			if state().loading {
				div { class: "text-center p-4 text-[color:var(--text-subtle)]", "Loading results..." }
			} else if let Some(error) = &state().error {
				div { class: "bg-red-100 border border-red-400 text-red-700 px-4 py-3 rounded",
					"{error}"
				}
			} else if !state().results.is_empty() {
				div {
					h2 { class: "text-xl font-semibold mb-4 text-[color:var(--text-heading)]",
						"Search Results"
					}
					div { class: "space-y-4",
						p { class: "text-[color:var(--text-subtle)]",
							"Found: {state().results.len()} results"
						}
						for (index , result) in state.read().results.iter().enumerate() {
							div {
								key: "{index}",
								class: "border border-[color:var(--border-color)] p-4 rounded bg-[color:var(--result-bg)]",
								h3 { class: "text-lg font-medium text-[color:var(--link-color)]",
									a {
										href: "{result.link}",
										target: "_blank",
										"{result.title}"
									}
								}
								p { class: "text-sm text-[color:var(--text-muted)]",
									"{result.displayed_link}"
								}
								if let Some(snippet) = &result.snippet {
									p { class: "mt-2 text-[color:var(--text-color)]",
										"{snippet}"
									}
								}
								p { class: "text-sm text-[color:var(--text-muted)] mt-2",
									"Position: {result.position}"
								}
							}
						}
					}
				}
			} else {
				div {
					p { class: "text-center text-[color:var(--accent-orange)]",
						"Try searching anything!"
					}
				}
			}
		}
	}
}
