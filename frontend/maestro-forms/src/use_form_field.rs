use dioxus::prelude::*;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use validator::Validate;

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
		self.set_json_value(serde_json::to_value(value).expect("Failed to serialize with serde_json"));
	}

	pub fn get_json_value(&self) -> Value {
		self.value.read().clone()
	}

	pub fn get_value<TReturnType>(&self) -> TReturnType
	where
		TReturnType: for<'de> serde::Deserialize<'de>,
	{
		serde_json::from_value(self.value.read().clone()).expect("Failed to parse with serde_json")
	}

	pub fn clear_errors(&mut self) {
		self.errors.clear();
	}

	pub fn push_error(&mut self, error: String) {
		self.errors.push(error);
	}

	pub fn set_touched(&mut self, touched: bool) {
		self.touched.set(touched);
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
	use_context::<super::use_formik::Formik<T>>().get_form_field(name.into())
}
