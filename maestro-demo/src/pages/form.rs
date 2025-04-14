use dioxus::prelude::*;
use maestro_forms::{
	fields::form::{Form, FormResult},
	use_formik::use_init_form_ctx,
};
use maestro_toast::{ctx::use_toast, toast_info::ToastInfo};

use crate::{
	components::{form::form_content::FormContent, ui::features::Features},
	models::user::{Role, User},
};

async fn simulate_submission(delay_ms: u64) -> Result<(), String> {
	async_std::task::sleep(std::time::Duration::from_millis(delay_ms)).await;

	let success_rate = 95;
	let success = js_sys::Math::random() * 100.0 < success_rate as f64;

	if success { Ok(()) } else { Err("server error".to_string()) }
}

#[component]
pub fn FormsDemo() -> Element {
	let mut toast = use_toast();

	let initial_values = User { role: Role::Admin, ..User::default() };
	let form = use_init_form_ctx(initial_values); // initialize form context

	let on_submit = move |(_event, (submitted_user, is_valid), complete_submission): (FormEvent, FormResult<User>, Box<dyn FnOnce()>)| {
		spawn(async move {
			if !is_valid {
				toast.write().popup(ToastInfo::builder().context("Form validation failed. Please check your inputs.".to_owned()).build());
				complete_submission();
				return;
			}

			let delay = 1000;
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

	rsx! {
		div {
			id: "maestro-form",
			class: "flex flex-col lg:gap-16 sm:gap-8 gap-6",
			div { class: "flex flex-col gap-3",
				h1 { class: "text-slate-100 text-center text-2xl sm:text-3xl lg:text-4xl 2xl:text-5xl font-semibold",
					"Maestro Form"
				}
				p { class: "text-slate-300 text-center text-base lg:text-xl 2xl:text-2xl",
					"A powerful, type-safe form management solution for Dioxus applications that brings the best of Formik's paradigms to Rust."
				}
			}

			div { id: "maestro-form-features", class: "flex space-x-2",
				Features {
					title: "Features".to_string(),
					features: vec![
							"Type-safe Form Handling: fully type-safe form state management with Rust's powerful type system"
									.to_string(),
							"Performance Optimized: built-in debounced inputs prevent unnecessary re-renders"
									.to_string(),
							"Validation Integration: seamless integration with the validator crate for declarative validation"
									.to_string(),
							"Smart Field Tracking: automatic touched state management and error tracking"
									.to_string(),
							"UI Agnostic: flexible design that separates form logic from presentation"
									.to_string(),
					],
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
