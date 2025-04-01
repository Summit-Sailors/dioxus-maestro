use {
	crate::{
		button::Button,
		shared::{UseControllableStateParams, use_controllable_state},
	},
	dioxus::prelude::*,
	std::fmt::Debug,
};

#[derive(Clone, PartialEq, Debug)]
pub struct CheckboxContext {
	pub checked: Memo<bool>,
	pub disabled: ReadOnlySignal<bool>,
}

impl CheckboxContext {
	pub fn new(checked: Memo<bool>, disabled: ReadOnlySignal<bool>) -> Self {
		Self { checked, disabled }
	}
}

#[derive(Props, PartialEq, Debug, Clone)]
pub struct CheckboxProps {
	#[props(optional, default = ReadOnlySignal::new(Signal::new(None)))]
	pub checked: ReadOnlySignal<Option<bool>>,
	#[props(optional, default = false)]
	pub default_checked: bool,
	#[props(default = None)]
	pub on_change: Option<Callback<bool>>,

	pub value: ReadOnlySignal<String>,
	pub name: String,
	#[props(optional, default = ReadOnlySignal::new(Signal::new(false)))]
	pub disabled: ReadOnlySignal<bool>,
	#[props(default = false)]
	pub required: bool,

	#[props(extends = GlobalAttributes, extends = button)]
	pub attributes: Vec<Attribute>,
	#[props(default = Vec::new())]
	pub extra_attributes: Vec<Attribute>,
	pub children: Element,
}

#[component]
pub fn Checkbox(props: CheckboxProps) -> Element {
	let CheckboxProps { checked, default_checked, on_change, value, name, disabled, required, attributes, extra_attributes, children } = props;
	let is_controlled = use_hook(move || checked().is_some());
	let (checked, set_checked) = use_controllable_state(UseControllableStateParams { is_controlled, prop: checked, default_prop: default_checked, on_change });
	use_context_provider::<CheckboxContext>(|| CheckboxContext::new(checked, disabled));

	let mut attributes = attributes.clone();
	attributes.extend(extra_attributes);

	rsx! {
		div { position: "relative",
			input {
				position: "absolute",
				width: 0,
				height: 0,
				opacity: 0,
				margin: 0,
				tabindex: -1,
				r#type: "checkbox",
				checked: checked(),
				name,
				value: value(),
				disabled: disabled(),
				required,
				aria_hidden: true,
			}
			Button {
				r#type: "button",
				tabindex: if disabled() { "-1" } else { "0" },
				role: "checkbox",
				disabled,
				aria_checked: checked(),
				aria_required: required,
				"data-state": if checked() { "checked" } else { "unchecked" },
				pointer_events: if disabled() { "none" } else { "auto" },
				onclick: move |_| set_checked(!checked()),
				extra_attributes: attributes,
				{children}
			}
		}
	}
}

#[derive(Props, PartialEq, Debug, Clone)]
pub struct CheckboxIndicatorProps {
	#[props(extends = GlobalAttributes)]
	pub attributes: Vec<Attribute>,
	#[props(default = None)]
	pub children: Option<Element>,
}

#[component]
pub fn CheckboxIndicator(props: CheckboxIndicatorProps) -> Element {
	let context = use_context::<CheckboxContext>();

	rsx! {
		span {
			aria_disabled: *context.disabled.read(),
			aria_checked: *context.checked.read(),
			"data-disabled": *context.disabled.read(),
			"data-state": if *context.checked.read() { "checked" } else { "unchecked" },
			pointer_events: "none",
			position: "relative",
			display: "flex",
			justify_content: "center",
			align_items: "center",
			..props.attributes,
			if *context.checked.read() {
				if let Some(children) = props.children {
					{children}
				} else {
					svg {
						stroke: "currentColor",
						fill: "currentColor",
						stroke_width: "0",
						view_box: "0 0 512 512",
						height: "16px",
						width: "16px",
						xmlns: "http://www.w3.org/2000/svg",
						path {
							fill: "none",
							stroke_linecap: "round",
							stroke_linejoin: "round",
							stroke_width: "32",
							d: "M416 128 192 384l-96-96",
						}
					}
				}
			}
		}
	}
}
