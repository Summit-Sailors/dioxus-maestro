use {
	crate::{
		button::Button,
		hooks::{UseControllableStateParams, use_controllable_state},
	},
	dioxus::prelude::*,
	std::fmt::Debug,
};

#[derive(Clone, PartialEq, Debug)]
pub struct RadioContext {
	pub value: ReadOnlySignal<String>,
	pub name: String,
	pub checked: Memo<Option<bool>>,
	pub required: bool,
	pub disabled: ReadOnlySignal<bool>,
	pub on_change: Callback<Option<bool>>,
}

impl RadioContext {
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
pub struct RadioProps {
	pub value: ReadOnlySignal<String>,
	pub name: String,
	#[props(optional, default = ReadOnlySignal::new(Signal::new(None)))]
	pub checked: ReadOnlySignal<Option<bool>>,
	#[props(optional, default = false)]
	pub default_checked: bool,
	#[props(default = None)]
	pub on_change: Option<Callback<Option<bool>>>,

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
pub fn Radio(props: RadioProps) -> Element {
	let RadioProps { disabled, value, name, checked, default_checked, attributes, extra_attributes, on_change, children, required, .. } = props;
	let is_controlled = use_hook(move || checked().is_some());
	let (checked, set_checked) = use_controllable_state(UseControllableStateParams { is_controlled, prop: checked, default_prop: default_checked, on_change });
	let context = use_context_provider::<RadioContext>(|| RadioContext::new(value, set_checked, checked, name, required, disabled));
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
				r#type: "radio",
				checked: context.checked.read().unwrap_or_default(),
				name: context.name,
				disabled: disabled(),
				aria_hidden: true,
				required: context.required,
			}
			Button {
				tabindex: if disabled() { "-1" } else { "0" },
				role: "radio",
				aria_checked: *context.checked.read(),
				aria_required: context.required,
				"data-state": if context.checked.read().unwrap_or_default() { "checked" } else { "unchecked" },
				disabled,
				onclick: move |_| {
						match !context.checked.peek().unwrap_or_default() {
								true => context.on_change.call(Some(true)),
								false => context.on_change.call(None),
						};
				},
				extra_attributes: attributes,
				{children}
			}
		}
	}
}

#[derive(Props, PartialEq, Debug, Clone)]
pub struct RadioIndicatorProps {
	#[props(extends = GlobalAttributes)]
	pub attributes: Vec<Attribute>,
	#[props(default = None)]
	pub children: Option<Element>,
}

#[component]
pub fn RadioIndicator<T: Clone + PartialEq + Debug + Default + 'static>(props: RadioIndicatorProps) -> Element {
	let context = use_context::<RadioContext>();

	rsx! {
		span {
			aria_disabled: *context.disabled.read(),
			"data-state": if context.checked.read().unwrap_or_default() { "checked" } else { "unchecked" },
			"data-disabled": *context.disabled.read(),
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
					span {
						width: "8px",
						height: "8px",
						border: "1px solid",
						border_radius: "100%",
					}
				}
			}
		}
	}
}
