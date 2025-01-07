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
	pub form_fields: Signal<Vec<FormField>>,
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
		let (form_fields, name_to_id_map, id_to_name_map) = if let Value::Object(map) = json_value {
			let form_fields: Signal<Vec<FormField>> = Signal::new(map.clone().into_iter().map(|(_key, value)| FormField::new(value)).collect());
			let name_to_id_map: ReadOnlySignal<HashMap<String, usize>> =
				ReadOnlySignal::new(Signal::new(map.clone().into_iter().enumerate().map(|(id, (key, _value))| (key, id)).collect()));
			let id_to_name_map: ReadOnlySignal<HashMap<usize, String>> =
				ReadOnlySignal::new(Signal::new(map.clone().into_iter().enumerate().map(|(id, (key, _value))| (id, key)).collect()));
			(form_fields, name_to_id_map, id_to_name_map)
		} else {
			panic!("Initial values must be serde_json::Value::Object serializable");
		};
		Self {
			initial_values: ReadOnlySignal::new(Signal::new(initial_values.to_owned())),
			form_fields,
			name_to_id_map,
			id_to_name_map,
			is_submitting: Signal::new(false),
			is_valid: Signal::new(true),
			is_dirty: Signal::new(false),
		}
	}

	pub fn reset_form(&mut self) {
		for field in self.form_fields.write().iter_mut() {
			field.reset();
		}
		self.is_dirty.set(false);
		self.is_valid.set(true);
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
		*self.form_fields.read().get(*self.name_to_id_map.read().get(&name).unwrap()).unwrap()
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
		let map_inverted = self.id_to_name_map.read();
		let json_map: Map<String, Value> =
			self.form_fields.read().iter().enumerate().map(|(id, field)| (map_inverted.get(&id).unwrap().clone(), field.get_json_value())).collect();
		serde_json::from_value(Value::Object(json_map)).expect("Failed to reconstruct form values into struct")
	}

	pub fn as_validated_struct(&mut self) -> (T, bool) {
		let form_struct = self.as_struct();
		match self.as_struct().validate() {
			Ok(()) => {
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
		(form_struct, *self.is_valid.read())
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

pub fn use_init_form_ctx<T>(initial_values: T) -> Formik<T>
where
	T: Validate + Clone + Serialize + PartialEq + 'static + for<'de> Deserialize<'de>,
{
	use_context_provider(|| Formik::new(initial_values))
}
