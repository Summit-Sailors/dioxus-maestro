use {
	crate::{
		button::Button,
		hooks::{InteractionStateContext, UseControllableStateParams, use_arrow_key_navigation, use_controllable_state, use_interaction_state},
	},
	dioxus::prelude::*,
	std::rc::Rc,
};

#[derive(Clone, PartialEq, Debug, Copy)]
pub enum AccordionVariant {
	Single,
	Multiple,
}

#[derive(Clone, PartialEq, Debug, Copy)]
pub struct AccordionItemContext {
	pub value: Memo<String>,
	pub open: Memo<bool>,
}

impl AccordionItemContext {
	pub fn new(value: Memo<String>, open: Memo<bool>) -> Self {
		Self { value, open }
	}
}

#[derive(Clone, PartialEq, Debug, Copy)]
struct AccordionContext {
	pub value: Memo<Option<Vec<String>>>,
	pub set_value: Callback<Option<Vec<String>>>,
	pub collapsible: bool,
	pub variant: AccordionVariant,
}

impl AccordionContext {
	pub fn new(value: Memo<Option<Vec<String>>>, set_value: Callback<Option<Vec<String>>>, collapsible: bool, variant: AccordionVariant) -> Self {
		Self { value, set_value, collapsible, variant }
	}

	pub fn onopen(&self, value: String) {
		match self.variant {
			AccordionVariant::Single => {
				self.set_value.call(Some(Vec::from([value.clone()])));
			},
			AccordionVariant::Multiple => {
				let mut new_value = self.value.peek().clone().unwrap_or_default();
				if !new_value.contains(&value) {
					new_value.push(value);
					self.set_value.call(Some(new_value));
				}
			},
		}
	}

	pub fn onclose(&mut self, value: String) {
		match self.variant {
			AccordionVariant::Single =>
				if self.collapsible {
					self.set_value.call(None);
				},
			AccordionVariant::Multiple => {
				let mut new_value = self.value.peek().clone().unwrap_or_default();
				new_value.retain(|v| v != &value);
				self.set_value.call(Some(new_value));
			},
		}
	}
}

#[derive(Props, Clone, PartialEq)]
pub struct AccordionProps {
	#[props(optional, default = ReadOnlySignal::new(Signal::new(None)))]
	pub value: ReadOnlySignal<Option<Vec<String>>>,
	#[props(optional, default = Vec::from([String::new()]))]
	pub default_value: Vec<String>,
	#[props(optional)]
	pub on_value_change: Option<Callback<Option<Vec<String>>>>,
	#[props(optional, default = true)]
	pub collapsible: bool,
	#[props(optional, default = ReadOnlySignal::new(Signal::new(false)))]
	pub disabled: ReadOnlySignal<bool>,
	#[props(extends = ul, extends = GlobalAttributes)]
	attributes: Vec<Attribute>,
	#[props(optional, default = AccordionVariant::Single)]
	variant: AccordionVariant,
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
	#[props(optional, default = None)]
	pub children: Element,
}

#[component]
pub fn Accordion(props: AccordionProps) -> Element {
	let AccordionProps { value, default_value, on_value_change, collapsible, disabled, variant, children, attributes, .. } = props;
	let is_controlled = use_hook(move || value().is_some());
	let (value, set_value) =
		use_controllable_state(UseControllableStateParams { is_controlled, prop: value, default_prop: default_value, on_change: on_value_change });

	use_context_provider::<AccordionContext>(|| AccordionContext::new(value, set_value, collapsible, variant));
	let mut interaction_state = use_interaction_state(ReadOnlySignal::new(Signal::new(false)), disabled);
	let mut current_ref = use_signal(|| None::<Rc<MountedData>>);
	let handle_key_down = use_arrow_key_navigation(current_ref, Some(String::from("li[role='presentation']:not([tabindex='-1'])")));

	rsx! {
		ul {
			role: "accordion",
			onmounted: move |event| {
					interaction_state.self_ref.set(Some(event.clone()));
					current_ref.set(Some(event.data()));
			},
			onmousedown: move |event| {
					interaction_state.onmousedown();
					if let Some(handler) = props.onmousedown {
							handler.call(event);
					}
			},
			onkeydown: move |event| {
					interaction_state.onkeydown();
					handle_key_down(event.clone());
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
			aria_disabled: "{!interaction_state.is_allowed()}",
			"data-disabled": *interaction_state.disabled.read(),
			"data-pressed": *interaction_state.is_pressed.read(),
			"data-hovered": *interaction_state.is_hovered.read(),
			"data-focused": *interaction_state.is_focused.read(),
			"data-focuse-visible": *interaction_state.is_focused.read(),
			..attributes,
			{children}
		}
	}
}

#[derive(Props, Clone, PartialEq)]
pub struct AccordionItemProps {
	#[props(optional, default = ReadOnlySignal::new(Signal::new(false)))]
	pub disabled: ReadOnlySignal<bool>,
	pub value: String,
	#[props(extends = li, extends = GlobalAttributes)]
	pub attributes: Vec<Attribute>,
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
	#[props(optional, default = None)]
	pub children: Element,
}

#[component]
pub fn AccordionItem(props: AccordionItemProps) -> Element {
	let accordion_context = use_context::<AccordionContext>();
	let accordion_interaction_state = use_context::<InteractionStateContext>();
	let mut interaction_state = use_interaction_state(ReadOnlySignal::new(Signal::new(false)), props.disabled);
	let cloned_value = props.value.clone();
	let open = use_memo(move || accordion_context.value.read().clone().unwrap_or_default().contains(&cloned_value));
	let item_value = use_memo(move || props.value.clone());
	let accordion_item_context = use_context_provider::<AccordionItemContext>(|| AccordionItemContext::new(item_value, open));

	let is_disabled = use_memo(move || *accordion_interaction_state.disabled.read() || *interaction_state.disabled.read());
	use_context_provider::<Memo<bool>>(|| is_disabled);

	rsx! {
		li {
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
			"data-pressed": *interaction_state.is_pressed.read(),
			"data-hovered": *interaction_state.is_hovered.read(),
			"data-focused": *interaction_state.is_focused.read(),
			"data-focuse-visible": *interaction_state.is_focused.read(),
			"data-state": if *accordion_item_context.open.read() { "open" } else { "closed" },
			"data-disabled": is_disabled(),
			aria_disabled: is_disabled(),
			role: "presentation",
			tabindex: if is_disabled() { -1 } else { 0 },
			..props.attributes,
			{props.children}
		}
	}
}

#[derive(Clone, PartialEq, Props)]
pub struct AccordionTriggerProps {
	pub children: Element,
	#[props(extends = GlobalAttributes, extends = button)]
	pub attributes: Vec<Attribute>,
}

#[component]
pub fn AccordionTrigger(props: AccordionTriggerProps) -> Element {
	let mut accordion_context = use_context::<AccordionContext>();
	let accordion_item_context = use_context::<AccordionItemContext>();
	let is_disabled = use_context::<Memo<bool>>();

	let mut attributes = props.attributes.clone();
	attributes.push(Attribute::new("aria-disabled", is_disabled(), None, false));
	attributes.push(Attribute::new("aria-expanded", *accordion_item_context.open.read(), None, false));
	attributes.push(Attribute::new("data-disabled", *accordion_item_context.open.read(), None, false));
	attributes.push(Attribute::new("data-state", if *accordion_item_context.open.read() { "open" } else { "closed" }, None, false));

	rsx! {
		Button {
			r#type: "button",
			style: if is_disabled() { "pointer-events:none;" } else { "cursor:pointer;" },
			tabindex: -1,
			onclick: move |_| {
					if is_disabled() {
							return;
					}
					let value = accordion_item_context.value.peek().clone();
					let open = *accordion_item_context.open.peek();
					match accordion_context.variant {
							AccordionVariant::Single => {
									if accordion_context.collapsible {
											if open {
													accordion_context.onclose(value);
											} else {
													accordion_context.onopen(value);
											}
									} else if !open {
											accordion_context.onopen(value);
									}
							}
							AccordionVariant::Multiple => {
									if open {
											accordion_context.onclose(value);
									} else {
											accordion_context.onopen(value);
									}
							}
					}
			},
			disabled: is_disabled(),
			additional_attributes: attributes.clone(),
			{props.children}
		}
	}
}

#[derive(Clone, PartialEq, Props)]
pub struct AccordionHeaderProps {
	#[props(extends = div, extends = GlobalAttributes)]
	attributes: Vec<Attribute>,
	pub children: Element,
}

#[component]
pub fn AccordionHeader(props: AccordionHeaderProps) -> Element {
	let accordion_item_context = use_context::<AccordionItemContext>();
	rsx! {
		h3 {
			"data-state": if *accordion_item_context.open.read() { "open" } else { "closed" },
			..props.attributes,
			{props.children}
		}
	}
}

#[derive(Clone, PartialEq, Props)]
pub struct AccordionContentProps {
	#[props(extends = div, extends = GlobalAttributes)]
	attributes: Vec<Attribute>,
	pub children: Element,
}

#[component]
pub fn AccordionContent(props: AccordionContentProps) -> Element {
	let accordion_item_context = use_context::<AccordionItemContext>();
	rsx! {
		div {
			role: "region",
			"data-state": if *accordion_item_context.open.read() { "open" } else { "closed" },
			..props.attributes,
			{props.children}
		}
	}
}
