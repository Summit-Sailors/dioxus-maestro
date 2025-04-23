use {
	crate::{
		button::Button,
		presence::use_presence,
		shared::{EOrientation, UseControllableStateParams, use_arrow_key_navigation, use_controllable_state},
	},
	dioxus::prelude::*,
	std::rc::Rc,
};

#[derive(Clone, PartialEq, Debug, Copy)]
struct TabsContext {
	pub value: Memo<String>,
	pub set_value: Callback<String>,
	pub orientation: ReadOnlySignal<EOrientation>,
}

impl TabsContext {
	pub fn new(value: Memo<String>, set_value: Callback<String>, orientation: ReadOnlySignal<EOrientation>) -> Self {
		Self { value, set_value, orientation }
	}
}

#[derive(Props, Clone, PartialEq)]
pub struct TabsRootProps {
	#[props(optional, default = ReadOnlySignal::new(Signal::new(None)))]
	pub value: ReadOnlySignal<Option<String>>,
	#[props(optional, default = String::new())]
	pub default_value: String,
	#[props(optional)]
	pub on_value_change: Option<Callback<String>>,

	#[props(optional, default = ReadOnlySignal::new(Signal::new(EOrientation::Horizontal)))]
	pub orientation: ReadOnlySignal<EOrientation>,

	#[props(extends = div, extends = GlobalAttributes)]
	attributes: Vec<Attribute>,
	#[props(default = Vec::new())]
	pub extra_attributes: Vec<Attribute>,
	pub children: Element,
}

#[component]
pub fn TabsRoot(props: TabsRootProps) -> Element {
	let TabsRootProps { value, default_value, on_value_change, children, orientation, attributes, extra_attributes } = props;
	let is_controlled = use_hook(move || value().is_some());
	let (value, set_value) =
		use_controllable_state(UseControllableStateParams { is_controlled, prop: value, default_prop: default_value, on_change: on_value_change });

	use_context_provider::<TabsContext>(|| TabsContext::new(value, set_value, orientation));

	rsx! {
		div {
			role: "presentation",
			aria_orientation: orientation().to_string(),
			"data-orientation": orientation().to_string(),
			..attributes,
			..extra_attributes,
			{children}
		}
	}
}

#[derive(Props, Clone, PartialEq)]
pub struct TabsListProps {
	#[props(extends = div, extends = GlobalAttributes)]
	pub attributes: Vec<Attribute>,
	#[props(default = Vec::new())]
	pub extra_attributes: Vec<Attribute>,
	#[props(optional, default = None)]
	pub children: Element,
}

#[component]
pub fn TabsList(props: TabsListProps) -> Element {
	let context = use_context::<TabsContext>();

	let mut current_ref = use_signal(|| None::<Rc<MountedData>>);
	let handle_key_down = use_arrow_key_navigation(current_ref, Some(String::from("[role='tab'][data-focusable='true']")), *context.orientation.read());

	rsx! {
		div {
			role: "tablist",
			aria_orientation: context.orientation.read().to_string(),
			"data-orientation": context.orientation.read().to_string(),
			onmounted: move |event| {
					current_ref.set(Some(event.data()));
			},
			onkeydown: handle_key_down,
			..props.attributes,
			..props.extra_attributes,
			{props.children}
		}
	}
}

#[derive(Clone, PartialEq, Props)]
pub struct TabsTriggerProps {
	pub value: ReadOnlySignal<String>,
	#[props(optional, default = ReadOnlySignal::new(Signal::new(false)))]
	pub disabled: ReadOnlySignal<bool>,

	#[props(extends = GlobalAttributes, extends = button)]
	pub attributes: Vec<Attribute>,
	#[props(default = Vec::new())]
	pub extra_attributes: Vec<Attribute>,
	#[props(optional, default = None)]
	pub children: Element,
}

#[component]
pub fn TabsTrigger(props: TabsTriggerProps) -> Element {
	let TabsTriggerProps { value, disabled, attributes, extra_attributes, children } = props;

	let context = use_context::<TabsContext>();
	let is_active = use_memo(move || value() == context.value.read().clone());

	let mut attrs = attributes.clone();
	attrs.extend(extra_attributes);

	rsx! {
		Button {
			id: "{*value()}-trigger".to_string(),
			r#type: "button",
			role: "tab",
			pointer_events: if disabled() { "none" } else { "auto" },
			cursor: if disabled() { "" } else { "pointer" },
			tabindex: if disabled() { -1 } else { 0 },
			disabled: disabled(),
			aria_controls: "{value()}-content".to_string(),
			aria_selected: is_active(),
			aria_expanded: is_active(),
			"data-focusable": !disabled(),
			"data-state": if is_active() { "active" } else { "inactive" },
			aria_orientation: context.orientation.read().to_string(),
			"data-orientation": context.orientation.read().to_string(),
			onclick: move |_| { context.set_value.call(value()) },
			onfocus: move |_| { context.set_value.call(value()) },
			extra_attributes: attrs.clone(),
			{children}
		}
	}
}

#[derive(Clone, PartialEq, Props)]
pub struct TabsContentProps {
	pub value: ReadOnlySignal<String>,

	#[props(extends = div, extends = GlobalAttributes)]
	attributes: Vec<Attribute>,
	#[props(default = Vec::new())]
	pub extra_attributes: Vec<Attribute>,
	pub children: Element,
}

#[component]
pub fn TabsContent(props: TabsContentProps) -> Element {
	let TabsContentProps { value, attributes, extra_attributes, children } = props;

	let context = use_context::<TabsContext>();
	let is_active = use_memo(move || value() == context.value.read().clone());
	let mut current_ref = use_signal(|| None::<Rc<MountedData>>);

	let is_present = use_presence(is_active, current_ref);

	rsx! {
		div {
			id: "{value()}-content".to_string(),
			aria_labelledby: "{value()}-trigger".to_string(),
			role: "tabpannel",
			"data-state": if is_present() { "active" } else { "inactive" },
			aria_orientation: context.orientation.read().to_string(),
			"data-orientation": context.orientation.read().to_string(),
			hidden: !is_present(),
			onmounted: move |event: Event<MountedData>| current_ref.set(Some(event.data())),
			..attributes,
			..extra_attributes,
			{children}
		}
	}
}
