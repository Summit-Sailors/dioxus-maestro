use {
	crate::{
		button::Button,
		presence::Presence,
		shared::{UseControllableStateParams, use_controllable_state, use_dimensions, use_ref_provider},
	},
	dioxus::prelude::*,
	uuid::Uuid,
};

#[derive(Debug, Clone, Copy, PartialEq)]
struct CollapsibleContext {
	pub open: Memo<bool>,
	pub set_open: Callback<bool>,
	pub content_id: Uuid,
	pub trigger_id: Uuid,
	pub disabled: ReadOnlySignal<bool>,
}

impl CollapsibleContext {
	pub fn new(open: Memo<bool>, set_open: Callback<bool>, disabled: ReadOnlySignal<bool>) -> Self {
		Self { open, set_open, content_id: Uuid::new_v4(), trigger_id: Uuid::new_v4(), disabled }
	}
}

#[derive(Clone, PartialEq, Props)]
pub struct CollapsibleRootProps {
	#[props(optional, default = ReadOnlySignal::new(Signal::new(None)))]
	pub open: ReadOnlySignal<Option<bool>>,
	#[props(optional, default = false)]
	pub default_open: bool,
	#[props(optional)]
	pub on_open_change: Option<Callback<bool>>,

	#[props(optional, default = ReadOnlySignal::new(Signal::new(false)))]
	pub disabled: ReadOnlySignal<bool>,

	#[props(extends = div, extends = GlobalAttributes)]
	attributes: Vec<Attribute>,
	#[props(default = Vec::new())]
	pub extra_attributes: Vec<Attribute>,
	#[props(optional, default = None)]
	pub children: Element,
}

#[component]
pub fn CollapsibleRoot(props: CollapsibleRootProps) -> Element {
	let CollapsibleRootProps { open, default_open, on_open_change, disabled, children, attributes, extra_attributes } = props;
	let is_controlled = use_hook(move || open().is_some());
	let (open, set_open) =
		use_controllable_state(UseControllableStateParams { is_controlled, prop: open, default_prop: default_open, on_change: on_open_change });

	use_context_provider::<CollapsibleContext>(|| CollapsibleContext::new(open, set_open, disabled));

	rsx! {
		div {
			role: "region",
			aria_disabled: disabled().then_some(Some(true)),
			"data_disabled": disabled().then_some(Some(true)),
			"data-state": if open() { "open" } else { "closed" },
			..attributes,
			..extra_attributes,
			{children}
		}
	}
}

#[derive(Clone, PartialEq, Props)]
pub struct CollapsibleTriggerProps {
	#[props(extends = GlobalAttributes, extends = button)]
	pub attributes: Vec<Attribute>,
	#[props(default = Vec::new())]
	pub extra_attributes: Vec<Attribute>,
	pub children: Element,
}

#[component]
pub fn CollapsibleTrigger(props: CollapsibleTriggerProps) -> Element {
	let context = use_context::<CollapsibleContext>();

	let mut attrs = props.attributes.clone();
	attrs.extend(props.extra_attributes.clone());

	rsx! {
		Button {
			r#type: "button",
			pointer_events: context.disabled.read().then_some(Some("none")),
			cursor: if *context.disabled.read() { "" } else { "pointer" },
			tabindex: if *context.disabled.read() { "-1" } else { "0" },
			disabled: *context.disabled.read(),
			aria_controls: context.content_id.to_string(),
			aria_expanded: context.open.read().then_some(Some(true)),
			"data-state": if *context.open.read() { "open" } else { "closed" },
			onclick: move |_| {
					let open = *context.open.peek();
					context.set_open.call(!open);
			},
			extra_attributes: attrs.clone(),
			{props.children}
		}
	}
}

#[derive(Clone, PartialEq, Props)]
pub struct CollapsibleContentProps {
	#[props(extends = div, extends = GlobalAttributes)]
	attributes: Vec<Attribute>,
	#[props(default = Vec::new())]
	pub extra_attributes: Vec<Attribute>,
	pub children: Element,
}

#[component]
pub fn CollapsibleContent(props: CollapsibleContentProps) -> Element {
	let CollapsibleContentProps { attributes, extra_attributes, children } = props;
	let context = use_context::<CollapsibleContext>();
	let mut current_ref = use_ref_provider();
	let (width, height) = use_dimensions(current_ref, *context.open.peek());

	let mut attrs = attributes.clone();
	attrs.extend(extra_attributes);

	attrs.push(Attribute::new(
		"--maestro-headless-collapsible-height",
		if height() > 0.0 { Some(format!("{}px", height())) } else { None },
		Some("style"),
		false,
	));
	attrs.push(Attribute::new("--maestro-headless-collapsible-width", if width() > 0.0 { Some(format!("{}px", width())) } else { None }, Some("style"), false));

	rsx! {
		Presence { present: *context.open.read(),
			div {
				id: context.content_id.to_string(),
				role: "region",
				aria_expanded: context.open.read().then_some(Some(true)),
				aria_hidden: !*context.open.read(),
				aria_labelledby: context.trigger_id.to_string(),
				"data-state": if *context.open.read() { "open" } else { "closed" },
				"data-disabled": context.disabled.read().then_some(true),
				onmounted: move |event: Event<MountedData>| current_ref.set(Some(event.data())),
				..attrs,
				{children}
			}
		}
	}
}
