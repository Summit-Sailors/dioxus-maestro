use dioxus::prelude::*;
use maestro_forms::fields::{form::InnerComponentProps, select::SelectFormField, text::TextFormInput, textarea::TextArea};
use maestro_ui::button::{Button, ButtonSize, ButtonVariant};
use strum::VariantNames;
use tailwind_fuse::tw_join;

use crate::{
	components::form::{form_field_wrapper::FormFieldWrapper, form_state_debugger::FormStateDebugger},
	models::user::{Role, User},
};

#[component]
pub fn FormContent(props: InnerComponentProps<User>) -> Element {
	let roles = Role::VARIANTS.iter().map(|&s| s.to_string()).collect::<Vec<_>>();

	let input_class = "border border-[color:var(--border-color)] px-3 w-full py-2 flex items-center min-h-11 rounded-md bg-[color:var(--input-bg)] text-[color:var(--text-color)] ring-[color:var(--focus-ring)] ring-offset-transparent focus:outline-none placeholder-[color:var(--muted-text)]";

	rsx! {
		div { class: "flex flex-col rounded-lg gap-6 bg-[color:var(--card-bg)] text-[color:var(--card-text)] lg:px-16 sm:px-6 px-2 py-8",
			FormFieldWrapper {
				label: "Username",
				field: props.form.get_form_field("username".to_string()),
				show_validation: true,
				required: Some(true),
				help_text: Some("Choose a username".into()),
				TextFormInput::<User> {
					name: "username",
					placeholder: "Enter your username",
					class: "{input_class}",
					"aria-required": "true",
					disabled: *props.form.is_submitting.read(),
				}
			}
			FormFieldWrapper {
				label: "Email",
				field: props.form.get_form_field("email".to_string()),
				show_validation: true,
				required: Some(true),
				help_text: Some("Enter a valid email address".into()),
				TextFormInput::<User> {
					name: "email",
					placeholder: "Enter your email address",
					class: "{input_class}",
					"aria-required": "true",
					r#type: "email",
					disabled: *props.form.is_submitting.read(),
				}
			}
			FormFieldWrapper {
				label: "Bio",
				field: props.form.get_form_field("bio".to_string()),
				show_validation: true,
				help_text: Some("Tell us about yourself".into()),
				TextArea::<User> {
					name: "bio",
					placeholder: "Tell us about yourself...",
					rows: 4,
					class: "{input_class}",
					disabled: *props.form.is_submitting.read(),
				}
			}
			FormFieldWrapper {
				label: "Role",
				field: props.form.get_form_field("role".to_string()),
				show_validation: true,
				required: Some(true),
				help_text: Some("Select your role".into()),
				SelectFormField::<User,String> {
					name: "role",
					values: roles.clone(),
					labels: Some(roles),
					class: "{input_class}",
					disabled: *props.form.is_submitting.read(),
				}
			}

			div { class: "flex justify-center mt-2",
				Button {
					r#type: "submit",
					disabled: *props.form.is_submitting.read(),
					size: ButtonSize::Default,
					variant: ButtonVariant::Default,
					class: tw_join!(
							"py-2 rounded-md font-semibold transition-all duration-200 bg-[color:var(--primary-bg)] text-[color:var(--primary-text)] hover:bg-[color:var(--hover-bg)] ring-[color:var(--focus-ring)] ring-offset-transparent",
							(* props.form.is_submitting.read())
							.then_some("bg-[color:var(--border-color)] cursor-not-allowed opacity-70")
					),
					if *props.form.is_submitting.read() {
						div { class: "flex items-center gap-2 justify-center",
							div { class: "animate-spin h-4 w-4 border-2 border-white border-t-transparent rounded-full" }
							"Processing..."
						}
					} else {
						"Submit"
					}
				}
			}
		}
		FormStateDebugger { form: props.form }
	}
}
