use std::fmt::Error;

use dioxus::prelude::*;
use instant::Instant;
use maestro_query::prelude::*;
use maestro_ui::button::Button;
use tailwind_fuse::tw_join;

#[derive(Debug, Clone, PartialEq)]
struct CachedData {
	value: String,
	timestamp: Instant,
}

#[component]
pub fn CacheDemo() -> Element {
	let query_client: UseQueryClient<CachedData, Error, String> = use_init_query_client();
	let mut stale_time = use_signal(|| 5000u64);
	let mut auto_refresh = use_signal(|| false);

	// main cached query with timestamp
	let cached_query = use_get_query([String::from("cached-data")], |_| async move {
		async_std::task::sleep(std::time::Duration::from_secs(1)).await;
		QueryResult::Ok::<CachedData, Error>(CachedData {
			value: format!("Data fetched at: {}", chrono::Local::now().format("%H:%M:%S")),
			timestamp: Instant::now(),
		})
	});

	// auto-refresh logic
	use_effect(move || {
		if auto_refresh() {
			spawn(async move {
				loop {
					async_std::task::sleep(std::time::Duration::from_millis(stale_time())).await;
					if !auto_refresh() {
						break;
					}
					query_client.invalidate_query(String::from("cached-data"));
				}
			});
		}
	});

	let force_refetch = move |_| {
		query_client.invalidate_query(String::from("cached-data"));
	};

	rsx! {
		div { class: "flex flex-col items-center gap-4 p-4 bg-[color:var(--bg-color)] rounded-lg shadow-lg",
			div { class: "space-y-4",
				h2 { class: "text-2xl font-bold text-[color:var(--text-color)] text-center",
					"Cache Management"
				}
				div { class: "mt-4 space-y-2 items-center",
					label { class: "block text-sm font-medium text-center text-[color:var(--text-color)]",
						"Stale Time: {stale_time}ms"
					}
					input {
						class: "w-full",
						r#type: "range",
						min: "500",
						max: "10000",
						step: "500",
						value: "{stale_time}",
						onchange: move |e| stale_time.set(e.value().parse().unwrap_or(2000)),
					}
				}
				div { class: "items-center space-x-2",
					input {
						r#type: "checkbox",
						checked: "{auto_refresh}",
						onchange: move |e| auto_refresh.set(e.value().parse().unwrap_or(false)),
					}
					label { class: "text-sm font-medium text-[color:var(--text-color)]",
						"Auto Refresh"
					}
				}
			}
			div { class: "grid justify-center grid-cols-1 md:grid-cols-2 p-4 bg-[color:var(--bg-color)] rounded-lg",
				div { class: "text-center p-2 rounded",
					p { class: "font-medium text-[color:var(--text-color)] text-center",
						"Cache Status:"
					}
					p {
						class: tw_join!(
								"p-4 rounded-lg border border-[color:var(--border)] transition-colors cursor-pointer",
								"text-[color:var(--accent)]"
						),
						{if cached_query.result().is_fresh() { "Fresh" } else { "Stale" }}
					}
				}
				div { class: "text-center p-2 rounded",
					p { class: "font-medium text-[color:var(--text-color)]", "Query Status:" }
					p {
						class: tw_join!(
								"p-4 rounded-lg border border-[color:var(--border)] transition-colors cursor-pointer",
								if cached_query.result().is_fresh() { "text-[color:var(--primary)]" } else {
								"text-[color:var(--accent)]" }
						),
						if cached_query.result().is_loading() {
							"Loading..."
						} else {
							"Ready"
						}
					}
				}
			}
			// cached data display
			div { class: "flex items-center p-4 bg-[color:var(--bg-color)] rounded border border-[color:var(--border)]",
				match cached_query.result().value() {
						QueryResult::Loading(_) => rsx! {
							div { class: "text-center text-[color:var(--primary)]", "Fetching fresh data..." }
						},
						QueryResult::Ok(data) => rsx! {
							p { class: "font-medium text-center text-[color:var(--text-color)]", "{data.value}" }
							p { class: "text-sm text-[color:var(--text-color)] text-center",
								{format!("Cache age: {}ms", data.timestamp.elapsed().as_millis())}
							}
						},
						QueryResult::Err(e) => rsx! {
							div { class: "text-center text-[color:var(--destructive)]", "Error: {e:?}" }
						},
				}
			}
			// actions
			div { class: "flex items-center space-x-4",
				Button {
					class: "bg-[color:var(--primary)] hover:bg-[color:var(--hover-bg)] text-[color:var(--primary-foreground)] px-4 py-2 rounded transition",
					onclick: force_refetch,
					disabled: cached_query.result().is_loading(),
					if cached_query.result().is_loading() {
						"Fetching..."
					} else {
						"Force Refresh"
					}
				}
			}
		}
	}
}
