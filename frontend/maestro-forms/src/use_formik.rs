use {
	super::use_form_field::FormField,
	dioxus::prelude::*,
	serde::{Deserialize, Serialize},
	serde_json::{Map, Value},
	std::collections::HashMap,
	validator::Validate,
};

#[derive(Debug, PartialEq)]
pub struct Formik<T>
where
	T: Validate + Clone + Serialize + PartialEq + 'static + for<'de> Deserialize<'de>,
{
	pub initial_values: ReadOnlySignal<T>,
	pub fields: ReadOnlySignal<Vec<FormField>>,
	pub name_to_id_map: ReadOnlySignal<HashMap<String, usize>>,
	pub id_to_name_map: ReadOnlySignal<HashMap<usize, String>>,
	pub is_submitting: Signal<bool>,
	pub is_valid: Signal<bool>,
	pub is_dirty: Signal<bool>,
}

impl<T> Formik<T>
where
	T: Validate + Clone + Serialize + PartialEq + 'static + for<'de> Deserialize<'de>,
{
	pub fn new(initial_values: T) -> Self {
		let json_value = serde_json::to_value(&initial_values).expect("Failed to serialize initial values");
		let (fields, name_to_id_map, id_to_name_map) = if let Value::Object(map) = json_value {
			let fields_vec: ReadOnlySignal<Vec<FormField>> =
				ReadOnlySignal::new(Signal::new(map.clone().into_iter().map(|(_key, value)| FormField::new(value)).collect()));
			let name_to_id_map: ReadOnlySignal<HashMap<String, usize>> =
				ReadOnlySignal::new(Signal::new(map.clone().into_iter().enumerate().map(|(id, (key, _value))| (key, id)).collect()));
			let id_to_name_map: ReadOnlySignal<HashMap<usize, String>> =
				ReadOnlySignal::new(Signal::new(map.clone().into_iter().enumerate().map(|(id, (key, _value))| (id, key)).collect()));
			(fields_vec, name_to_id_map, id_to_name_map)
		} else {
			panic!("Initial values must be serde_json serializable");
		};
		Self {
			initial_values: ReadOnlySignal::new(Signal::new(initial_values.to_owned())),
			fields,
			name_to_id_map,
			id_to_name_map,
			is_submitting: Signal::new(false),
			is_valid: Signal::new(true),
			is_dirty: Signal::new(false),
		}
	}

	pub fn validate_all(&mut self) -> bool {
		match self.as_struct().validate() {
			Ok(_) => {
				self.is_valid.set(true);
			},
			Err(errors) => {
				for (field_name, field_errors) in errors.field_errors() {
					for field_error in field_errors {
						self.push_field_error(field_name.to_string(), field_error.message.as_deref().unwrap_or("Unknown error").to_owned());
					}
				}
				self.is_valid.set(false);
			},
		}
		*self.is_valid.read()
	}

	pub fn reset_form(&mut self) {
		for mut field in self.fields.read().clone() {
			field.reset();
		}
		self.is_dirty.set(false);
		self.is_valid.set(true);
	}

	fn get_name_to_id_map(&self) -> HashMap<String, usize> {
		self.name_to_id_map.read().clone()
	}

	fn get_id_to_name_map(&self) -> HashMap<usize, String> {
		self.id_to_name_map.read().clone()
	}

	fn get_fields(&self) -> Vec<FormField> {
		self.fields.read().clone()
	}

	pub fn set_field_json_value(&mut self, name: String, value: Value) {
		self.get_form_field(name).set_json_value(value);
		self.is_dirty.set(true);
	}

	pub fn set_field_value<TFieldType: Serialize>(&mut self, name: String, value: TFieldType) {
		self.get_form_field(name).set_value(value);
		self.is_dirty.set(true);
	}

	pub fn get_field_json_signal(&mut self, name: String) -> Signal<Value> {
		self.get_form_field(name).value
	}

	pub fn get_field_json_value(&mut self, name: String) -> Value {
		self.get_field_json_signal(name).read().clone()
	}

	pub fn get_form_field(&self, name: String) -> FormField {
		let map = self.get_name_to_id_map();
		*self.get_fields().get(*map.get(&name).unwrap()).unwrap()
	}

	pub fn get_field_value<TReturnType>(&mut self, name: String) -> TReturnType
	where
		TReturnType: for<'de> serde::Deserialize<'de>,
	{
		serde_json::from_value(self.get_field_json_value(name)).unwrap()
	}

	pub fn push_field_error(&mut self, name: String, error: String) {
		self.get_form_field(name).push_error(error);
	}

	pub fn as_struct(&self) -> T {
		let map_inverted = self.get_id_to_name_map();
		let json_map: Map<String, Value> =
			self.fields.read().clone().iter().enumerate().map(|(id, field)| (map_inverted.get(&id).unwrap().clone(), field.get_json_value())).collect();
		serde_json::from_value(Value::Object(json_map)).expect("Failed to reconstruct form values into struct")
	}
}

impl<T> Clone for Formik<T>
where
	T: Validate + Clone + Serialize + PartialEq + 'static + for<'de> Deserialize<'de>,
{
	fn clone(&self) -> Self {
		*self
	}
}

impl<T> Copy for Formik<T> where T: Validate + Clone + Serialize + PartialEq + 'static + for<'de> Deserialize<'de> {}

pub fn use_formik<T>(initial_values: T) -> Formik<T>
where
	T: Validate + Clone + Serialize + PartialEq + 'static + for<'de> Deserialize<'de>,
{
	use_context_provider(|| Formik::new(initial_values))
}
