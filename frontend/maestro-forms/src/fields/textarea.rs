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
pub struct TextAreaProps {
	pub name: String,
	#[props(default = 3)]
	pub rows: u32,
	#[props(optional)]
	pub oninput: Option<EventHandler<Value>>,
	#[props(optional)]
	pub onblur: Option<EventHandler<FocusEvent>>,
	#[props(optional)]
	pub onfocus: Option<EventHandler<FocusEvent>>,
	#[props(extends = GlobalAttributes, extends = textarea)]
	pub attributes: Vec<Attribute>,
}

#[component]
pub fn TextArea<T>(props: TextAreaProps) -> Element
where
	T: Validate + Clone + Serialize + PartialEq + 'static + for<'de> Deserialize<'de>,
{
	let mut field = use_formik_field::<T>(props.name.clone());

	let mut debounced_input = use_debounce(Duration::from_millis(200), move |text_input| {
		let value = Value::String(text_input);
		field.set_value(value.clone());
		if let Some(handler) = &props.oninput {
			handler.call(value);
		}
	});

	rsx! {
		textarea {
			name: "{props.name}",
			rows: "{props.rows}",
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
			onfocus: move |evt| {
				if let Some(handler) = &props.onfocus {
						handler.call(evt);
				}
		},
			..props.attributes,
		}
	}
}
