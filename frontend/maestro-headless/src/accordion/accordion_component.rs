use {
	crate::{button::Button, hooks::use_arrow_key_navigation},
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
	pub value: Signal<String>,
	pub disabled: Signal<bool>,
	pub open: Signal<bool>,
}

impl AccordionItemContext {
	pub fn new(value: Signal<String>, open: Signal<bool>, disabled: Signal<bool>) -> Self {
		Self { value, open, disabled }
	}
}

#[derive(Clone, PartialEq, Debug, Copy)]
struct AccordionContext {
	pub value: Signal<Vec<String>>,
	pub on_value_change: Option<Callback<Vec<String>>>,
	pub collapsible: bool,
	pub disabled: Signal<bool>,
	pub variant: AccordionVariant,
}

impl AccordionContext {
	pub fn new(
		value: Signal<Vec<String>>,
		on_value_change: Option<Callback<Vec<String>>>,
		collapsible: bool,
		disabled: Signal<bool>,
		variant: AccordionVariant,
	) -> Self {
		Self { value, on_value_change, collapsible, disabled, variant }
	}

	pub fn onopen(&mut self, value: String) {
		match self.variant {
			AccordionVariant::Single => {
				self.value.set(Vec::from([value.clone()]));

				if let Some(callback) = self.on_value_change {
					callback.call(self.value.peek().clone());
				}
			},
			AccordionVariant::Multiple =>
				if !self.value.peek().contains(&value) {
					self.value.write().push(value);
					if let Some(callback) = self.on_value_change {
						callback.call(self.value.peek().clone());
					}
				},
		}
	}

	pub fn onclose(&mut self, value: String) {
		match self.variant {
			AccordionVariant::Single =>
				if self.collapsible {
					self.value.set(Vec::new());
					if let Some(callback) = self.on_value_change {
						callback.call(self.value.peek().clone());
					}
				},
			AccordionVariant::Multiple => {
				self.value.write().retain(|v| v != &value);

				if let Some(callback) = self.on_value_change {
					callback.call(self.value.peek().clone());
				}
			},
		}
	}
}

#[derive(Props, Clone, PartialEq)]
pub struct AccordionProps {
	#[props(optional, default = Signal::new(Vec::from([String::new()])))]
	pub value: Signal<Vec<String>>,
	#[props(optional)]
	pub on_value_change: Option<Callback<Vec<String>>>,
	#[props(optional, default = true)]
	pub collapsible: bool,
	#[props(optional, default = Signal::new(false))]
	pub disabled: Signal<bool>,
	#[props(extends = ul, extends = GlobalAttributes)]
	attributes: Vec<Attribute>,
	#[props(optional, default = AccordionVariant::Single)]
	variant: AccordionVariant,
	pub children: Element,
}

#[component]
pub fn Accordion(props: AccordionProps) -> Element {
	let AccordionProps { value, on_value_change, collapsible, disabled, variant, children, attributes } = props;

	use_context_provider::<AccordionContext>(|| AccordionContext::new(value, on_value_change, collapsible, disabled, variant));
	let mut current_ref = use_signal(|| None::<Rc<MountedData>>);
	let handle_key_down = use_arrow_key_navigation(current_ref, Some(String::from("li[role='presentation']:not([tabindex='-1'])")));

	rsx! {
		ul {
			role: "accordion",
			onmounted: move |event| current_ref.set(Some(event.data())),
			onkeydown: handle_key_down,
			..attributes,
			{children}
		}
	}
}

#[derive(Props, Clone, PartialEq)]
pub struct AccordionItemProps {
	#[props(optional, default = Signal::new(false))]
	pub disabled: Signal<bool>,
	pub value: String,
	#[props(extends = li, extends = GlobalAttributes)]
	pub attributes: Vec<Attribute>,
	pub children: Element,
}

#[component]
pub fn AccordionItem(props: AccordionItemProps) -> Element {
	let accordion_context = use_context::<AccordionContext>();
	let mut accordion_item_context = use_context_provider::<AccordionItemContext>(|| {
		AccordionItemContext::new(Signal::new(props.value.clone()), Signal::new(accordion_context.value.peek().contains(&props.value.clone())), props.disabled)
	});

	let is_disabled = *accordion_context.disabled.read() || *accordion_item_context.disabled.read();

	use_effect(move || {
		if !accordion_context.value.read().contains(&*accordion_item_context.value.peek()) {
			accordion_item_context.open.set(false);
		}
	});

	rsx! {
		li {
			"data-state": if *accordion_item_context.open.read() { "open" } else { "closed" },
			"data-disabled": is_disabled,
			aria_disabled: is_disabled,
			role: "presentation",
			tabindex: if is_disabled { -1 } else { 0 },
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
	let mut accordion_item_context = use_context::<AccordionItemContext>();
	let is_disabled = *accordion_context.disabled.read() || *accordion_item_context.disabled.read();

	let mut attributes = props.attributes.clone();
	attributes.push(Attribute::new("aria-disabled", is_disabled, None, false));
	attributes.push(Attribute::new("aria-expanded", *accordion_item_context.open.read(), None, false));
	attributes.push(Attribute::new("data-disabled", *accordion_item_context.open.read(), None, false));
	attributes.push(Attribute::new("data-state", if *accordion_item_context.open.read() { "open" } else { "closed" }, None, false));

	rsx! {
		Button {
			r#type: "button",
			style: if is_disabled { "pointer-events:none;" } else { "cursor:pointer;" },
			tabindex: -1,
			onclick: move |_| {
					if is_disabled {
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
											accordion_item_context.open.toggle();
									} else if !open {
											accordion_context.onopen(value);
											accordion_item_context.open.toggle();
									}
							}
							AccordionVariant::Multiple => {
									if open {
											accordion_context.onclose(value);
									} else {
											accordion_context.onopen(value);
									}
									accordion_item_context.open.toggle();
							}
					}
			},
			disabled: Signal::new(is_disabled),
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
