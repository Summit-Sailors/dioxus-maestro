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
}

#[component]
pub fn FormFieldWrapper(props: FormFieldWrapperProps) -> Element {
  let show_validation = props.show_validation;
  let has_error = show_validation && !props.field.errors.read().is_empty() && *props.field.touched.read();

  let label_class = tw_join!(
    "block text-sm font-medium mb-1",
    if has_error { "text-red-600" } else { "text-gray-700" }
  );

  rsx! {
    div {
      class: "form-group",
      label {
        class: "{label_class}",
        "{props.label}"
      }
      div {
        class: "relative",
        {props.children}
        {(has_error).then(|| rsx! {
          div {
            class: "text-red-600 text-sm mt-1",
            "{props.field.errors.read().join(\", \")}"
          }
        })}
        {(show_validation && *props.field.touched.read()).then(|| rsx! {
          div {
            class: "absolute top-2.5 right-2",
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
