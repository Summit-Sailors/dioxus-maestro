use {
	crate::{
		button::Button,
		hooks::{UseControllableStateParams, use_controllable_state},
	},
	dioxus::prelude::*,
	dioxus_free_icons::{
		Icon,
		icons::ld_icons::{LdMinus, LdPlus},
	},
};

#[derive(Props, Clone, PartialEq)]
pub struct NumberInputProps {
	#[props(optional, default = ReadOnlySignal::new(Signal::new(None)))]
	pub value: ReadOnlySignal<Option<f32>>,
	#[props(optional, default = 0.0)]
	pub default_value: f32,
	#[props(optional)]
	pub on_value_change: Option<Callback<Option<f32>>>,

	#[props(default = ReadOnlySignal::new(Signal::new(true)))]
	pub can_increment: ReadOnlySignal<bool>,
	#[props(default = ReadOnlySignal::new(Signal::new(true)))]
	pub can_decrement: ReadOnlySignal<bool>,
	#[props(default = ReadOnlySignal::new(Signal::new(0.0)))]
	pub min: ReadOnlySignal<f32>,
	#[props(default = ReadOnlySignal::new(Signal::new(100.0)))]
	pub max: ReadOnlySignal<f32>,
	#[props(default = ReadOnlySignal::new(Signal::new(1.0)))]
	pub step: ReadOnlySignal<f32>,

	#[props(default = ReadOnlySignal::new(Signal::new(false)))]
	pub disabled: ReadOnlySignal<bool>,
	#[props(default = ReadOnlySignal::new(Signal::new(false)))]
	pub invalid: ReadOnlySignal<bool>,

	#[props(default = None)]
	pub increment_icon: Option<Element>,
	#[props(default = None)]
	pub decrement_icon: Option<Element>,

	#[props(default = None)]
	pub oninput: Option<EventHandler<Event<FormData>>>,
	#[props(default = None)]
	pub onchange: Option<EventHandler<Event<FormData>>>,

	#[props(extends = input, extends = GlobalAttributes)]
	pub attributes: Vec<Attribute>,
	pub children: Element,
}

#[component]
pub fn NumberInput(props: NumberInputProps) -> Element {
	let NumberInputProps { value, default_value, on_value_change, disabled, invalid, can_increment, can_decrement, step, min, max, attributes, children, .. } =
		props;
	let is_controlled = use_hook(move || value().is_some());
	let (value, set_value) =
		use_controllable_state(UseControllableStateParams { is_controlled, prop: value, default_prop: default_value, on_change: on_value_change });

	let is_increment_disabled = use_memo(move || !can_increment() || value().unwrap_or_default() + step() >= max());
	let is_decrement_disabled = use_memo(move || !can_decrement() || value().unwrap_or_default() - step() <= min());

	let increment = move || {
		let new_val = value().unwrap_or_default() + step();
		if new_val <= max() {
			set_value.call(Some(new_val));
		}
	};

	let decrement = move || {
		let new_val = value().unwrap_or_default() - step();
		if new_val >= min() {
			set_value.call(Some(new_val));
		}
	};

	let oninput = move |event: Event<FormData>| {
		let new_val = event.value().parse::<f32>().ok().unwrap_or(value().unwrap_or_default());
		if new_val >= min() && new_val <= max() {
			if let Some(handler) = props.oninput {
				handler.call(event.clone());
			}
			set_value.call(Some(new_val));
		}
	};

	let decrement_icon = props.decrement_icon.clone().unwrap_or_else(|| {
		rsx! {
			Icon { width: 16, height: 16, icon: LdMinus }
		}
	});

	let increment_icon = props.increment_icon.clone().unwrap_or_else(|| {
		rsx! {
			Icon { width: 16, height: 16, icon: LdPlus }
		}
	});

	rsx! {
		Button {
			r#type: "button",
			disabled: is_decrement_disabled(),
			tabindex: if is_decrement_disabled() { "-1" } else { "0" },
			aria_label: "Decrement",
			"data-label": "decrement",
			onclick: move |_| {
					decrement();
			},
			{decrement_icon}
		}
		input {
			disabled: disabled(),
			aria_disabled: disabled(),
			aria_invalid: invalid(),
			"data-disabled": disabled(),
			"data-invalid": invalid(),
			inputmode: "numeric",
			value: value().unwrap_or_default(),
			onchange: move |event| {
					if let Some(handler) = props.onchange {
							handler.call(event.clone());
					}
					oninput(event);
			},
			oninput,
			..attributes,
			{children}
		}
		Button {
			r#type: "button",
			disabled: is_increment_disabled(),
			tabindex: if is_increment_disabled() { "-1" } else { "0" },
			aria_label: "Increment",
			"data-label": "increment",
			onclick: move |_| {
					increment();
			},
			{increment_icon}
		}
	}
}
