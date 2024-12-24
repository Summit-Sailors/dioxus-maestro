use {
	dioxus::prelude::*,
	serde::{Deserialize, Serialize},
	serde_json::{Map, Value},
	std::collections::HashMap,
	validator::Validate,
};

#[derive(Debug, Clone)]
pub struct FormField {
	pub value: Signal<Value>,
	pub error: Signal<Option<String>>,
	pub touched: Signal<bool>,
	pub initial_value: Value,
}

impl FormField {
	pub fn new(initial_value: Value) -> Self {
		Self { value: Signal::new(initial_value.clone()), error: Signal::new(None), touched: Signal::new(false), initial_value }
	}

	pub fn reset(&mut self) {
		self.value.set(self.initial_value.clone());
		self.error.set(None);
		self.touched.set(false);
	}
}

#[derive(Debug, Clone)]
pub struct Formik<T>
where
	T: Validate + Clone + Serialize + for<'de> Deserialize<'de>,
{
	pub initial_values: T,
	pub fields: HashMap<String, FormField>,
	pub is_submitting: bool,
	pub is_valid: bool,
	pub is_dirty: bool,
}

impl<T> Formik<T>
where
	T: Validate + Clone + Serialize + for<'de> Deserialize<'de>,
{
	pub fn new(initial_values: T) -> Self {
		let json_value = serde_json::to_value(&initial_values).expect("Failed to serialize initial values");
		let fields = if let Value::Object(map) = json_value {
			map.into_iter().map(|(key, value)| (key, FormField::new(value))).collect()
		} else {
			panic!("Initial values must be serde_json serializable");
		};

		Self { initial_values, fields, is_submitting: false, is_valid: true, is_dirty: false }
	}

	pub fn validate_all(&mut self) -> bool {
		match self.get_values_as_struct().validate() {
			Ok(_) => {
				self.is_valid = true;
			},
			Err(errors) => {
				for (field_name, field_errors) in errors.field_errors() {
					if let Some(field) = self.fields.get_mut(field_name) {
						field.error.set(Some(format!("{:?}", field_errors)));
					}
				}
				self.is_valid = false;
			},
		}
		self.is_valid
	}

	pub fn reset_form(&mut self) {
		for field in self.fields.values_mut() {
			field.reset();
		}
		self.is_dirty = false;
		self.is_valid = true;
	}

	pub fn set_field_value(&mut self, name: &str, value: Value) {
		if let Some(field) = self.fields.get_mut(name) {
			field.value.set(value);
			self.is_dirty = true;
		}
	}

	pub fn set_field_error(&mut self, name: &str, error: Option<String>) {
		if let Some(field) = self.fields.get_mut(name) {
			field.error.set(error);
		}
	}

	pub fn get_values_as_struct(&self) -> T {
		let json_map: Map<String, Value> = self.fields.iter().map(|(key, field)| (key.clone(), field.value.read().clone())).collect();
		serde_json::from_value(Value::Object(json_map)).expect("Failed to reconstruct form values into struct")
	}
}
