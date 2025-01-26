use {
	crate::{
		components::forms::{form_field_wrapper::FormFieldWrapper, form_state_debugger::FormStateDebugger},
		models::user::User,
	}, async_std::task::sleep, dioxus::prelude::*, maestro_forms::fields::{
		form::{Form, InnerComponentProps},
		select::SelectFormField,
		text::TextFormInput,
		textarea::TextArea,
	}, maestro_toast::{
    ctx::use_toast, toast_info::ToastInfo
  }, 
  maestro_ui::button::{Button, ButtonSize, ButtonType, ButtonVariant}
};

const AVAILABLE_ROLES: &[&str] = &["admin", "user", "moderator"];

pub fn form_content(props: InnerComponentProps<User>) -> Element {
  let loading = use_signal(|| false);

  rsx! {
    div { class: "space-y-6",
      FormFieldWrapper {
        label: "Username",
        field: props.form.get_form_field("username".to_string()),
        TextFormInput::<User> {
          name: "username",
          placeholder: "Enter your username",
          class: "w-full p-2 rounded-md border border-gray-300 focus:ring focus:ring-blue-300 focus:outline-none",
        }
      }

      FormFieldWrapper {
        label: "Email",
        field: props.form.get_form_field("email".to_string()),
        TextFormInput::<User> {
          name: "email",
          placeholder: "Enter your email address",
          class: "w-full p-2 rounded-md border border-gray-300 focus:ring focus:ring-blue-300 focus:outline-none",
        }
      }

      FormFieldWrapper {
        label: "Bio",
        field: props.form.get_form_field("bio".to_string()),
        TextArea::<User> {
          name: "bio",
          placeholder: "Tell us about yourself...",
          rows: 4,
          class: "w-full p-2 rounded-md border border-gray-300 focus:ring focus:ring-blue-300 focus:outline-none",
        }
      }

      FormFieldWrapper {
        label: "Role",
        field: props.form.get_form_field("role".to_string()),
        SelectFormField::<User, String> {
          name: "role",
          values: AVAILABLE_ROLES.iter().map(|&s| s.to_string()).collect(),
          class: "w-full p-2 rounded-md border border-gray-300 focus:ring focus:ring-blue-300 focus:outline-none",
        }
      }

      Button {
        class: format!(
          "mt-4 py-2 rounded-md text-white font-semibold {} transition-opacity duration-200",
          if loading() { "bg-gray-400 cursor-not-allowed opacity-70" } else { "bg-blue-500 hover:bg-blue-600" }
        ),
        button_type: ButtonType::Submit,
        disabled: loading(),
        size: ButtonSize::Default,
        variant: ButtonVariant::Default,
        if loading() { "Loading..." } else { "Submit" }
      }

      FormStateDebugger {
        form: props.form,
      }
    }
  }
}

#[component]
pub fn FormsDemo() -> Element {
  let mut toast = use_toast();
  let mut is_async = use_signal(|| true);

  let on_submit = Some(Callback::new(
    move |(event, (submitted_user, is_valid)): (Event<FormData>, (User, bool))| {
    event.prevent_default();

    let db_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");

    if is_valid {
      spawn(async move {
        match async_post(db_url).await {
          Ok(_) => {
            toast.write().popup(
              ToastInfo::builder()
                .context(format!(
                  "Form submitted successfully for user: {}",
                  submitted_user.username
                ))
                .build(),
            );
          }
          Err(_) => {
            toast.write().popup(
              ToastInfo::builder()
                .context("Submission failed".to_string())
                .build(),
            );
          }
        }
      });
    } else {
      toast.write().popup(
        ToastInfo::builder()
          .context("Form validation failed. Please check your inputs.".to_owned())
          .build(),
      );
    }
  }));


  let async_class = if *is_async.read() { "bg-blue-500 text-white" } else { "bg-gray-200" };
  let sync_class = if !*is_async.read() { "bg-blue-500 text-white" } else { "bg-gray-200" };

  rsx! {
    div { class: "max-w-4xl mx-auto p-6",
      div { class: "mb-8",
        h1 { class: "text-3xl font-bold mb-2", "Maestro Forms Demo" }
        p { class: "text-gray-600",
          "A comprehensive demonstration of form handling with validation and database integration."
        }

        div { class: "mt-4",
          button {
            class: format!("px-4 py-2 rounded {}", async_class),
            onclick: move |_| is_async.set(true),
            "Async Mode"
          }
          button {
            class: format!("ml-2 px-4 py-2 rounded {}", sync_class),
            onclick: move |_| is_async.set(false),
            "Sync Mode"
          }
        }
      }

      Form {
        initial_value: User::default(),
        onsubmit: on_submit,
        inner: form_content
      }
    
    }
  }
}

#[server(AsyncPost)]
pub async fn async_post(db_url: String) -> Result<(), ServerFnError> {
	use maestro_diesel::async_client::client::acreate_diesel_pool;
	let _pool = acreate_diesel_pool(&db_url);
	sleep(std::time::Duration::from_secs(1)).await;
	Ok(())
}

#[server(SyncPost)]
pub async fn sync_post(db_url: String) -> Result<(), ServerFnError> {
	use maestro_diesel::sync_client::create_db_pool_diesel;
	let _pool = create_db_pool_diesel(&db_url);
	std::thread::sleep(std::time::Duration::from_secs(1));
	Ok(())
}
