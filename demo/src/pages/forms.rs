use dioxus::prelude::*;
use maestro_forms::fields::{form::{Form, InnerComponentProps}, select::SelectFormField, text::TextFormInput, textarea::TextArea};
use maestro_sqlx::{async_client::apalis_storage::create_apalis_storage_async, sync_client::apalis_storage::create_apalis_storage_sync};
use maestro_toast::{ctx::use_toast, toast_info::ToastInfo};

use crate::{components::forms::{form_field_wrapper::FormFieldWrapper, form_state_debugger::FormStateDebugger}, models::user::User};

const AVAILABLE_ROLES: &[&str] = &["admin", "user", "moderator"];

fn form_content(props: InnerComponentProps<User>) -> Element {
  let loading = use_signal(|| false);
  
  rsx! {
    div { class: "space-y-4 bg-white p-6 rounded-lg shadow",
      FormFieldWrapper {
        label: "Username",
        field: props.form.get_form_field("username".to_string()),
        TextFormInput::<User> {
          name: "username",
          class: "mt-1 block w-full rounded-md border-gray-300 shadow-sm focus:border-blue-500 focus:ring-blue-500"
        }
      }

      FormFieldWrapper {
        label: "Email",
        field: props.form.get_form_field("email".to_string()),
        TextFormInput::<User> {
          name: "email",
          class: "mt-1 block w-full rounded-md border-gray-300 shadow-sm focus:border-blue-500 focus:ring-blue-500"
        }
      }

      FormFieldWrapper {
        label: "Bio",
        field: props.form.get_form_field("bio".to_string()),
        TextArea::<User> {
          name: "bio",
          rows: 4,
          class: "mt-1 block w-full rounded-md border-gray-300 shadow-sm focus:border-blue-500 focus:ring-blue-500"
        }
      }

      FormFieldWrapper {
        label: "Role",
        field: props.form.get_form_field("role".to_string()),
        SelectFormField::<User, String> {
          name: "role",
          values: AVAILABLE_ROLES.iter().map(|&s| s.to_string()).collect(),
          class: "mt-1 block w-full rounded-md border-gray-300 shadow-sm focus:border-blue-500 focus:ring-blue-500"
        }
      }

      button {
        class: "w-full mt-6 px-4 py-2 bg-blue-500 text-white rounded hover:bg-blue-600 focus:outline-none focus:ring-2 focus:ring-blue-500 focus:ring-offset-2 disabled:opacity-50",
        r#type: "submit",
        disabled: *loading.read(),
        
        if *loading.read() {
          span { 
            class: "inline-flex items-center",
            span { 
              class: "mr-2", 
              "Loading..." 
            }
          }
        } else {
          "Submit"
        }
      }
      
      FormStateDebugger { form: props.form }
    }
  }
}

#[component]
pub fn FormsDemo() -> Element {
  let mut toast = use_toast();
  let mut is_async = use_signal(|| true);

  let on_submit = move |_event: FormEvent| {
    if *is_async.read() {
      spawn(async move {
        let _storage = create_apalis_storage_async::<User>().await;
        tokio::time::sleep(std::time::Duration::from_secs(1)).await;
        toast.write().popup(ToastInfo::builder().context("Form submitted successfully (async)!".to_string()).build());
      });
    } else {
      let _storage = create_apalis_storage_sync::<User>();
      std::thread::sleep(std::time::Duration::from_secs(1));
      toast.write().popup(ToastInfo::builder().context("Form submitted successfully (sync)!".to_string()).build());
    }
  };

  let async_class = if *is_async.read() { "bg-blue-500 text-white" } else { "bg-gray-200" };
  let sync_class = if !*is_async.read() { "bg-blue-500 text-white" } else { "bg-gray-200" };

  rsx! {
    div { class: "max-w-4xl mx-auto p-6",
      div { class: "mb-8",
        h1 { class: "text-3xl font-bold mb-2", "Maestro Forms Demo" }
        p { class: "text-gray-600", "A comprehensive demonstration of form handling with validation and database integration." }

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
