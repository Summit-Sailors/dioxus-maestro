use dioxus::prelude::*;
use maestro_forms::use_formik::Formik;
use serde::{Deserialize, Serialize};
use validator::Validate;

#[derive(Props, PartialEq)]
#[derive(Clone)]
pub struct FormStateDebuggerProps<T>
where
  T: Validate + Clone + Serialize + PartialEq + 'static + for<'de> Deserialize<'de>,
{
  pub form: Formik<T>,
}

#[component]
pub fn FormStateDebugger<T>(props: FormStateDebuggerProps<T>) -> Element
where
    T: Validate + Clone + Serialize + PartialEq + 'static + for<'de> Deserialize<'de>,
{
  let form = props.form;
  let mut show_debug = use_signal(|| false);

  rsx! {
    div { class: "mt-8 p-4 bg-gray-100 rounded-lg",
      button {
        class: "text-sm text-gray-600 hover:text-gray-800",
        onclick: move |_| {
          let current_value = *show_debug.read();
          show_debug.set(!current_value);
        },
        if *show_debug.read() {
          "Hide Form State"
        } else {
          "Show Form State"
        }
      }
      {show_debug().then(|| {
          let serialized_form = serde_json::to_string_pretty(&form.as_struct())
            .unwrap_or_else(|_| "Serialization error".to_string());
          rsx! {
            div { class: "mt-4 space-y-2",
              div {
                span { class: "font-semibold", "Is Valid: " }
                span { 
                  class: if *form.is_valid.read() { "text-green-600" } else { "text-red-600" },
                  "{form.is_valid.read()}"
                }
              }
              div {
                span { class: "font-semibold", "Is Dirty: " }
                "{form.is_dirty.read()}"
              }
              div {
                span { class: "font-semibold", "Is Submitting: " }
                "{form.is_submitting.read()}"
              }
              pre {
                class: "mt-4 p-4 bg-gray-800 text-white rounded overflow-auto",
                code { "{serialized_form}" }
              }
            }
          }
      })}
    }
  }
}
