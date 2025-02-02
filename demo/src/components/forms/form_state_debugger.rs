use {
  dioxus::prelude::*,
  maestro_forms::use_formik::Formik,
  serde::{Deserialize, Serialize},
  validator::Validate,
};

#[derive(Props, PartialEq, Clone)]
pub struct FormStateDebuggerProps<T>
where
  T: Validate + Clone + Serialize + PartialEq + 'static + for<'de> Deserialize<'de>,
{
  pub form: Formik<T>,
  #[props(optional)]
  pub initial_expanded: Option<bool>,
}

#[component]
pub fn FormStateDebugger<T>(props: FormStateDebuggerProps<T>) -> Element
where
  T: Validate + Clone + Serialize + PartialEq + 'static + for<'de> Deserialize<'de>,
{
  let form = props.form;
  let mut show_debug = use_signal(|| props.initial_expanded.unwrap_or(false));
  let mut serialized_form = use_signal(String::new);
  let mut field_values = use_signal(Vec::new);

  use_effect(move || {
    let serialized = serde_json::to_string_pretty(&form.as_struct())
        .unwrap_or_else(|_| "Serialization error".to_string());
    serialized_form.set(serialized);

    let mut values = Vec::new();
    for (name, _) in form.name_to_id_map.read().iter() {
      let value = form.get_field_json_value(name.to_string());
      values.push((name.clone(), value));
    }
    field_values.set(values);
  });

  rsx! {
    div { 
      class: "mt-8 p-4 bg-gray-100 rounded-lg",
      button {
        r#type: "button",
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
      {show_debug.read().then(|| rsx! {
        div { 
          class: "mt-4 space-y-2",
          // form status
          div {
            class: "grid grid-cols-3 gap-4",
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
          }
          // field values
          div {
            class: "mt-4",
            h3 { class: "font-semibold mb-2", "Field Values:" }
            div {
              class: "space-y-2",
              {field_values.read().iter().map(|(name, value)| {
                rsx! {
                  div {
                    class: "grid grid-cols-2",
                    span { class: "font-medium", "{name}: " }
                    span { "{value}" }
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
              class: "p-4 bg-gray-800 text-white rounded overflow-auto",
              code { "{serialized_form.read()}" }
            }
          }
        }
    })}
    }
  }
}
