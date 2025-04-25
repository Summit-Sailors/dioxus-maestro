use {
	crate::shared::{UseControllableStateParams, use_controllable_state},
	dioxus::prelude::*,
	dioxus_sdk::utils::timing::use_debounce,
	std::time::Duration,
};

#[derive(Props, Clone, PartialEq)]
pub struct TextInputProps {
	#[props(optional, default = ReadOnlySignal::new(Signal::new(None)))]
	pub value: ReadOnlySignal<Option<String>>,
	#[props(optional, default = String::new())]
	pub default_value: String,
	#[props(optional)]
	pub on_value_change: Option<Callback<String>>,

	#[props(default = 0)]
	pub debounce_ms: u32,
	#[props(default = ReadOnlySignal::new(Signal::new(false)))]
	pub disabled: ReadOnlySignal<bool>,
	#[props(default = ReadOnlySignal::new(Signal::new(false)))]
	pub invalid: ReadOnlySignal<bool>,

	#[props(default = None)]
	pub oninput: Option<EventHandler<Event<FormData>>>,
	#[props(default = None)]
	pub onchange: Option<EventHandler<Event<FormData>>>,

	#[props(extends = input, extends = GlobalAttributes)]
	pub attributes: Vec<Attribute>,
	#[props(default = Vec::new())]
	pub extra_attributes: Vec<Attribute>,
}

#[component]
pub fn TextInput(props: TextInputProps) -> Element {
	let TextInputProps { value, default_value, on_value_change, debounce_ms, disabled, invalid, attributes, oninput, onchange, extra_attributes } = props;

	let is_controlled = use_hook(move || value().is_some());
	let (value, set_value) =
		use_controllable_state(UseControllableStateParams { is_controlled, prop: value, default_prop: default_value, on_change: on_value_change });

	let mut on_input = use_debounce(Duration::from_millis(debounce_ms as u64), move |event: Event<FormData>| {
		if let Some(handler) = oninput {
			handler.call(event.clone());
		}
		set_value.call(event.value());
	});

	rsx! {
		input {
			disabled: disabled(),
			aria_disabled: disabled(),
			aria_invalid: invalid(),
			"data-disabled": disabled(),
			"data-invalid": invalid(),
			value: value(),
			onchange: move |event| {
					if let Some(handler) = onchange {
							handler.call(event.clone());
					}
					set_value.call(event.value());
			},
			oninput: move |event| {
					on_input.action(event);
			},
			..attributes,
			..extra_attributes,
		}
	}
}
