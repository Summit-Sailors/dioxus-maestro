use {
	crate::{
		button::Button,
		shared::{UseControllableStateParams, use_controllable_state},
	},
	dioxus::prelude::*,
};

#[derive(Props, PartialEq, Debug, Clone)]
pub struct ToggleProps {
	pub value: ReadOnlySignal<String>,
	#[props(default = ReadOnlySignal::new(Signal::new(None)))]
	pub pressed: ReadOnlySignal<Option<bool>>,
	#[props(default = false)]
	pub default_pressed: bool,
	#[props(default = None)]
	pub on_toggle_change: Option<Callback<bool>>,
	#[props(optional, default = ReadOnlySignal::new(Signal::new(false)))]
	pub disabled: ReadOnlySignal<bool>,

	#[props(extends = GlobalAttributes, extends = button)]
	pub attributes: Vec<Attribute>,
	#[props(default = Vec::new())]
	pub extra_attributes: Vec<Attribute>,
	#[props(default = None)]
	pub children: Option<Element>,
}

#[component]
pub fn Toggle(props: ToggleProps) -> Element {
	let ToggleProps { value, disabled, attributes, extra_attributes, on_toggle_change, pressed, default_pressed, children } = props;

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
							let new_toggle = !pressed();
							set_pressed(new_toggle);
					}
			},
			extra_attributes: attributes.clone(),
			"data-state": if pressed() { "on" } else { "off" },
			if let Some(children) = children {
				{children}
			} else {
				{
						rsx! {
							svg {
								stroke: "currentColor",
								fill: "currentColor",
								stroke_width: "0",
								view_box: "0 0 512 512",
								height: "24px",
								width: "24px",
								xmlns: "http://www.w3.org/2000/svg",
								path {
									fill: "none",
									stroke_linecap: "round",
									stroke_linejoin: "round",
									stroke_width: "40",
									d: "M196 220h64v172",
								}
								path {
									fill: "none",
									stroke_linecap: "round",
									stroke_miterlimit: "10",
									stroke_width: "40",
									d: "M187 396h138",
								}
								path { d: "M256 160a32 32 0 1 1 32-32 32 32 0 0 1-32 32z" }
							}
						}
				}
			}
		}
	}
}
