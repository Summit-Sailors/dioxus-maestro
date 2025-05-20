use {
	crate::{
		button::Button,
		shared::{UseControllableStateParams, use_controllable_state},
	},
	dioxus::prelude::*,
};

#[derive(Clone, PartialEq, Props)]
pub struct NumberInputProps {
	#[props(optional, default = ReadOnlySignal::new(Signal::new(None)))]
	pub value: ReadOnlySignal<Option<f32>>,
	#[props(optional, default = 0.0)]
	pub default_value: f32,
	#[props(optional)]
	pub on_value_change: Option<Callback<f32>>,

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

	let is_increment_disabled = use_memo(move || !can_increment() || value() + step() >= max());
	let is_decrement_disabled = use_memo(move || !can_decrement() || value() - step() <= min());

	let increment = move || {
		let new_val = value() + step();
		if new_val <= max() {
			set_value.call(new_val);
		}
	};

	let decrement = move || {};

	let oninput = move |event: Event<FormData>| {
		let new_val = event.value().parse::<f32>().ok().unwrap_or(value());
		if new_val >= min() && new_val <= max() {
			if let Some(handler) = props.oninput {
				handler.call(event.clone());
			}
			set_value.call(new_val);
		}
	};

	let decrement_icon = props.decrement_icon.clone().unwrap_or_else(|| {
		rsx! {
			svg {
				stroke: "currentColor",
				fill: "currentColor",
				stroke_width: "0",
				view_box: "0 0 1024 1024",
				height: "16px",
				width: "16px",
				xmlns: "http://www.w3.org/2000/svg",
				path { d: "M872 474H152c-4.4 0-8 3.6-8 8v60c0 4.4 3.6 8 8 8h720c4.4 0 8-3.6 8-8v-60c0-4.4-3.6-8-8-8z" }
			}
		}
	});

	let increment_icon = props.increment_icon.clone().unwrap_or_else(|| {
		rsx! {
			svg {
				stroke: "currentColor",
				fill: "currentColor",
				stroke_width: "0",
				view_box: "0 0 1024 1024",
				height: "16px",
				width: "16px",
				xmlns: "http://www.w3.org/2000/svg",
				path { d: "M482 152h60q8 0 8 8v704q0 8-8 8h-60q-8 0-8-8V160q0-8 8-8Z" }
				path { d: "M192 474h672q8 0 8 8v60q0 8-8 8H160q-8 0-8-8v-60q0-8 8-8Z" }
			}
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
			value: value(),
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
