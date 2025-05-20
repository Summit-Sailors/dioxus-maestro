use {
	crate::{
		button::Button,
		shared::{UseControllableStateParams, use_controllable_state},
	},
	dioxus::prelude::*,
	std::fmt::Debug,
};

#[derive(Debug, Clone, PartialEq)]
pub struct CheckboxContext {
	pub checked: Memo<bool>,
	pub disabled: ReadOnlySignal<bool>,
}

impl CheckboxContext {
	pub fn new(checked: Memo<bool>, disabled: ReadOnlySignal<bool>) -> Self {
		Self { checked, disabled }
	}
}

#[derive(Debug, Clone, PartialEq, Props)]
pub struct CheckboxRootProps {
	#[props(optional, default = ReadOnlySignal::new(Signal::new(None)))]
	pub checked: ReadOnlySignal<Option<bool>>,
	#[props(optional, default = false)]
	pub default_checked: bool,
	#[props(default = None)]
	pub on_change: Option<Callback<bool>>,

	pub value: ReadOnlySignal<String>,
	#[props(optional, default = ReadOnlySignal::new(Signal::new(false)))]
	pub disabled: ReadOnlySignal<bool>,
	#[props(default = ReadOnlySignal::new(Signal::new(false)))]
	pub required: ReadOnlySignal<bool>,

	#[props(extends = GlobalAttributes, extends = button)]
	pub attributes: Vec<Attribute>,
	#[props(default = Vec::new())]
	pub extra_attributes: Vec<Attribute>,
	pub children: Element,
}

#[component]
pub fn CheckboxRoot(props: CheckboxRootProps) -> Element {
	let CheckboxRootProps { checked, default_checked, on_change, value, disabled, required, attributes, extra_attributes, children } = props;
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
				margin: "-1px",
				z_index: -10,
				tabindex: -1,
				r#type: "checkbox",
				checked: checked(),
				value: value(),
				disabled: disabled(),
				required: required(),
				aria_hidden: true,
			}
			Button {
				r#type: "button",
				tabindex: if disabled() { "-1" } else { "0" },
				role: "checkbox",
				disabled,
				aria_checked: checked().then_some(Some(true)),
				aria_required: required().then_some(Some(true)),
				"data-required": required().then_some(Some(true)),
				"data-state": if checked() { "checked" } else { "unchecked" },
				pointer_events: disabled().then_some(Some("none")),
				onclick: move |_| set_checked(!checked()),
				extra_attributes: attributes,
				{children}
			}
		}
	}
}

#[derive(Debug, Clone, PartialEq, Props)]
pub struct CheckboxIndicatorProps {
	#[props(extends = GlobalAttributes)]
	pub attributes: Vec<Attribute>,
	#[props(default = Vec::new())]
	pub extra_attributes: Vec<Attribute>,
	#[props(default = None)]
	pub children: Option<Element>,
}

#[component]
pub fn CheckboxIndicator(props: CheckboxIndicatorProps) -> Element {
	let context = use_context::<CheckboxContext>();

	rsx! {
		span {
			aria_disabled: context.disabled.read().then_some(Some(true)),
			aria_checked: context.checked.read().then_some(Some(true)),
			"data-disabled": context.disabled.read().then_some(Some(true)),
			"data-state": if *context.checked.read() { "checked" } else { "unchecked" },
			pointer_events: "none",
			position: "relative",
			display: "flex",
			justify_content: "center",
			align_items: "center",
			..props.attributes,
			..props.extra_attributes,
			if *context.checked.read() {
				if let Some(children) = props.children {
					{
							rsx! {
								{children}
							}
					}
				} else {
					{
							rsx! {
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
	}
}
