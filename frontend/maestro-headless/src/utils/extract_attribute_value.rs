use dioxus::dioxus_core::AttributeValue;

pub fn extract_attribute_value(value: &AttributeValue) -> String {
	match value {
		AttributeValue::Text(s) => s.to_string(),
		AttributeValue::Int(i) => i.to_string(),
		AttributeValue::Float(f) => f.to_string(),
		AttributeValue::Bool(b) => b.to_string(),
		AttributeValue::Any(any) => {
			let any_ref = any.as_any();
			if let Some(s) = any_ref.downcast_ref::<String>() {
				s.to_string()
			} else if let Some(i) = any_ref.downcast_ref::<i32>() {
				i.to_string()
			} else if let Some(f) = any_ref.downcast_ref::<f64>() {
				f.to_string()
			} else if let Some(b) = any_ref.downcast_ref::<bool>() {
				b.to_string()
			} else {
				format!("{:?}", any_ref)
			}
		},
		AttributeValue::None => "".to_string(),
		_ => format!("{:?}", value),
	}
}
