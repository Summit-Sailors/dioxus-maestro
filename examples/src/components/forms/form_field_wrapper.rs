use dioxus::prelude::*;
use maestro_forms::use_form_field::FormField;

#[derive(Props, PartialEq)]
pub struct FormFieldWrapperProps {
  pub label: &'static str,
  pub field: FormField,
  pub children: Element,
}

#[component]
pub fn FormFieldWrapper(cx: Scope, props: FormFieldWrapperProps) -> Element {
  let has_error = !cx.props.field.errors.read().is_empty() && *cx.props.field.touched.read();
  
  cx.render(rsx! {
    div { class: "form-group",
      label { 
        class: "block text-sm font-medium mb-1 {if has_error { 'text-red-600' } else { 'text-gray-700' }}", 
        "{props.label}"
      }
      div { class: "relative",
        &props.children
        
        {has_error.then(|| rsx! {
          div { 
            class: "text-red-600 text-sm mt-1",
            "{props.field.errors.read().join(", ")}"
          }
        })}
        
        // validation state indicator
        {props.field.touched.read().then(|| rsx! {
          div { 
            class: "absolute top-2.5 right-2",
            if has_error {
              rsx! { i { class: "fas fa-exclamation-circle text-red-600" } }
            } else {
              rsx! { i { class: "fas fa-check-circle text-green-600" } }
            }
          }
        })}
      }
    }
  })
}
