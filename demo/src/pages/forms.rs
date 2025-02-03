use {
	crate::{
		components::forms::{form_field_wrapper::FormFieldWrapper, form_state_debugger::FormStateDebugger},
		models::user::{Role, User},
	}, 
  dioxus::prelude::*, maestro_forms::fields::{
		form::{Form, InnerComponentProps},
		select::SelectFormField,
		text::TextFormInput,
		textarea::TextArea,
	}, 
  maestro_toast::{
    ctx::use_toast, toast_info::ToastInfo
  }, maestro_ui::button::{Button, ButtonSize, ButtonType, ButtonVariant}, strum::VariantNames, tailwind_fuse::tw_join
};

#[component]
pub fn FormContent(props: InnerComponentProps<User>) -> Element {
  let loading = use_signal(|| false);
  
  let role_values = Role::VARIANTS.iter().map(|&s| s.to_string()).collect::<Vec<_>>();
  let role_labels = Role::VARIANTS.iter().map(|&s| s.to_string()).collect::<Vec<_>>();

  let input_class = tw_join!(
    "w-full p-2 rounded-md border border-gray-300 shadow-sm",
    "focus:ring-2 focus:ring-blue-400 focus:outline-none placeholder-opacity-50"
  );
  
  rsx! {
    div { 
      class: "space-y-6",
      FormFieldWrapper {
        label: "Username",
        field: props.form.get_form_field("username".to_string()),
        show_validation: true,
        TextFormInput::<User> {
          name: "username",
          placeholder: "Enter your username",
          class: "{input_class}",
        }
      }

      FormFieldWrapper {
        label: "Email",
        field: props.form.get_form_field("email".to_string()),
        show_validation: true,
        TextFormInput::<User> {
          name: "email",
          placeholder: "Enter your email address",
          class: "{input_class}",
        }
      }

      FormFieldWrapper {
        label: "Bio",
        field: props.form.get_form_field("bio".to_string()),
        show_validation: true,
        TextArea::<User> {
          name: "bio",
          placeholder: "Tell us about yourself...",
          rows: 4,
          class: "{input_class}",
        }
      }

      FormFieldWrapper {
        label: "Role",
        field: props.form.get_form_field("role".to_string()),
        show_validation: true,
        SelectFormField::<User, String> {
          name: "role",
          values: role_values,
          labels: Some(role_labels),
          class: "w-full p-2 rounded-md border border-gray-300 focus:ring focus:ring-blue-300 focus:outline-none",
        }
      }

      Button {
				button_type: ButtonType::Submit,
				disabled: loading(),
				prevent_default: false,
				size: ButtonSize::Default,
				variant: ButtonVariant::Default,
				class: tw_join!(
          "mt-4 py-2 rounded-md text-white font-semibold transition-all duration-200",
          if loading() {
              "bg-gray-400 cursor-not-allowed opacity-70"
          } else {
              "bg-blue-500 hover:bg-blue-600 transform hover:scale-105"
          }
      ),
				if loading() {
					"Loading..."
				} else {
					"Submit"
				}
			}

      FormStateDebugger {
        form: props.form
      }
    }
  }
}

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
  let mut loading = use_signal(|| false);

  let on_submit = move |(_event, (submitted_user, is_valid)): (FormEvent, (User, bool))| async move {
    if is_valid {
      loading.set(true);
      let mut toast_clone = toast.clone();
      let user_clone = submitted_user.clone();
      let is_async_mode = is_async();

      spawn(async move {
        let delay = if is_async_mode { 1000 } else { 500 };
        let result = simulate_submission(delay).await;

        match result {
          Ok(_) => {
            toast_clone.write().popup(
              ToastInfo::builder()
                .context(format!(
                  "Form submitted successfully for user: {}",
                  user_clone.username
                ))
                .build()
            );
          }
          Err(err) => {
            toast_clone.write().popup(
              ToastInfo::builder()
                .context(format!("Submission failed: {}", err))
                .build(),
            );
          }
        }
        loading.set(false);
      });
    } else {
      toast.write().popup(
        ToastInfo::builder()
          .context("Form validation failed. Please check your inputs.".to_owned())
          .build(),
      );
    }
  };

  let mode_button_base = tw_join!(
    "px-4 py-2 rounded-md font-medium transition-all duration-200"
  );

  let async_class = tw_join!(
    mode_button_base.clone(),
    if is_async() { "bg-blue-500 text-white" } else { "bg-gray-200" }
  );

  let sync_class = tw_join!(
    mode_button_base,
    if !is_async() { "bg-blue-500 text-white" } else { "bg-gray-200" }
  );

  rsx! {
    div {
      class: "max-w-4xl mx-auto p-6",
      div {
        class: "mb-8",
        h1 {
          class: "text-3xl font-bold mb-2",
          "Maestro Forms Demo"
        }
        p {
          class: "text-gray-600",
          "A comprehensive demonstration of form handling with simulation mode."
        }

        div {
          class: "mt-4 space-x-2",
          button {
            class: "{async_class}",
            onclick: move |_| is_async.set(true),
            "Async Mode"
          }
          button {
            class: "{sync_class}",
            onclick: move |_| is_async.set(false),
            "Sync Mode"
          }
        }
      }

      Form {
        initial_value: User {
          role: Role::Admin,
          ..User::default()
        },
        onsubmit: on_submit,
        inner: FormContent,
      }
    }
  }
}

#[server(AsyncPost)]
pub async fn async_post(db_url: String) -> Result<(), ServerFnError> {
	use maestro_diesel::async_client::client::acreate_diesel_pool;
	let _pool = acreate_diesel_pool(&db_url);
	async_std::task::sleep(std::time::Duration::from_secs(1)).await;
	Ok(())
}

#[server(SyncPost)]
pub async fn sync_post(db_url: String) -> Result<(), ServerFnError> {
	use maestro_diesel::sync_client::create_db_pool_diesel;
	let _pool = create_db_pool_diesel(&db_url);
	async_std::task::sleep(std::time::Duration::from_secs(1));
	Ok(())
}
