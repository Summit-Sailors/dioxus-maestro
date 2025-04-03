use dioxus::prelude::*;

// for adding a new job
#[component]
pub fn JobForm() -> Element {
	let mut to = use_signal(|| String::new());
	let mut subject = use_signal(|| String::new());
	let mut body = use_signal(|| String::new());
	let status = use_signal(|| String::new());

	let add_job = move |_| {
		to_owned![to, subject, body, status];
		async move {
			status.set("Adding job...".to_string());

			match apalis_api::add_email_job(to(), subject(), body()).await {
				Ok(_) => {
					status.set("Job added successfully!".to_string());
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
		div {
			h2 { "Add Email Job" }
			div {
				label { "To: " }
				input { value: "{to}", oninput: move |e| to.set(e.value()) }
			}
			div {
				label { "Subject: " }
				input {
					value: "{subject}",
					oninput: move |e| subject.set(e.value()),
				}
			}
			div {
				label { "Body: " }
				textarea { value: "{body}", oninput: move |e| body.set(e.value()) }
			}
			button { onclick: add_job, "Add Job" }
			div {
				p { "{status}" }
			}
		}
	}
}

// to display pending jobs
#[component]
pub fn JobsList() -> Element {
	let mut jobs_state = use_signal(|| String::new());
	let mut loading = use_signal(|| false);

	let fetch_jobs = move |_| async move {
		loading.set(true);
		match apalis_api::list_pending_jobs().await {
			Ok(job_list) => {
				jobs_state.set(job_list);
			},
			Err(e) => {
				println!("Error fetching jobs status: {}", e);
			},
		}
		loading.set(false);
	};

	rsx! {
		div {
			h2 { "Pending Email Jobs" }
			button { onclick: fetch_jobs, "Refresh Job List" }

			if loading() {
				p { "Loading jobs state..." }
			} else {
				p { "{jobs_state()}" }
			}
		}
	}
}

#[component]
pub fn ApalisDemo() -> Element {
	rsx! {
		div {
			h1 { "Maestro-Apalis Demo" }
			p { "A simple demo of using apalis job scheduling in Dioxus" }

			JobForm {}
			hr {}
			JobsList {}
		}
	}
}
