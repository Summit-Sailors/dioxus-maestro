use dioxus::prelude::*;
use maestro_forms::use_formik::Formik;
use serde::Serialize;
use validator::Validate;

#[derive(Props)]
pub struct FormStateDebuggerProps<T>
where
  T: Validate + Clone + Serialize + PartialEq + 'static,
{
  pub form: Formik<T>,
}

#[component]
pub fn FormStateDebugger<T>(cx: Scope<FormStateDebuggerProps<T>>) -> Element
where
  T: Validate + Clone + Serialize + PartialEq + 'static,
{
  let form = cx.props.form;
  let show_debug = use_state(cx, || false);
  
  cx.render(rsx! {
    div { class: "mt-8 p-4 bg-gray-100 rounded-lg",
      button {
        class: "text-sm text-gray-600 hover:text-gray-800",
        onclick: move |_| show_debug.set(!show_debug.get()),
        if *show_debug.get() {
          "Hide Form State"
        } else {
          "Show Form State"
        }
      }
      
      {show_debug.then(|| rsx! {
        div { class: "mt-4 space-y-2",
          div {
            span { class: "font-semibold", "Is Valid: " }
            span { class: if *form.is_valid.read() { "text-green-600" } else { "text-red-600" },
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
          pre { class: "mt-4 p-4 bg-gray-800 text-white rounded overflow-auto",
            code {
              "{serde_json::to_string_pretty(&form.as_struct()).unwrap()}"
            }
          }
        }
      })}
    }
  })
}
