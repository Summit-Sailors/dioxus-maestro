use {
	crate::{
		button::Button,
		presence::Presence,
		shared::{EOrientation, UseControllableStateParams, use_arrow_key_navigation, use_controllable_state, use_dimensions},
	},
	dioxus::prelude::*,
	std::rc::Rc,
	uuid::Uuid,
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
	pub content_id: Uuid,
	pub trigger_id: Uuid,
}

impl AccordionItemContext {
	pub fn new(value: Memo<String>, open: Memo<bool>) -> Self {
		Self { value, open, content_id: Uuid::new_v4(), trigger_id: Uuid::new_v4() }
	}
}

#[derive(Clone, PartialEq, Debug, Copy)]
struct AccordionContext {
	pub value: Memo<Vec<String>>,
	pub set_value: Callback<Vec<String>>,
	pub collapsible: bool,
	pub variant: AccordionVariant,
	pub disabled: ReadOnlySignal<bool>,
}

impl AccordionContext {
	pub fn new(value: Memo<Vec<String>>, set_value: Callback<Vec<String>>, collapsible: bool, variant: AccordionVariant, disabled: ReadOnlySignal<bool>) -> Self {
		Self { value, set_value, collapsible, variant, disabled }
	}

	pub fn onopen(&self, value: String) {
		match self.variant {
			AccordionVariant::Single => {
				self.set_value.call(Vec::from([value.clone()]));
			},
			AccordionVariant::Multiple => {
				let mut new_value = self.value.peek().clone();
				if !new_value.contains(&value) {
					new_value.push(value);
					self.set_value.call(new_value);
				}
			},
		}
	}

	pub fn onclose(&mut self, value: String) {
		match self.variant {
			AccordionVariant::Single =>
				if self.collapsible {
					self.set_value.call(Vec::new());
				},
			AccordionVariant::Multiple => {
				let mut new_value = self.value.peek().clone();
				new_value.retain(|v| v != &value);
				self.set_value.call(new_value);
			},
		}
	}
}

#[derive(Props, Clone, PartialEq)]
pub struct AccordionRootProps {
	#[props(optional, default = ReadOnlySignal::new(Signal::new(None)))]
	pub value: ReadOnlySignal<Option<Vec<String>>>,
	#[props(optional, default = Vec::from([String::new()]))]
	pub default_value: Vec<String>,
	#[props(optional)]
	pub on_value_change: Option<Callback<Vec<String>>>,

	#[props(optional, default = ReadOnlySignal::new(Signal::new(EOrientation::Vertical)))]
	pub orientation: ReadOnlySignal<EOrientation>,
	#[props(optional, default = true)]
	pub collapsible: bool,
	#[props(optional, default = ReadOnlySignal::new(Signal::new(false)))]
	pub disabled: ReadOnlySignal<bool>,
	#[props(optional, default = AccordionVariant::Single)]
	variant: AccordionVariant,

	#[props(extends = ul, extends = GlobalAttributes)]
	attributes: Vec<Attribute>,
	pub children: Element,
}

#[component]
pub fn AccordionRoot(props: AccordionRootProps) -> Element {
	let AccordionRootProps { value, default_value, on_value_change, orientation, collapsible, disabled, variant, attributes, children } = props;
	let is_controlled = use_hook(move || value().is_some());
	let (value, set_value) =
		use_controllable_state(UseControllableStateParams { is_controlled, prop: value, default_prop: default_value, on_change: on_value_change });

	use_context_provider::<AccordionContext>(|| AccordionContext::new(value, set_value, collapsible, variant, disabled));

	let mut current_ref = use_signal(|| None::<Rc<MountedData>>);
	let handle_key_down = use_arrow_key_navigation(current_ref, Some(String::from("button[role='button']:not([tabindex='-1'])")), orientation());

	rsx! {
		ul {
			role: "accordion",
			aria_disabled: disabled().then_some(Some(true)),
			aria_orientation: orientation.read().to_string(),
			"data-disabled": disabled().then_some(Some(true)),
			"data-orientation": orientation.read().to_string(),
			"data-role": "accordion",
			onmounted: move |event| {
					current_ref.set(Some(event.data()));
			},
			onkeydown: handle_key_down,
			..attributes,
			{children}
		}
	}
}

#[derive(Props, Clone, PartialEq)]
pub struct AccordionItemProps {
	pub value: String,
	#[props(optional, default = ReadOnlySignal::new(Signal::new(false)))]
	pub disabled: ReadOnlySignal<bool>,

	#[props(extends = li, extends = GlobalAttributes)]
	pub attributes: Vec<Attribute>,
	#[props(optional, default = None)]
	pub children: Element,
}

#[component]
pub fn AccordionItem(props: AccordionItemProps) -> Element {
	let AccordionItemProps { value, disabled, attributes, children } = props;
	let accordion_context = use_context::<AccordionContext>();

	let cloned_value = value.clone();
	let open = use_memo(move || accordion_context.value.read().clone().contains(&cloned_value));
	let item_value = use_memo(move || value.clone());
	let accordion_item_context = use_context_provider::<AccordionItemContext>(|| AccordionItemContext::new(item_value, open));

	let is_disabled = use_memo(move || disabled() || *accordion_context.disabled.read());
	use_context_provider::<Memo<bool>>(|| is_disabled);

	rsx! {
		li {
			role: "presentation",
			aria_disabled: is_disabled().then_some(Some(true)),
			"data-state": if *accordion_item_context.open.read() { "open" } else { "closed" },
			"data-disabled": is_disabled().then_some(Some(true)),
			"data-role": "accordion-item",
			..attributes,
			{children}
		}
	}
}

#[derive(Clone, PartialEq, Props)]
pub struct AccordionTriggerProps {
	#[props(extends = GlobalAttributes, extends = button)]
	pub attributes: Vec<Attribute>,
	pub children: Element,
}

#[component]
pub fn AccordionTrigger(props: AccordionTriggerProps) -> Element {
	let mut accordion_context = use_context::<AccordionContext>();
	let accordion_item_context = use_context::<AccordionItemContext>();
	let is_disabled = use_context::<Memo<bool>>();

	rsx! {
		Button {
			id: accordion_item_context.trigger_id.to_string(),
			role: "button",
			r#type: "button",
			pointer_events: if is_disabled() { "none" } else { "auto" },
			cursor: if is_disabled() { "auto" } else { "pointer" },
			tabindex: if is_disabled() { -1 } else { 0 },
			disabled: is_disabled(),
			aria_controls: accordion_item_context.content_id.to_string(),
			aria_expanded: accordion_item_context.open.read().then_some(Some(true)),
			"data-state": if *accordion_item_context.open.read() { "open" } else { "closed" },
			"data-role": "accordion-trigger",
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
			extra_attributes: props.attributes.clone(),
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
	let mut current_ref = use_signal(|| None::<Rc<MountedData>>);
	let (width, height) = use_dimensions(current_ref, *accordion_item_context.open.peek());

	let mut attrs = props.attributes.clone();

	attrs.push(Attribute::new("--maestro-headless-accordion-height", if height() > 0.0 { Some(format!("{}px", height())) } else { None }, Some("style"), false));
	attrs.push(Attribute::new("--maestro-headless-accordion-width", if width() > 0.0 { Some(format!("{}px", width())) } else { None }, Some("style"), false));

	rsx! {
		Presence {
			present: *accordion_item_context.open.read(),
			node_ref: current_ref,
			div {
				id: accordion_item_context.content_id.to_string(),
				role: "region",
				aria_labelledby: accordion_item_context.trigger_id.to_string(),
				"data-state": if *accordion_item_context.open.read() { "open" } else { "closed" },
				"data-role": "accordion-content",
				aria_expanded: accordion_item_context.open.read().then_some(Some(true)),
				aria_hidden: !*accordion_item_context.open.read(),
				onmounted: move |event: Event<MountedData>| current_ref.set(Some(event.data())),
				..attrs,
				{props.children}
			}
		}
	}
}
