use {
	crate::button::Button,
	dioxus::prelude::*,
	dioxus_free_icons::{Icon, icons::bs_icons::BsInfo},
};

#[derive(Props, PartialEq, Debug, Clone)]
pub struct ToggleProps {
	#[props(optional, default = Signal::new(false))]
	pub disabled: Signal<bool>,
	#[props(extends = GlobalAttributes, extends = button)]
	pub attributes: Vec<Attribute>,
	#[props(default = Vec::new())]
	pub additional_attributes: Vec<Attribute>,
	#[props(default = None)]
	pub on_toggle_change: Option<Callback<bool>>,
	#[props(default = Signal::new(false))]
	pub pressed: Signal<bool>,
	#[props(default = None)]
	pub children: Option<Element>,
}

#[component]
pub fn Toggle(props: ToggleProps) -> Element {
	let ToggleProps { disabled, attributes, additional_attributes, on_toggle_change, mut pressed, children } = props;
	let mut attributes = attributes.clone();
	attributes.extend(additional_attributes);
	attributes.push(Attribute::new("aria-pressed", pressed(), None, false));
	attributes.push(Attribute::new("aria-disabled", pressed(), None, false));
	attributes.push(Attribute::new("data-state", if pressed() { "on" } else { "off" }, None, false));
	attributes.push(Attribute::new("data-disabled", disabled(), None, false));
	if !attributes.iter().any(|x| x.name == "aria-role") {
		attributes.push(Attribute::new("aria-role", "radio", None, false));
	}

	rsx! {
		Button {
			r#type: "button",
			onclick: move |_| {
					if !disabled() {
							let new_toggle = !*pressed.peek();
							pressed.toggle();
							if let Some(callback) = on_toggle_change {
									callback.call(new_toggle);
							}
					}
			},
			disabled,
			additional_attributes: attributes.clone(),
			if let Some(children) = children {
				{children}
			} else {
				Icon { icon: BsInfo }
			}
		}
	}
}
