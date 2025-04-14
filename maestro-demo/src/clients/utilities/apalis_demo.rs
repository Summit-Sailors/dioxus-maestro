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
		div { class: "bg-gray-800 p-6 rounded-lg shadow-md w-full max-w-md",
			h2 { class: "text-xl font-semibold text-white mb-4 text-center", "Add Email Job" }

			div { class: "mb-4",
				label { class: "block text-sm text-gray-300 mb-1", "To:" }
				input {
					class: "w-full p-2 rounded-md bg-gray-700 text-white border border-gray-600 focus:outline-none focus:ring focus:border-blue-500",
					value: "{to}",
					oninput: move |e| to.set(e.value()),
				}
			}

			div { class: "mb-4",
				label { class: "block text-sm text-gray-300 mb-1", "Subject:" }
				input {
					class: "w-full p-2 rounded-md bg-gray-700 text-white border border-gray-600 focus:outline-none focus:ring focus:border-blue-500",
					value: "{subject}",
					oninput: move |e| subject.set(e.value()),
				}
			}

			div { class: "mb-4",
				label { class: "block text-sm text-gray-300 mb-1", "Body:" }
				textarea {
					class: "w-full p-2 rounded-md bg-gray-700 text-white border border-gray-600 h-32 resize-none focus:outline-none focus:ring focus:border-blue-500",
					value: "{body}",
					oninput: move |e| body.set(e.value()),
				}
			}

			button {
				class: "w-full bg-blue-600 hover:bg-blue-700 text-white font-semibold py-2 px-4 rounded-md transition",
				onclick: add_job,
				"Add Job"
			}

			if !status().is_empty() {
				div { class: "mt-4 text-sm text-gray-300 text-center",
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
		div { class: "bg-gray-800 p-6 rounded-lg shadow-md w-full max-w-md mt-4 mb-4",
			h2 { class: "text-xl font-semibold text-white mb-4 text-center", "Completed Email Jobs" }

			button {
				class: "w-full bg-blue-600 hover:bg-blue-700 text-white font-semibold py-2 px-4 rounded-md transition mb-4",
				onclick: fetch_jobs,
				"Refresh Job List"
			}

			if loading() {
				p { class: "text-gray-400 text-center", "Loading jobs state..." }
			} else {
				pre { class: "bg-gray-900 text-gray-100 p-4 rounded-md whitespace-pre-wrap text-sm",
					{
							{
									jobs_states
											.iter()
											.enumerate()
											.map(|(index, job_state)| {
													rsx! {
														div {
															key: "{index}",
															class: "border border-slate-700 rounded-xl p-4 bg-gray-800 text-slate-100 shadow transition hover:shadow-lg hover:border-slate-500",
															p { class: "text-xl font-bold text-white", "{job_state}" }
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
		div { class: "max-h-screen bg-gray-900 text-white flex items-center justify-center p-4",
			div { class: "flex flex-col items-center gap-6 w-full",
				div { class: "flex flex-col gap-3",
					h1 { class: "text-slate-100 text-center text-2xl sm:text-3xl lg:text-4xl 2xl:text-5xl font-semibold",
						"Maestro Apalis"
					}
					p { class: "text-slate-300 text-center text-base lg:text-xl 2xl:text-2xl",
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
				p { class: "text-gray-400 text-center max-w-md",
					"A simple demo of using apalis job scheduling in Dioxus"
				}

				JobForm {}
				JobsList {}
			}
		}
	}
}
