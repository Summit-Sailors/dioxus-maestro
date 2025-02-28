use {
	crate::{
		components::form::{form_field_wrapper::FormFieldWrapper, form_state_debugger::FormStateDebugger},
		models::user::{Role, User},
	},
	dioxus::prelude::*,
	maestro_forms::fields::{form::InnerComponentProps, select::SelectFormField, text::TextFormInput, textarea::TextArea},
	maestro_ui::button::{Button, ButtonSize, ButtonVariant},
	strum::VariantNames,
	tailwind_fuse::tw_join,
};

#[component]
pub fn FormContent(props: InnerComponentProps<User>) -> Element {
	let roles = Role::VARIANTS.iter().map(|&s| s.to_string()).collect::<Vec<_>>();

	let input_class = tw_join!(
		"w-full flex justify-center p-2 rounded-md border border-gray-500 bg-gray-900 text-gray-50",
		"focus:ring-2 focus:ring-blue-500 focus:outline-none placeholder-gray-500"
	);

	rsx! {
    div { class: "space-y-6 w-4/5 mx-auto sm:w-full",
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
              "py-2 rounded-md text-gray-100 font-semibold transition-all duration-200 bg-blue-500 hover:bg-blue-500",
              if * props.form.is_submitting.read() {
              "bg-gray-500 cursor-not-allowed opacity-70" } else {
              "bg-blue-600 hover:bg-blue-500 hover:shadow-lg transform hover:scale-105" }
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
      FormStateDebugger { form: props.form }
    }
  }
}
