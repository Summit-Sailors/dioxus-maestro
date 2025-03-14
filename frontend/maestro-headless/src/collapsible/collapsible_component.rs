use {
	crate::{
		button::Button,
		hooks::{InteractionStateContext, UseControllableStateParams, use_controllable_state, use_interaction_state},
	},
	dioxus::prelude::*,
	uuid::Uuid,
};

#[derive(Clone, PartialEq, Debug, Copy)]
struct CollapsibleContext {
	pub open: Memo<Option<bool>>,
	pub set_open: Callback<Option<bool>>,
	pub content_id: Uuid,
}

impl CollapsibleContext {
	pub fn new(open: Memo<Option<bool>>, set_open: Callback<Option<bool>>) -> Self {
		Self { open, set_open, content_id: Uuid::new_v4() }
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

	use_context_provider::<CollapsibleContext>(|| CollapsibleContext::new(open, set_open));
	use_interaction_state(ReadOnlySignal::new(Signal::new(false)), disabled);

	rsx! {
		div {
			role: "accordion",
			aria_disabled: disabled(),
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
	let mut interaction_state = use_context::<InteractionStateContext>();

	let mut attributes = props.attributes.clone();
	attributes.push(Attribute::new("aria-disabled", *interaction_state.disabled.read(), None, false));
	attributes.push(Attribute::new("aria-expanded", *context.open.read(), None, false));
	attributes.push(Attribute::new("data-disabled", *interaction_state.disabled.read(), None, false));
	attributes.push(Attribute::new("data-state", if context.open.read().unwrap_or_default() { "open" } else { "closed" }, None, false));

	rsx! {
		Button {
			r#type: "button",
			style: if *interaction_state.disabled.read() { "pointer-events:none;" } else { "cursor:pointer;" },
			tabindex: if *interaction_state.disabled.read() { "-1" } else { "0" },
			disabled: *interaction_state.disabled.read(),
			aria_controls: context.content_id.to_string(),
			onclick: move |_| {
					let open = context.open.peek().unwrap_or_default();
					context.set_open.call(Some(!open));
			},
			onmousedown: move |event| {
					interaction_state.onmousedown();
					if let Some(handler) = props.onmousedown {
							handler.call(event);
					}
			},
			onkeydown: move |event| {
					interaction_state.onkeydown();
					if let Some(handler) = props.onkeydown {
							handler.call(event);
					}
			},
			onkeyup: move |event| {
					interaction_state.onkeyup();
					if let Some(handler) = props.onkeyup {
							handler.call(event);
					}
			},
			onmouseup: move |event| {
					interaction_state.onmouseup();
					if let Some(handler) = props.onmouseup {
							handler.call(event);
					}
			},
			onmouseenter: move |event| {
					interaction_state.onmouseenter();
					if let Some(handler) = props.onmouseenter {
							handler.call(event);
					}
			},
			onmouseleave: move |event| {
					interaction_state.onmouseleave();
					if let Some(handler) = props.onmouseleave {
							handler.call(event);
					}
			},
			onfocus: move |event| {
					interaction_state.onfocus();
					if let Some(handler) = props.onfocus {
							handler.call(event);
					}
			},
			onblur: move |event| {
					interaction_state.onblur();
					if let Some(handler) = props.onblur {
							handler.call(event);
					}
			},
			additional_attributes: attributes.clone(),
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
	rsx! {
		div {
			id: context.content_id.to_string(),
			role: "region",
			aria_expanded: context.open.read().unwrap_or_default(),
			"data-state": if context.open.read().unwrap_or_default() { "open" } else { "closed" },
			aria_hidden: !context.open.read().unwrap_or_default(),
			hidden: !context.open.read().unwrap_or_default(),
			..props.attributes,
			if context.open.read().unwrap_or_default() {
				{props.children}
			}
		}
	}
}
