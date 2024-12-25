use {
	crate::use_form_field::use_formik_field,
	dioxus::prelude::*,
	serde::{Deserialize, Serialize},
	serde_json::Value,
	validator::Validate,
};

#[derive(PartialEq, Props, Clone)]
pub struct TextAreaProps {
	pub name: String,
	#[props(default = 3)]
	pub rows: u32,
	#[props(optional)]
	pub onchange: Option<EventHandler<FormEvent>>,
	#[props(optional)]
	pub onblur: Option<EventHandler<FocusEvent>>,
	#[props(extends = GlobalAttributes, extends = textarea)]
	pub attributes: Vec<Attribute>,
}

#[component]
pub fn TextArea<T>(props: TextAreaProps) -> Element
where
	T: Validate + Clone + Serialize + PartialEq + 'static + for<'de> Deserialize<'de>,
{
	let mut field = use_formik_field::<T>(props.name.clone());

	rsx! {
    textarea {
      name: "{props.name}",
      rows: "{props.rows}",
      value: "{field.value}",
      ontouchstart: move |_| field.set_touched(true),
      onchange: move |evt| {
          field.set_value(Value::String(evt.value().clone()));
          if let Some(handler) = &props.onchange {
              handler.call(evt);
          }
      },
      onblur: move |evt| {
          field.set_touched(true);
          if let Some(handler) = &props.onblur {
              handler.call(evt);
          }
      },
      ..props.attributes,
    }
  }
}
