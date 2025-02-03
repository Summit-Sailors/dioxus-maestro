use {
  dioxus::prelude::*, 
  maestro_forms::use_formik::Formik, 
  maestro_ui::button::{Button, ButtonType}, 
  serde::{Deserialize, Serialize}, 
  tailwind_fuse::tw_join, 
  validator::Validate
};

#[derive(Props, PartialEq, Clone)]
pub struct FormStateDebuggerProps<T>
where
  T: Validate + Clone + Serialize + PartialEq + 'static + for<'de> Deserialize<'de>,
{
  pub form: Formik<T>,
  #[props(default = false)]
  pub initial_expanded: bool,
}

#[component]
pub fn FormStateDebugger<T>(props: FormStateDebuggerProps<T>) -> Element
where
  T: Validate + Clone + Serialize + PartialEq + 'static + for<'de> Deserialize<'de>,
{
  let form = props.form;
  let mut show_debug = use_signal(|| props.initial_expanded);

  type ClassFn = fn(bool) -> &'static str;

  rsx! {
    div {
      class: "mt-8 p-4 bg-gray-100 rounded-lg",
      Button {
        button_type: ButtonType::Button,
        class: "text-sm text-gray-600 hover:text-gray-800",
        on_click: move |_| show_debug.set(!show_debug()),
        if show_debug() { "Hide Form State" } else { "Show Form State" }
      }
      
      if show_debug() {
          div {
            class: "mt-4 space-y-2",
            
            // form status grid
            div {
              class: "grid grid-cols-3 gap-4",
              {[
                ("Is Valid", form.is_valid.read(), 
                  (|v: bool| v.then_some("text-green-600").unwrap_or("text-red-600")) as ClassFn),
                ("Is Dirty", form.is_dirty.read(), 
                  (|_: bool| "") as ClassFn),
                ("Is Submitting", form.is_submitting.read(), 
                  (|_: bool| "") as ClassFn)
              ].iter().map(|(label, value, class_fn)| rsx! {
                div {
                  span { class: "font-semibold", "{label}: " }
                  span { 
                    class: tw_join!(class_fn(**value)),
                    "{value}" 
                  }
                }
              })}
            }
            
            // field values
            div {
              class: "mt-4",
              h3 { class: "font-semibold mb-2", "Field Values:" }
              div {
                class: "space-y-2",
                {form.name_to_id_map.read().keys().map(|name| {
                  let value = form.get_field_json_value(name.clone());
                  rsx! {
                    div {
                      class: "grid grid-cols-2",
                      span { 
                        class: "font-medium", 
                        "{name}: " 
                      }
                      span { 
                        class: "break-words", 
                        "{value}" 
                      }
                    }
                  }
                })}
              }
            }
            
            // complete form state
            div {
              class: "mt-4",
              h3 { class: "font-semibold mb-2", "Complete Form State:" }
              pre {
                class: "p-4 bg-gray-800 text-white rounded overflow-auto max-h-96 overflow-y-auto",
                code { 
                  "{serde_json::to_string_pretty(&form.as_struct()).unwrap_or_else(|_| \"Serialization error\".to_string())}" 
                }
              }
            }
          }
      }
    }
  }
}
