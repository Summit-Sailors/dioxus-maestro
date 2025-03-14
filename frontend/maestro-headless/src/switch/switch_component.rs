use {
	crate::{
		button::Button,
		hooks::{UseControllableStateParams, use_controllable_state},
	},
	dioxus::prelude::*,
};

#[derive(Props, PartialEq, Debug, Clone)]
pub struct SwitchProps {
	#[props(default = ReadOnlySignal::new(Signal::new("on".to_string())))]
	pub value: ReadOnlySignal<String>,

	#[props(default = ReadOnlySignal::new(Signal::new(None)))]
	pub checked: ReadOnlySignal<Option<bool>>,
	#[props(default = false)]
	pub default_checked: bool,
	#[props(default = None)]
	pub on_toggle_change: Option<Callback<Option<bool>>>,
	#[props(optional, default = ReadOnlySignal::new(Signal::new(false)))]
	pub disabled: ReadOnlySignal<bool>,
	#[props(optional, default = false)]
	pub required: bool,

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

	#[props(extends = GlobalAttributes, extends = button)]
	pub attributes: Vec<Attribute>,
	#[props(default = Vec::new())]
	pub additional_attributes: Vec<Attribute>,
	pub children: Option<Element>,
}

#[component]
pub fn Switch(props: SwitchProps) -> Element {
	let SwitchProps { value, disabled, required, attributes, additional_attributes, on_toggle_change, checked, default_checked, children, .. } = props;
	let is_controlled = use_hook(move || checked().is_some());
	let (checked, set_checked) =
		use_controllable_state(UseControllableStateParams { is_controlled, prop: checked, default_prop: default_checked, on_change: on_toggle_change });
	let mut attributes = attributes.clone();
	use_context_provider::<Memo<Option<bool>>>(|| checked);

	attributes.extend(additional_attributes);
	attributes.push(Attribute::new("aria-pressed", checked(), None, false));
	attributes.push(Attribute::new("aria-disabled", disabled(), None, false));
	attributes.push(Attribute::new("data-state", if checked().unwrap_or_default() { "on" } else { "off" }, None, false));
	if !attributes.iter().any(|x| x.name == "aria-role") {
		attributes.push(Attribute::new("aria-role", "radio", None, false));
	}

	rsx! {
		Button {
			value: value(),
			r#type: "button",
			disabled: disabled(),
			role: "switch",
			aria_checked: checked().unwrap_or_default(),
			aria_required: required,
			"data-state": if checked().unwrap_or_default() { "checked" } else { "unchecked" },
			aria_disabled: disabled(),
			onclick: move |_| {
					if !disabled() {
							let new_checked = !checked.peek().unwrap_or_default();
							set_checked(Some(new_checked));
					}
			},
			onblur: props.onblur,
			onfocus: props.onfocus,
			onkeydown: props.onkeydown,
			onkeyup: props.onkeyup,
			onmousedown: props.onmousedown,
			onmouseenter: props.onmouseenter,
			onmouseleave: props.onmouseleave,
			onmouseup: props.onmouseup,
			additional_attributes: attributes.clone(),

			{children}

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
	let checked = use_context::<Memo<Option<bool>>>();
	rsx! {
		span {
			"data-state": if checked().unwrap_or_default() { "checked" } else { "unchecked" },
			..props.attributes,
			{props.children}
		}
	}
}
