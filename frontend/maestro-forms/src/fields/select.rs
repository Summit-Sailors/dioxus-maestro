use {
	crate::use_form_field::use_formik_field,
	dioxus::prelude::*,
	serde::{Deserialize, Serialize},
	serde_json::Value,
	std::collections::HashMap,
	validator::Validate,
};

#[derive(Clone, PartialEq, Props)]
pub struct SelectProps<TField>
where
	TField: Clone + Serialize + PartialEq + 'static + for<'de> Deserialize<'de>,
{
	pub name: String,
	#[props(optional)]
	pub label_map: Option<HashMap<Value, String>>,
	#[props(optional)]
	pub onchange: Option<EventHandler<TField>>,
	#[props(optional)]
	pub onblur: Option<EventHandler<FocusEvent>>,
	#[props(extends = GlobalAttributes, extends = select)]
	pub attributes: Vec<Attribute>,
}

impl<TField> SelectProps<TField>
where
	TField: Clone + Serialize + PartialEq + 'static + for<'de> Deserialize<'de>,
{
	fn get_inverted_label_map(&self) -> Option<HashMap<String, Value>> {
		if let Some(map) = &self.label_map {
			return Some(map.clone().into_iter().map(|(k, v)| (v, k)).collect());
		}
		None
	}
}

#[component]
pub fn SelectFormField<TForm, TField>(props: SelectProps<TField>) -> Element
where
	TForm: Validate + Clone + Serialize + PartialEq + 'static + for<'de> Deserialize<'de>,
	TField: Clone + Serialize + PartialEq + 'static + for<'de> Deserialize<'de>,
{
	let mut field = use_formik_field::<TForm>(props.name.clone());
	let map = use_signal(|| props.label_map.clone());
	let inverted_map = use_signal(|| props.get_inverted_label_map());
	let values = use_signal(|| map().into_iter().flatten().map(|map| map.0).collect::<Vec<Value>>());

	rsx! {
    select {
      name: props.name,
      multiple: false,
      ontouchstart: move |_| field.set_touched(true),
      onchange: move |evt| {
          match map() {
              Some(_) => {
                  let label = evt.value();
                  let inverted_map = inverted_map().unwrap();
                  let value = inverted_map.get(&label).unwrap();
                  field.set_value(value.clone());
                  if let Some(handler) = props.onchange {
                      handler.call(serde_json::from_value(value.clone()).unwrap());
                  }
              }
              None => {
                  let value = Value::from(evt.value());
                  field.set_value(Value::from(evt.value()));
                  if let Some(handler) = props.onchange {
                      handler.call(serde_json::from_value(value.clone()).unwrap());
                  }
              }
          }
      },
      onblur: move |evt| {
          field.set_touched(true);
          if let Some(handler) = props.onblur {
              handler.call(evt);
          }
      },
      ..props.attributes,
      match props.label_map {
          Some(label_map) => {
              rsx! {
                {
                    values
                        .iter()
                        .map(|value| {
                            let label = label_map.get(&value).unwrap();
                            rsx! {
                              option { value: "{label}", selected: field.value.read().clone() == value.clone(), "{label}" }
                            }
                        })
                }
              }
          }
          None => {
              rsx! {
                {
                    values
                        .iter()
                        .map(|value| {
                            rsx! {
                              option {
                                value: value.to_string(),
                                selected: field.value.read().clone() == value.clone(),
                                "{value}"
                              }
                            }
                        })
                }
              }
          }
      }
    }
  }
}
