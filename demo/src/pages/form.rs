use {
	crate::{
		components::form::form_content::FormContent,
		models::user::{Role, User},
	},
	dioxus::prelude::*,
	dioxus_logger::tracing::info,
	maestro_forms::{
		fields::form::{Form, FormResult},
		use_formik::use_init_form_ctx,
	},
	maestro_toast::{ctx::use_toast, toast_info::ToastInfo},
	tailwind_fuse::tw_join,
};

async fn simulate_submission(delay_ms: u64) -> Result<(), String> {
	async_std::task::sleep(std::time::Duration::from_millis(delay_ms)).await;

	let success_rate = 95;
	let success = js_sys::Math::random() * 100.0 < success_rate as f64;

	if success {
		Ok(())
	} else {
		Err("server error".to_string())
	}
}

#[component]
pub fn FormsDemo() -> Element {
	let mut toast = use_toast();
	let mut is_async = use_signal(|| true);

	let initial_values = User { role: Role::Admin, ..User::default() };
	let form = use_init_form_ctx(initial_values);

	let on_submit = move |(_event, (submitted_user, is_valid), complete_submission): (FormEvent, FormResult<User>, Box<dyn FnOnce()>)| {
		spawn(async move {
			if !is_valid {
				toast.write().popup(ToastInfo::builder().context("Form validation failed. Please check your inputs.".to_owned()).build());
				complete_submission();
				return;
			}

			let delay = if is_async() { 1000 } else { 500 };
			let result = simulate_submission(delay).await;

			match result {
				Ok(_) => {
					toast.write().popup(ToastInfo::builder().context(format!("Form submitted successfully for user: {:?}", submitted_user.username)).build());
				},
				Err(err) => {
					toast.write().popup(ToastInfo::builder().context(format!("Submission failed: {}", err)).build());
				},
			}

			complete_submission();
		});
	};

	let mode_button_base = tw_join!("px-4 py-2 rounded-md font-medium transition-all duration-200");

	let async_class = tw_join!(
		mode_button_base.clone(),
		if is_async() { "bg-blue-600 text-white shadow-md hover:shadow-lg hover:bg-blue-500" } else { "bg-gray-700 text-gray-300 hover:bg-gray-600" }
	);

	let sync_class = tw_join!(
		mode_button_base,
		if !is_async() { "bg-blue-600 text-white shadow-md hover:shadow-lg hover:bg-blue-500" } else { "bg-gray-700 text-gray-300 hover:bg-gray-600" }
	);

	rsx! {
		div { class: "max-w-4xl mx-auto p-6 bg-white text-gray-100 rounded-lg shadow-lg",
			div { class: "mb-8",
				h1 { class: "text-gray-800 text-center text-3xl font-bold mb-2", "Maestro Forms Demo" }
				p { class: "text-gray-500 text-center",
					"A comprehensive demonstration of form handling with simulation mode."
				}

				div { class: "grid justify-center grid-cols-2 mt-4 space-x-2 flex",
					button {
						class: "{async_class}",
						onclick: move |_| is_async.set(true),
						"Async"
					}
					button {
						class: "{sync_class}",
						onclick: move |_| is_async.set(false),
						"Sync"
					}
				}
			}

			Form {
				form,
				onsubmit: on_submit,
				auto_reset: true,
				inner: FormContent,
			}
		}
	}
}
