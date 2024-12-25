use {
	crate::use_form_field::use_formik_field,
	dioxus::prelude::*,
	serde::{Deserialize, Serialize},
	serde_json::Value,
	validator::Validate,
};

#[derive(PartialEq, Props, Clone)]
pub struct InputProps {
	pub name: String,
	#[props(optional)]
	pub oninput: Option<EventHandler<Value>>,
	#[props(optional)]
	pub onblur: Option<EventHandler<FocusEvent>>,
	#[props(extends = GlobalAttributes, extends = input)]
	pub attributes: Vec<Attribute>,
}

pub fn TextFormInput<TForm>(props: InputProps) -> Element
where
	TForm: Validate + Clone + Serialize + PartialEq + 'static + for<'de> Deserialize<'de>,
{
	let mut field = use_formik_field::<TForm>(props.name.clone());
	rsx! {
    input {
      r#type: "text",
      name: "{props.name}",
      value: "{field.value}",
      ontouchstart: move |_| field.set_touched(true),
      oninput: move |evt| {
          let value = Value::String(evt.value().clone());
          field.set_value(value.clone());
          if let Some(handler) = &props.oninput {
              handler.call(value);
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
