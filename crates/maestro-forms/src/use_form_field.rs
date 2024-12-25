use {
	dioxus::prelude::*,
	serde::{Deserialize, Serialize},
	serde_json::Value,
	validator::Validate,
};

#[derive(Debug, PartialEq)]
pub struct FormField {
	pub value: Signal<Value>,
	pub errors: Signal<Vec<String>>,
	pub touched: Signal<bool>,
	pub initial_value: ReadOnlySignal<Value>,
}

impl FormField {
	pub fn new(initial_value: Value) -> Self {
		Self {
			value: Signal::new(initial_value.clone()),
			errors: Signal::new(Vec::new()),
			touched: Signal::new(false),
			initial_value: ReadOnlySignal::new(Signal::new(initial_value.clone())),
		}
	}

	pub fn reset(&mut self) {
		let initial_value = self.initial_value.read().clone();
		self.set_json_value(initial_value);
		self.errors.clear();
		self.set_touched(false);
	}

	pub fn set_json_value(&mut self, value: Value) {
		self.value.set(value);
	}

	pub fn set_value<TValue>(&mut self, value: TValue)
	where
		TValue: Serialize,
	{
		self.set_json_value(serde_json::to_value(value).unwrap());
	}

	pub fn get_json_value(&self) -> Value {
		self.value.read().clone()
	}

	pub fn get_value<TReturnType>(&self) -> TReturnType
	where
		TReturnType: for<'de> serde::Deserialize<'de>,
	{
		serde_json::from_value(self.get_json_value()).unwrap()
	}

	pub fn push_error(&mut self, error: String) {
		self.errors.push(error);
	}

	pub fn set_touched(&mut self, touched: bool) {
		self.touched.set(touched);
	}
}

impl From<FormField> for String {
	fn from(val: FormField) -> Self {
		serde_json::from_value(val.value.read().clone()).expect("Failed to parse with serde_json")
	}
}

impl From<FormField> for usize {
	fn from(val: FormField) -> Self {
		serde_json::from_value(val.value.read().clone()).expect("Failed to parse with serde_json")
	}
}

impl From<FormField> for url::Url {
	fn from(val: FormField) -> Self {
		let value: String = serde_json::from_value(val.value.read().clone()).expect("Failed to parse with serde_json");
		url::Url::parse(value.as_str()).expect("cant parse url")
	}
}

impl Clone for FormField {
	fn clone(&self) -> Self {
		*self
	}
}

impl Copy for FormField {}

pub fn use_formik_field<T>(name: impl Into<String>) -> FormField
where
	T: Validate + Clone + Serialize + PartialEq + 'static + for<'de> Deserialize<'de>,
{
	use super::use_formik::Formik;
	use_context::<Formik<T>>().get_form_field(name.into())
}
