use {
	crate::{
		button::Button,
		shared::{UseControllableStateParams, use_controllable_state},
	},
	dioxus::prelude::*,
};

#[derive(Props, PartialEq, Debug, Clone)]
pub struct SwitchProps {
	#[props(default = ReadOnlySignal::new(Signal::new("on".to_string())))]
	pub value: ReadOnlySignal<String>,
	#[props(default = "switch".to_string())]
	pub name: String,

	#[props(default = ReadOnlySignal::new(Signal::new(None)))]
	pub checked: ReadOnlySignal<Option<bool>>,
	#[props(default = false)]
	pub default_checked: bool,
	#[props(default = None)]
	pub on_toggle_change: Option<Callback<bool>>,
	#[props(optional, default = ReadOnlySignal::new(Signal::new(false)))]
	pub disabled: ReadOnlySignal<bool>,
	#[props(optional, default = false)]
	pub required: bool,

	#[props(extends = GlobalAttributes, extends = button)]
	pub attributes: Vec<Attribute>,
	pub children: Option<Element>,
}

#[component]
pub fn Switch(props: SwitchProps) -> Element {
	let SwitchProps { value, disabled, required, attributes, on_toggle_change, checked, default_checked, name, children } = props;
	let is_controlled = use_hook(move || checked().is_some());
	let (checked, set_checked) =
		use_controllable_state(UseControllableStateParams { is_controlled, prop: checked, default_prop: default_checked, on_change: on_toggle_change });
	use_context_provider::<Memo<bool>>(|| checked);

	rsx! {
		div { position: "relative",
			input {
				position: "absolute",
				width: 0,
				height: 0,
				opacity: 0,
				margin: 0,
				tabindex: -1,
				r#type: "radio",
				value: value.clone(),
				checked: checked(),
				name,
				disabled: disabled(),
				required,
				aria_hidden: true,
			}
			Button {
				r#type: "button",
				role: "switch",
				disabled,
				tabindex: if disabled() { "-1" } else { "0" },
				aria_required: required,
				aria_checked: checked(),
				"data-state": if checked() { "checked" } else { "unchecked" },
				onclick: move |_| {
						set_checked(!checked());
				},
				extra_attributes: attributes,
				{children}
			}
		}
	}
}

#[derive(Props, PartialEq, Debug, Clone)]
pub struct SwitchIndicatorProps {
	#[props(extends = GlobalAttributes, extends = span)]
	pub attributes: Vec<Attribute>,
	#[props(optional)]
	pub children: Element,
}

#[component]
pub fn SwitchIndicator(props: SwitchIndicatorProps) -> Element {
	let checked = use_context::<Memo<bool>>();
	rsx! {
		span {
			"data-state": if checked() { "checked" } else { "unchecked" },
			..props.attributes,
			{props.children}
		}
	}
}
