use {
	crate::{
		button::Button,
		hooks::{UseControllableStateParams, use_arrow_key_navigation, use_controllable_state},
		presence::use_presence,
		utils::EOrientation,
	},
	dioxus::prelude::*,
	std::rc::Rc,
};

#[derive(Clone, PartialEq, Debug, Copy)]
struct TabsContext {
	pub value: Memo<Option<String>>,
	pub set_value: Callback<Option<String>>,
	pub orientation: ReadOnlySignal<EOrientation>,
}

impl TabsContext {
	pub fn new(value: Memo<Option<String>>, set_value: Callback<Option<String>>, orientation: ReadOnlySignal<EOrientation>) -> Self {
		Self { value, set_value, orientation }
	}
}

#[derive(Props, Clone, PartialEq)]
pub struct TabsProps {
	#[props(optional, default = ReadOnlySignal::new(Signal::new(None)))]
	pub value: ReadOnlySignal<Option<String>>,
	#[props(optional, default = String::new())]
	pub default_value: String,
	#[props(optional)]
	pub on_value_change: Option<Callback<Option<String>>>,

	#[props(optional, default = ReadOnlySignal::new(Signal::new(EOrientation::Horizontal)))]
	pub orientation: ReadOnlySignal<EOrientation>,

	#[props(extends = div, extends = GlobalAttributes)]
	attributes: Vec<Attribute>,
	pub children: Element,
}

#[component]
pub fn Tabs(props: TabsProps) -> Element {
	let TabsProps { value, default_value, on_value_change, children, orientation, attributes, .. } = props;
	let is_controlled = use_hook(move || value().is_some());
	let (value, set_value) =
		use_controllable_state(UseControllableStateParams { is_controlled, prop: value, default_prop: default_value, on_change: on_value_change });

	use_context_provider::<TabsContext>(|| TabsContext::new(value, set_value, orientation));

	rsx! {
		div {
			role: "presentation",
			aria_orientation: orientation().to_string(),
			..attributes,
			{children}
		}
	}
}

#[derive(Props, Clone, PartialEq)]
pub struct TabsListProps {
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
	#[props(extends = div, extends = GlobalAttributes)]
	pub attributes: Vec<Attribute>,
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
			tabindex: 0,
			aria_orientation: context.orientation.read().to_string(),
			onmounted: move |event| {
					current_ref.set(Some(event.data()));
			},
			onkeydown: handle_key_down,
			..props.attributes,
			{props.children}
		}
	}
}

#[derive(Clone, PartialEq, Props)]
pub struct TabsTriggerProps {
	pub value: ReadOnlySignal<String>,
	#[props(optional, default = ReadOnlySignal::new(Signal::new(false)))]
	pub disabled: ReadOnlySignal<bool>,

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
	#[props(optional, default = None)]
	pub children: Element,
}

#[component]
pub fn TabsTrigger(props: TabsTriggerProps) -> Element {
	let context = use_context::<TabsContext>();
	let is_active = use_memo(move || *props.value.read() == context.value.read().clone().unwrap_or_default());

	rsx! {
		Button {
			id: "{*props.value.peek().clone()}-trigger".to_string(),
			r#type: "button",
			role: "tab",
			pointer_events: if *props.disabled.read() { "none" } else { "auto" },
			cursor: if *props.disabled.read() { "" } else { "pointer" },
			tabindex: if *props.disabled.read() || !is_active() { -1 } else { 0 },
			disabled: *props.disabled.read(),
			aria_controls: "{*props.value.peek().clone()}-content".to_string(),
			aria_disabled: *props.disabled.read(),
			aria_selected: is_active(),
			"data-focusable": !*props.disabled.read(),
			"data-disabled": *props.disabled.read(),
			"data-state": if is_active() { "active" } else { "inactive" },
			onclick: move |_| { context.set_value.call(Some(props.value.peek().clone())) },
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
pub struct TabsContentProps {
	pub value: ReadOnlySignal<String>,

	#[props(extends = div, extends = GlobalAttributes)]
	attributes: Vec<Attribute>,
	pub children: Element,
}

#[component]
pub fn TabsContent(props: TabsContentProps) -> Element {
	let context = use_context::<TabsContext>();
	let is_active = use_memo(move || *props.value.read() == context.value.read().clone().unwrap_or_default());
	let mut current_ref = use_signal(|| None::<Rc<MountedData>>);

	let is_present = use_presence(is_active, current_ref);

	rsx! {
		div {
			id: "{*props.value.peek().clone()}-content".to_string(),
			aria_labelledby: "{*props.value.peek().clone()}-trigger".to_string(),
			role: "tabpannel",
			"data-state": if is_present() { "active" } else { "inactive" },
			hidden: !is_present(),
			onmounted: move |event: Event<MountedData>| current_ref.set(Some(event.data())),
			..props.attributes,
			{props.children}
		}
	}
}
