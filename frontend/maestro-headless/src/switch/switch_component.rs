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
	#[props(default = "switch".to_string())]
	pub name: String,

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
	pub children: Option<Element>,
}

#[component]
pub fn Switch(props: SwitchProps) -> Element {
	let SwitchProps { value, disabled, required, attributes, on_toggle_change, checked, default_checked, children, .. } = props;
	let is_controlled = use_hook(move || checked().is_some());
	let (checked, set_checked) =
		use_controllable_state(UseControllableStateParams { is_controlled, prop: checked, default_prop: default_checked, on_change: on_toggle_change });
	use_context_provider::<Memo<Option<bool>>>(|| checked);

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
				checked: checked().unwrap_or_default(),
				name: props.name.clone(),
				disabled: disabled(),
				required,
				aria_hidden: true,
			}
			Button {
				role: "switch",
				tabindex: if disabled() { "-1" } else { "0" },
				aria_disabled: disabled(),
				aria_required: required,
				aria_checked: checked(),
				"data-state": if checked().unwrap_or_default() { "checked" } else { "unchecked" },
				onclick: move |_| {
						match !checked().unwrap_or_default() {
								true => set_checked(Some(true)),
								false => set_checked(None),
						};
				},
				onmousedown: props.onmousedown,
				onkeydown: props.onkeydown,
				onkeyup: props.onkeyup,
				onmouseup: props.onmouseup,
				onmouseenter: props.onmouseenter,
				onmouseleave: props.onmouseleave,
				onfocus: props.onfocus,
				onblur: props.onblur,
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
	let checked = use_context::<Memo<Option<bool>>>();
	rsx! {
		span {
			"data-state": if checked().unwrap_or_default() { "checked" } else { "unchecked" },
			..props.attributes,
			{props.children}
		}
	}
}
