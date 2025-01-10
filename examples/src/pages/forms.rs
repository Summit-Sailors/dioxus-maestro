use crate::{
  models::user::User,
  components::forms::{form_field_wrapper::FormFieldWrapper, form_state_debugger::FormStateDebugger},
};
use dioxus::prelude::*;
use maestro_forms::fields::{Form, SelectFormField, TextFormInput, TextArea};
use maestro_sqlx::{
  async_client::apalis_storage::create_apalis_storage_async,
  sync_client::apalis_storage::create_apalis_storage_sync,
};
use maestro_toast::use_toast;

/// FormsDemo component showcases the capabilities of maestro-forms.
/// It provides a complete example of form handling with validation,
/// async/sync database operations, and real-time form state management.
#[component]
pub fn FormsDemo(cx: Scope) -> Element {
  let toast = use_toast(cx);
  let is_async = use_state(cx, || true);
  let loading = use_state(cx, || false);
  
  let roles = vec![
    "admin".to_string(),
    "user".to_string(),
    "moderator".to_string(),
  ];

  let on_submit = move |event: FormEvent| {
    loading.set(true);
    
    if *is_async.get() {
      spawn(async move {
        // async storage example
        let storage = create_apalis_storage_async().await;
        // API call simulate
        tokio::time::sleep(std::time::Duration::from_secs(1)).await;
        loading.set(false);
        toast.success("Form submitted successfully (async)!");
      });
    } else {
      // sync storage example
      let storage = create_apalis_storage_sync();
      // processing simaulate
      std::thread::sleep(std::time::Duration::from_secs(1));
      loading.set(false);
      toast.success("Form submitted successfully (sync)!");
    }
  };

  cx.render(rsx! {
    div { class: "max-w-4xl mx-auto p-6",
      // header section
      div { class: "mb-8",
        h1 { class: "text-3xl font-bold mb-2",
          "Maestro Forms Demo"
        }
        p { class: "text-gray-600",
          "A comprehensive demonstration of form handling with validation and database integration."
        }
        
        // toggle async/sync mode (you can use what is best for you application "entirely")
        div { class: "mt-4",
          button {
            class: "px-4 py-2 rounded {if *is_async.get() { 'bg-blue-500 text-white' } else { 'bg-gray-200' }}",
            onclick: move |_| is_async.set(true),
            "Async Mode"
          }
          button {
            class: "ml-2 px-4 py-2 rounded {if !*is_async.get() { 'bg-blue-500 text-white' } else { 'bg-gray-200' }}",
            onclick: move |_| is_async.set(false),
            "Sync Mode"
          }
        }
      }
      
      // main form
      Form {
        initial_value: User::default(),
        onsubmit: on_submit,
        
        move |form| rsx! {
          div { class: "space-y-4 bg-white p-6 rounded-lg shadow",
            FormFieldWrapper {
              label: "Username",
              field: form.get_form_field("username".to_string()),
              TextFormInput::<User> {
                name: "username",
                class: "mt-1 block w-full rounded-md border-gray-300 shadow-sm focus:border-blue-500 focus:ring-blue-500"
              }
            }

            FormFieldWrapper {
              label: "Email",
              field: form.get_form_field("email".to_string()),
              TextFormInput::<User> {
                name: "email",
                class: "mt-1 block w-full rounded-md border-gray-300 shadow-sm focus:border-blue-500 focus:ring-blue-500"
              }
            }

            FormFieldWrapper {
              label: "Bio",
              field: form.get_form_field("bio".to_string()),
              TextArea::<User> {
                name: "bio",
                rows: 4,
                class: "mt-1 block w-full rounded-md border-gray-300 shadow-sm focus:border-blue-500 focus:ring-blue-500"
              }
            }

            FormFieldWrapper {
              label: "Role",
              field: form.get_form_field("role".to_string()),
              SelectFormField::<User, String> {
                name: "role",
                values: roles,
                class: "mt-1 block w-full rounded-md border-gray-300 shadow-sm focus:border-blue-500 focus:ring-blue-500"
              }
            }

            // Submit button with loading state
            button {
              class: "w-full mt-6 px-4 py-2 bg-blue-500 text-white rounded hover:bg-blue-600 focus:outline-none focus:ring-2 focus:ring-blue-500 focus:ring-offset-2 disabled:opacity-50",
              r#type: "submit",
              disabled: "{loading}",
              
              if *loading.get() {
                  rsx! {
                    span { class: "inline-flex items-center",
                      span { class: "mr-2", "Loading..." }
                      // TODO: a loading spinner component here
                    }
                  }
              } else {
                rsx! { "Submit" }
              }
            }
          }

          // form state debugger
          FormStateDebugger { form: form }
        }
      }
    }
  })
}
