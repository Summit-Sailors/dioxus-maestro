use dioxus::prelude::*;

use crate::components::ui::features::Features;

#[component]
pub fn JobForm() -> Element {
	let mut to = use_signal(String::new);
	let mut subject = use_signal(String::new);
	let mut body = use_signal(String::new);
	let status = use_signal(String::new);

	let add_job = move |_| {
		to_owned![to, subject, body, status];
		async move {
			status.set("Adding job...".to_string());

			match crate::clients::utilities::apis::apalis_api::add_email_job(to(), subject(), body()).await {
				Ok(result) => {
					status.set(result);
					to.set(String::new());
					subject.set(String::new());
					body.set(String::new());
				},
				Err(e) => {
					status.set(format!("Error adding job: {}", e));
				},
			}
		}
	};

	rsx! {
		div { class: "bg-[color:var(--card-bg)] p-6 rounded-[var(--radius-lg)] shadow-md w-full max-w-md",
			h2 { class: "text-xl font-semibold text-[color:var(--card-text)] mb-4 text-center",
				"Add Email Job"
			}

			div { class: "mb-4",
				label { class: "block text-sm text-[color:var(--muted-text)] mb-1", "To:" }
				input {
					class: "w-full p-2 rounded-[var(--radius-md)] bg-[color:var(--input-bg)] text-[color:var(--text-color)] border border-[color:var(--border-color)] focus:outline-none focus:ring focus:border-[color:var(--focus-ring)]",
					value: "{to}",
					oninput: move |e| to.set(e.value()),
				}
			}

			div { class: "mb-4",
				label { class: "block text-sm text-[color:var(--muted-text)] mb-1", "Subject:" }
				input {
					class: "w-full p-2 rounded-[var(--radius-md)] bg-[color:var(--input-bg)] text-[color:var(--text-color)] border border-[color:var(--border-color)] focus:outline-none focus:ring focus:border-[color:var(--focus-ring)]",
					value: "{subject}",
					oninput: move |e| subject.set(e.value()),
				}
			}

			div { class: "mb-4",
				label { class: "block text-sm text-[color:var(--muted-text)] mb-1", "Body:" }
				textarea {
					class: "w-full p-2 rounded-[var(--radius-md)] bg-[color:var(--input-bg)] text-[color:var(--text-color)] border border-[color:var(--border-color)] h-32 resize-none focus:outline-none focus:ring focus:border-[color:var(--focus-ring)]",
					value: "{body}",
					oninput: move |e| body.set(e.value()),
				}
			}

			button {
				class: "w-full bg-[color:var(--primary-bg)] hover:bg-[color:var(--hover-bg)] text-[color:var(--primary-text)] font-semibold py-2 px-4 rounded-[var(--radius-md)] transition",
				onclick: add_job,
				"Add Job"
			}

			if !status().is_empty() {
				div { class: "mt-4 text-sm text-[color:var(--muted-text)] text-center",
					p { "{status}" }
				}
			}
		}
	}
}

#[component]
pub fn JobsList() -> Element {
	let mut jobs_states = use_signal(Vec::<String>::new);
	let mut loading = use_signal(|| false);

	let fetch_jobs = move |_| async move {
		loading.set(true);
		match crate::clients::utilities::apis::apalis_api::list_completed_jobs().await {
			Ok(job_list) => {
				jobs_states.set(job_list);
			},
			Err(e) => {
				println!("Error fetching jobs status: {}", e);
			},
		}
		loading.set(false);
	};

	rsx! {
		div { class: "bg-[color:var(--card-bg)] p-6 rounded-[var(--radius-lg)] shadow-md w-full max-w-md mt-4 mb-4",
			h2 { class: "text-xl font-semibold text-[color:var(--card-text)] mb-4 text-center",
				"Completed Email Jobs"
			}

			button {
				class: "w-full bg-[color:var(--primary-bg)] hover:bg-[color:var(--hover-bg)] text-[color:var(--primary-text)] font-semibold py-2 px-4 rounded-[var(--radius-md)] transition mb-4",
				onclick: fetch_jobs,
				"Refresh Job List"
			}

			if loading() {
				p { class: "text-[color:var(--muted-text)] text-center", "Loading jobs state..." }
			} else {
				pre { class: "bg-[color:var(--bg-color)] text-[color:var(--text-color)] p-4 rounded-md whitespace-pre-wrap text-sm",
					{
							{
									jobs_states
											.iter()
											.enumerate()
											.map(|(index, job_state)| {
													rsx! {
														div {
															key: "{index}",
															class: "border border-[color:var(--border-color)] rounded-[var(--radius-xl)] p-4 bg-[color:var(--card-bg)] text-[color:var(--card-text)] shadow transition hover:shadow-lg hover:border-[color:var(--focus-ring)]",
															p { class: "text-xl font-bold text-[color:var(--card-text)]", "{job_state}" }
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

#[component]
pub fn ApalisDemo() -> Element {
	rsx! {
		div { class: "max-h-screen bg-[color:var(--bg-color)] text-[color:var(--text-color)] flex items-center justify-center p-4",
			div { class: "flex flex-col items-center gap-6 w-full",
				div { class: "flex flex-col gap-3",
					h1 { class: "text-[color:var(--text-color)] text-center text-3xl font-bold mb-2",
						"Maestro Apalis"
					}
					p { class: "text-[color:var(--muted-text)] text-center",
						"A Apalis utility designed to make integrating Apalis into your Dioxus app easier"
					}
				}

				div {
					id: "maestro-apalis-features",
					class: "flex space-x-2 mt-4 mb-4",
					Features {
						title: "Features".to_string(),
						features: vec![
								"Synchronous and Asynchronous job storage connection pool creation support"
										.to_string(),
								"Storage retrieval from server context".to_string(),
								"Simple integration with Dioxus".to_string(),
						],
					}
				}
				p { class: "text-[color:var(--muted-text)] text-center max-w-md",
					"A simple demo of using apalis job scheduling in Dioxus"
				}

				JobForm {}
				JobsList {}
			}
		}
	}
}
