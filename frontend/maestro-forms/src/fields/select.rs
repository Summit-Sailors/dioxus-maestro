use {
	crate::use_form_field::use_formik_field,
	dioxus::prelude::*,
	serde::{Deserialize, Serialize},
	std::{collections::HashMap, fmt::Debug, hash::Hash},
	validator::Validate,
};

#[derive(Clone, PartialEq, Props)]
pub struct SelectProps<TField>
where
	TField: Clone + Serialize + PartialEq + Hash + Eq + Debug + 'static + for<'de> Deserialize<'de>,
{
	pub name: String,
	pub values: Vec<TField>,
	#[props(optional)]
	pub labels: Option<Vec<String>>,
	#[props(optional)]
	pub onchange: Option<EventHandler<TField>>,
	#[props(optional)]
	pub onblur: Option<EventHandler<FocusEvent>>,
	#[props(extends = GlobalAttributes, extends = select)]
	pub attributes: Vec<Attribute>,
}

#[component]
pub fn SelectFormField<TForm, TField>(SelectProps::<TField> { attributes, labels, name, values, onblur, onchange }: SelectProps<TField>) -> Element
where
	TForm: Validate + Clone + Serialize + PartialEq + 'static + for<'de> Deserialize<'de>,
	TField: Clone + Serialize + PartialEq + Hash + Eq + Debug + 'static + for<'de> Deserialize<'de>,
{
	let mut field = use_formik_field::<TForm>(name.clone());
	let value_to_label_map = use_memo({
		to_owned![labels, values];
		move || {
			labels
				.clone()
				.map(|labels| values.clone().into_iter().zip(labels.into_iter()).collect::<HashMap<TField, String>>())
				.clone()
				.unwrap_or(values.clone().into_iter().map(|value| (value.clone(), format!("{value:#?}"))).collect::<HashMap<TField, String>>())
		}
	});
	let label_to_value_map = use_memo(move || value_to_label_map().clone().into_iter().map(|(k, v)| (v, k)).collect::<HashMap<String, TField>>());

	rsx! {
		select {
			name,
			multiple: false,
			onchange: move |evt| {
					let label_to_value_map = label_to_value_map();
					let value = label_to_value_map.get(&evt.value()).unwrap();
					field.set_value(value.clone());
					if let Some(handler) = onchange {
							handler.call(value.clone());
					}
			},
			onblur: move |evt| {
					field.set_touched(true);
					if let Some(handler) = onblur {
							handler.call(evt);
					}
			},
			..attributes,

			{
					values
							.clone()
							.into_iter()
							.map(|value| {
									let map = value_to_label_map();
									let label = map.get(&value).unwrap();
									rsx! {
										option { value: "{label}", selected: field.get_value::<TField>() == value, "{label}" }
									}
							})
			}
		}
	}
}
