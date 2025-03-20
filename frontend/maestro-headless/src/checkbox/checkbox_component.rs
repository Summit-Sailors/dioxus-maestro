use {
	crate::{
		button::Button,
		hooks::{UseControllableStateParams, use_controllable_state},
	},
	dioxus::prelude::*,
	dioxus_free_icons::{Icon, icons::bs_icons::BsCheck},
	std::fmt::Debug,
};

#[derive(Clone, PartialEq, Debug)]
pub struct CheckboxContext {
	pub value: ReadOnlySignal<String>,
	pub name: String,
	pub checked: Memo<Option<bool>>,
	pub required: bool,
	pub disabled: ReadOnlySignal<bool>,
	pub on_change: Callback<Option<bool>>,
}

impl CheckboxContext {
	pub fn new(
		value: ReadOnlySignal<String>,
		on_change: Callback<Option<bool>>,
		checked: Memo<Option<bool>>,
		name: String,
		required: bool,
		disabled: ReadOnlySignal<bool>,
	) -> Self {
		Self { value, on_change, checked, name, required, disabled }
	}
}

#[derive(Props, PartialEq, Debug, Clone)]
pub struct CheckboxProps {
	#[props(optional, default = ReadOnlySignal::new(Signal::new(None)))]
	pub checked: ReadOnlySignal<Option<bool>>,
	#[props(optional, default = false)]
	pub default_checked: bool,
	#[props(default = None)]
	pub on_change: Option<Callback<Option<bool>>>,

	pub value: ReadOnlySignal<String>,
	pub name: String,
	#[props(optional, default = ReadOnlySignal::new(Signal::new(false)))]
	pub disabled: ReadOnlySignal<bool>,
	#[props(default = false)]
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
	pub extra_attributes: Vec<Attribute>,
	pub children: Element,
}

#[component]
pub fn Checkbox(props: CheckboxProps) -> Element {
	let CheckboxProps { disabled, value, name, checked, default_checked, attributes, extra_attributes, on_change, children, required, .. } = props;
	let is_controlled = use_hook(move || checked().is_some());
	let (checked, set_checked) = use_controllable_state(UseControllableStateParams { is_controlled, prop: checked, default_prop: default_checked, on_change });
	let context = use_context_provider::<CheckboxContext>(|| CheckboxContext::new(value, set_checked, checked, name, required, disabled));
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
				checked: *context.checked.read(),
				name: context.name.clone(),
				disabled: disabled(),
				required: context.required,
				aria_hidden: true,
			}
			Button {
				tabindex: if disabled() { "-1" } else { "0" },
				role: "checkbox",
				disabled,
				aria_checked: *context.checked.read(),
				aria_required: context.required,
				"data-state": if context.checked.read().unwrap_or_default() { "checked" } else { "unchecked" },
				onclick: move |_| {
						match !context.checked.peek().unwrap_or_default() {
								true => context.on_change.call(Some(true)),
								false => context.on_change.call(None),
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
			"data-state": if context.checked.read().unwrap_or_default() { "checked" } else { "unchecked" },
			pointer_events: "none",
			position: "relative",
			display: "flex",
			justify_content: "center",
			align_items: "center",
			..props.attributes,
			if context.checked.read().unwrap_or_default() {
				if let Some(children) = props.children {
					{children}
				} else {
					Icon { icon: BsCheck }
				}
			}
		}
	}
}
