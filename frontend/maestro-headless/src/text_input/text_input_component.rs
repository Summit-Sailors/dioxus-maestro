use {
	crate::hooks::{UseControllableStateParams, use_controllable_state, use_interaction_state},
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
	pub on_value_change: Option<Callback<Option<String>>>,

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
	#[props(default = None)]
	pub onkeydown: Option<EventHandler<Event<KeyboardData>>>,
	#[props(default = None)]
	pub onkeyup: Option<EventHandler<Event<KeyboardData>>>,
	#[props(default = None)]
	pub onfocus: Option<EventHandler<Event<FocusData>>>,
	#[props(default = None)]
	pub onblur: Option<EventHandler<Event<FocusData>>>,
	#[props(default = None)]
	pub onmousedown: Option<EventHandler<Event<MouseData>>>,
	#[props(default = None)]
	pub onmouseup: Option<EventHandler<Event<MouseData>>>,
	#[props(default = None)]
	pub onmouseenter: Option<EventHandler<Event<MouseData>>>,
	#[props(default = None)]
	pub onmouseleave: Option<EventHandler<Event<MouseData>>>,

	#[props(extends = input, extends = GlobalAttributes)]
	pub attributes: Vec<Attribute>,
	pub children: Element,
}

#[component]
pub fn TextInput(props: TextInputProps) -> Element {
	let TextInputProps { value, default_value, on_value_change, debounce_ms, disabled, invalid, attributes, children, .. } = props;
	let mut interaction_state = use_interaction_state();
	let is_controlled = use_hook(move || value().is_some());
	let (value, set_value) =
		use_controllable_state(UseControllableStateParams { is_controlled, prop: value, default_prop: default_value, on_change: on_value_change });

	let mut on_input = use_debounce(Duration::from_millis(debounce_ms as u64), move |event: Event<FormData>| {
		if let Some(handler) = props.oninput {
			handler.call(event.clone());
		}
		set_value.call(Some(event.value()));
	});

	rsx! {
		input {
			disabled: disabled(),
			aria_disabled: disabled(),
			aria_invalid: invalid(),
			"data-disabled": disabled(),
			"data-invalid": invalid(),
			"data-hovered": *interaction_state.is_hovered.read(),
			"data-focused": *interaction_state.is_focused.read(),
			"data-focuse-visible": *interaction_state.is_focused.read(),
			value: value().unwrap_or_default(),
			onchange: move |event| {
					if let Some(handler) = props.onchange {
							handler.call(event.clone());
					}
					set_value.call(Some(event.value()));
			},
			oninput: move |event| {
					on_input.action(event);
			},
			onmounted: move |event| {
					interaction_state.self_ref.set(Some(event.clone()));
			},
			onmousedown: move |event| {
					if let Some(handler) = props.onmousedown {
							handler.call(event);
					}
			},
			onkeydown: move |event| {
					if let Some(handler) = props.onkeydown {
							handler.call(event);
					}
			},
			onkeyup: move |event| {
					if let Some(handler) = props.onkeyup {
							handler.call(event);
					}
			},
			onmouseup: move |event| {
					if let Some(handler) = props.onmouseup {
							handler.call(event);
					}
			},
			onmouseenter: move |event| {
					interaction_state.onmouseenter();
					if let Some(handler) = props.onmouseenter {
							handler.call(event);
					}
			},
			onmouseleave: move |event| {
					interaction_state.onmouseleave();
					if let Some(handler) = props.onmouseleave {
							handler.call(event);
					}
			},
			onfocus: move |event| {
					interaction_state.onfocus();
					if let Some(handler) = props.onfocus {
							handler.call(event);
					}
			},
			onblur: move |event| {
					interaction_state.onblur();
					if let Some(handler) = props.onblur {
							handler.call(event);
					}
			},

			..attributes,
			{children}
		}
	}
}
