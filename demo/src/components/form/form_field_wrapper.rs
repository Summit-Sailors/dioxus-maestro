use {
  dioxus::prelude::*,
  maestro_forms::use_form_field::FormField,
  tailwind_fuse::tw_join,
};

#[derive(Props, PartialEq, Clone)]
pub struct FormFieldWrapperProps {
  pub label: &'static str,
  pub field: FormField,
  pub children: Element,
  #[props(default = true)]
  pub show_validation: bool,
  #[props(optional)]
  pub required: Option<bool>,
  #[props(optional)]
  pub help_text: Option<String>,
}

#[component]
pub fn FormFieldWrapper(props: FormFieldWrapperProps) -> Element {
  let show_validation = props.show_validation;
  let has_error = show_validation && !props.field.errors.read().is_empty() && *props.field.touched.read();
  
  let label_class = tw_join!(
    "block text-sm font-medium mb-1",
    if has_error { "text-red-500" } else { "text-gray-200" }
  );

  rsx! {
    div {
      class: "form-group grid",
      label {
        class: "{label_class}",
        "{props.label}"
        if props.required.unwrap_or(false) {
          span {
            class: "text-red-500 ml-1",
            "*"
          }
        }
      }
      div {
        class: "relative bg-gray-900",
        {props.children}
        if let Some(help_text) = &props.help_text {
          p {
            class: "mt-1 text-sm text-gray-400",
            "{help_text}"
          }
        }
        {(has_error).then(|| rsx! {
          div {
            class: "text-red-500 text-sm mt-1",
            "{props.field.errors.read().join(\", \")}"
          }
        })}
        {(show_validation && *props.field.touched.read()).then(|| rsx! {
          div {
            class: "absolute top-2.5 right-2",
            if has_error {
              i { class: "fas fa-exclamation-circle text-red-500" }
            } else {
              i { class: "fas fa-check-circle text-green-500" }
            }
          }
        })}
      }
    }
  }
}
