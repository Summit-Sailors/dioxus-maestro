use {crate::shared::extract_attribute_value, dioxus::prelude::*};

pub fn use_attributes(mut element_ref: Signal<Option<web_sys::Element>>, attribures: Vec<Attribute>) {
	use_effect(use_reactive!(|attribures| {
		element_ref.with_mut(|element| {
			if let Some(element) = element {
				attribures.iter().for_each(|attr| {
					let value_str = extract_attribute_value(&attr.value);
					element.set_attribute(attr.name, &value_str).unwrap();
				});
			};
		});
	}));
}
