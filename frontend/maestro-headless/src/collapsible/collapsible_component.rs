use {
	crate::{
		button::Button,
		hooks::{UseControllableStateParams, use_controllable_state},
		presence::use_presence,
	},
	dioxus::prelude::*,
	std::rc::Rc,
	uuid::Uuid,
};

#[derive(Clone, PartialEq, Debug, Copy)]
struct CollapsibleContext {
	pub open: Memo<Option<bool>>,
	pub set_open: Callback<Option<bool>>,
	pub content_id: Uuid,
	pub trigger_id: Uuid,
	pub disabled: ReadOnlySignal<bool>,
}

impl CollapsibleContext {
	pub fn new(open: Memo<Option<bool>>, set_open: Callback<Option<bool>>, disabled: ReadOnlySignal<bool>) -> Self {
		Self { open, set_open, content_id: Uuid::new_v4(), trigger_id: Uuid::new_v4(), disabled }
	}
}

#[derive(Props, Clone, PartialEq)]
pub struct CollapsibleProps {
	#[props(optional, default = ReadOnlySignal::new(Signal::new(None)))]
	pub open: ReadOnlySignal<Option<bool>>,
	#[props(optional, default = false)]
	pub default_open: bool,
	#[props(optional)]
	pub on_open_change: Option<Callback<Option<bool>>>,

	#[props(optional, default = ReadOnlySignal::new(Signal::new(false)))]
	pub disabled: ReadOnlySignal<bool>,

	#[props(extends = div, extends = GlobalAttributes)]
	attributes: Vec<Attribute>,
	#[props(optional, default = None)]
	pub children: Element,
}

#[component]
pub fn Collapsible(props: CollapsibleProps) -> Element {
	let CollapsibleProps { open, default_open, on_open_change, disabled, children, attributes, .. } = props;
	let is_controlled = use_hook(move || open().is_some());
	let (open, set_open) =
		use_controllable_state(UseControllableStateParams { is_controlled, prop: open, default_prop: default_open, on_change: on_open_change });

	use_context_provider::<CollapsibleContext>(|| CollapsibleContext::new(open, set_open, disabled));

	rsx! {
		div {
			role: "accordion",
			aria_disabled: disabled(),
			"data_disabled": disabled(),
			"data-state": if open().unwrap_or_default() { "open" } else { "closed" },
			..attributes,
			{children}
		}
	}
}

#[derive(Clone, PartialEq, Props)]
pub struct CollapsibleTriggerProps {
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
	pub children: Element,
}

#[component]
pub fn CollapsibleTrigger(props: CollapsibleTriggerProps) -> Element {
	let context = use_context::<CollapsibleContext>();

	rsx! {
		Button {
			r#type: "button",
			pointer_events: if *context.disabled.read() { "none" } else { "auto" },
			cursor: if *context.disabled.read() { "" } else { "pointer" },
			tabindex: if *context.disabled.read() { "-1" } else { "0" },
			disabled: *context.disabled.read(),
			aria_controls: context.content_id.to_string(),
			aria_disabled: *context.disabled.read(),
			aria_expanded: context.open.read().unwrap_or_default(),
			"data-state": if context.open.read().unwrap_or_default() { "open" } else { "closed" },
			"data-disabled": *context.disabled.read(),
			onclick: move |_| {
					let open = context.open.peek().unwrap_or_default();
					context.set_open.call(Some(!open));
			},
			onmousedown: props.onmousedown,
			onkeydown: props.onkeydown,
			onkeyup: props.onkeyup,
			onmouseup: props.onmouseup,
			onmouseenter: props.onmouseenter,
			onmouseleave: props.onmouseleave,
			onfocus: props.onfocus,
			onblur: props.onblur,
			extra_attributes: props.attributes.clone(),
			{props.children}
		}
	}
}

#[derive(Clone, PartialEq, Props)]
pub struct CollapsibleContentProps {
	#[props(extends = div, extends = GlobalAttributes)]
	attributes: Vec<Attribute>,
	pub children: Element,
}

#[component]
pub fn CollapsibleContent(props: CollapsibleContentProps) -> Element {
	let context = use_context::<CollapsibleContext>();

	let is_active = use_memo(move || context.open.read().unwrap_or_default());
	let mut current_ref = use_signal(|| None::<Rc<MountedData>>);

	let is_present = use_presence(is_active, current_ref);

	rsx! {
		div {
			id: context.content_id.to_string(),
			role: "region",
			aria_expanded: is_present(),
			aria_hidden: !is_present(),
			aria_labelledby: context.trigger_id.to_string(),
			"data-hidden": !is_present(),
			"data-state": if is_present() { "open" } else { "closed" },
			hidden: !is_present(),
			onmounted: move |event: Event<MountedData>| current_ref.set(Some(event.data())),
			..props.attributes,
			{props.children}
		}
	}
}
