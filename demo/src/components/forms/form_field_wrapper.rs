use dioxus::prelude::*;
use maestro_forms::use_form_field::FormField;

#[derive(Props, PartialEq, Clone)]
pub struct FormFieldWrapperProps {
  pub label: &'static str,
  pub field: FormField,
  pub children: Element,
}

#[component]
pub fn FormFieldWrapper(props: FormFieldWrapperProps) -> Element {
    let has_error = !props.field.errors.read().is_empty() && *props.field.touched.read();
    let label_class = if has_error { "block text-sm font-medium mb-1 text-red-600" } else { "block text-sm font-medium mb-1 text-gray-700" };

    rsx! {
      div { class: "form-group",
        label { class: "{label_class}", "{props.label}" }
        div { class: "relative",
          {props.children}

          {has_error.then(|| rsx! {
            div { 
              class: "text-red-600 text-sm mt-1",
              "{props.field.errors.read().join(\", \")}"
            }
          })}

          {props.field.touched.read().then(|| rsx! {
            div { class: "absolute top-2.5 right-2",
              if has_error {
                i { class: "fas fa-exclamation-circle text-red-600" }
              } else {
                i { class: "fas fa-check-circle text-green-600" }
              }
            }
        })}
      }
    }
  }
}
