use {
	crate::use_form_field::use_formik_field,
	dioxus::prelude::*,
	dioxus_sdk::utils::timing::use_debounce,
	serde::{Deserialize, Serialize},
	serde_json::Value,
	std::time::Duration,
	validator::Validate,
};

#[derive(Clone, PartialEq, Props)]
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

	let mut debounced_input = use_debounce(Duration::from_millis(200), move |text_input| {
		let value = Value::String(text_input);
		field.set_value(value.clone());
		if let Some(handler) = &props.oninput {
			handler.call(value);
		}
	});

	rsx! {
		input {
			r#type: "text",
			name: "{props.name}",
			value: "{field.get_value::<String>()}",
			oninput: move |evt| {
					debounced_input.action(evt.value());
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
