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
	pub value: ReadOnlySignal<String>,
	#[props(default = ReadOnlySignal::new(Signal::new(None)))]
	pub pressed: ReadOnlySignal<Option<bool>>,
	#[props(default = false)]
	pub default_pressed: bool,
	#[props(default = None)]
	pub on_toggle_change: Option<Callback<Option<bool>>>,
	#[props(optional, default = ReadOnlySignal::new(Signal::new(false)))]
	pub disabled: ReadOnlySignal<bool>,

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
	pub extra_attributes: Vec<Attribute>,
	#[props(default = None)]
	pub children: Option<Element>,
}

#[component]
pub fn Toggle(props: ToggleProps) -> Element {
	let ToggleProps { value, disabled, attributes, extra_attributes, on_toggle_change, pressed, default_pressed, children, .. } = props;
	let is_controlled = use_hook(move || pressed().is_some());
	let (pressed, set_pressed) =
		use_controllable_state(UseControllableStateParams { is_controlled, prop: pressed, default_prop: default_pressed, on_change: on_toggle_change });
	let mut attributes = attributes.clone();

	attributes.extend(extra_attributes);

	rsx! {
		Button {
			value: value(),
			r#type: "button",
			disabled,
			role: "radio",
			aria_pressed: pressed(),
			onclick: move |_| {
					if !disabled() {
							let new_toggle = !pressed.peek().unwrap_or_default();
							set_pressed(Some(new_toggle));
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
			extra_attributes: attributes.clone(),
			"data-state": if pressed().unwrap_or_default() { "on" } else { "off" },
			if let Some(children) = children {
				{children}
			} else {
				Icon { icon: BsInfo }
			}
		}
	}
}
