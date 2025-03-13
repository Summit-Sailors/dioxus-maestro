use {
	crate::{
		button::Button,
		hooks::{UseControllableStateParams, use_controllable_state},
	},
	dioxus::prelude::*,
	dioxus_free_icons::{Icon, icons::bs_icons::BsInfo},
};

#[derive(Props, PartialEq, Debug, Clone)]
pub struct ToggleProps {
	#[props(optional, default = ReadOnlySignal::new(Signal::new(false)))]
	pub disabled: ReadOnlySignal<bool>,
	#[props(extends = GlobalAttributes, extends = button)]
	pub attributes: Vec<Attribute>,
	#[props(default = Vec::new())]
	pub additional_attributes: Vec<Attribute>,
	#[props(default = None)]
	pub on_toggle_change: Option<Callback<Option<bool>>>,
	#[props(default = ReadOnlySignal::new(Signal::new(None)))]
	pub pressed: ReadOnlySignal<Option<bool>>,
	#[props(default = false)]
	pub default_pressed: bool,
	#[props(default = None)]
	pub children: Option<Element>,
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
}

#[component]
pub fn Toggle(props: ToggleProps) -> Element {
	let ToggleProps { disabled, attributes, additional_attributes, on_toggle_change, pressed, default_pressed, children, .. } = props;
	let is_controlled = use_hook(move || pressed().is_some());
	let (pressed, set_pressed) =
		use_controllable_state(UseControllableStateParams { is_controlled, prop: pressed, default_prop: default_pressed, on_change: on_toggle_change });
	let mut attributes = attributes.clone();

	attributes.extend(additional_attributes);
	attributes.push(Attribute::new("aria-pressed", pressed(), None, false));
	attributes.push(Attribute::new("aria-disabled", disabled(), None, false));
	attributes.push(Attribute::new("data-state", if pressed().unwrap_or_default() { "on" } else { "off" }, None, false));
	attributes.push(Attribute::new("data-disabled", disabled(), None, false));
	if !attributes.iter().any(|x| x.name == "aria-role") {
		attributes.push(Attribute::new("aria-role", "radio", None, false));
	}

	rsx! {
		Button {
			r#type: "button",
			onclick: move |_| {
					if !disabled() {
							let new_toggle = !pressed.peek().unwrap_or_default();
							set_pressed(Some(new_toggle));
					}
			},
			disabled,
			onblur: props.onblur,
			onfocus: props.onfocus,
			onkeydown: props.onkeydown,
			onkeyup: props.onkeyup,
			onmousedown: props.onmousedown,
			onmouseenter: props.onmouseenter,
			onmouseleave: props.onmouseleave,
			onmouseup: props.onmouseup,
			additional_attributes: attributes.clone(),
			if let Some(children) = children {
				{children}
			} else {
				Icon { icon: BsInfo }
			}
		}
	}
}
