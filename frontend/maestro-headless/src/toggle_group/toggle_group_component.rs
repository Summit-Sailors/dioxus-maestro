use {
	crate::{
		hooks::{InteractionStateContext, UseControllableStateParams, use_arrow_key_navigation, use_controllable_state, use_interaction_state},
		toggle::Toggle,
		utils::EGroupOrientation,
	},
	dioxus::prelude::*,
	std::{fmt::Debug, rc::Rc},
};

#[derive(Clone, PartialEq, Debug, Copy)]
pub struct ToggleGroupContext {
	pub value: Memo<Option<String>>,
	pub set_value: Callback<Option<String>>,
	pub orientation: ReadOnlySignal<EGroupOrientation>,
}

impl ToggleGroupContext {
	pub fn new(value: Memo<Option<String>>, set_value: Callback<Option<String>>, orientation: ReadOnlySignal<EGroupOrientation>) -> Self {
		Self { value, set_value, orientation }
	}

	pub fn onselect(&self, value: String) {
		self.set_value.call(Some(value));
	}

	pub fn ondeselect(&self) {
		self.set_value.call(None);
	}
}

#[derive(Props, Clone, PartialEq)]
pub struct ToggleGroupProps {
	#[props(optional, default = ReadOnlySignal::new(Signal::new(None)))]
	pub value: ReadOnlySignal<Option<String>>,
	#[props(optional, default = None)]
	default_value: Option<String>,
	#[props(optional)]
	pub on_value_chenge: Option<Callback<Option<String>>>,

	#[props(optional, default = ReadOnlySignal::new(Signal::new(false)))]
	pub disabled: ReadOnlySignal<bool>,
	#[props(optional, default = ReadOnlySignal::new(Signal::new(EGroupOrientation::Horizontal)))]
	pub orientation: ReadOnlySignal<EGroupOrientation>,

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
	pub children: Element,
}

#[component]
pub fn ToggleGroup(props: ToggleGroupProps) -> Element {
	let ToggleGroupProps { value, default_value, on_value_chenge, disabled, orientation, children, attributes, .. } = props;
	let is_controlled = use_hook(move || value().is_some());
	let (value, set_value) = use_controllable_state(UseControllableStateParams {
		is_controlled,
		prop: value,
		default_prop: default_value.unwrap_or_default(),
		on_change: on_value_chenge,
	});

	use_context_provider::<ToggleGroupContext>(|| ToggleGroupContext::new(value, set_value, orientation));

	let mut container_ref = use_signal(|| None::<Rc<MountedData>>);

	let on_key_down = use_arrow_key_navigation(container_ref, Some("[role='radio']:not([tabindex='-1'])".to_string()), orientation());
	let mut interaction_state = use_interaction_state(ReadOnlySignal::new(Signal::new(false)), disabled);

	rsx! {
		div {
			role: "group",
			aria_disabled: *interaction_state.disabled.read(),
			"data-pressed": *interaction_state.is_pressed.read(),
			"data-hovered": *interaction_state.is_hovered.read(),
			"data-focused": *interaction_state.is_focused.read(),
			"data-focuse-visible": *interaction_state.is_focused.read(),
			aria_orientation: &*orientation.read().to_string(),

			onmousedown: move |event| {
					interaction_state.onmousedown();
					if let Some(handler) = props.onmousedown {
							handler.call(event);
					}
			},
			onkeydown: move |event| {
					interaction_state.onkeydown();
					on_key_down(event.clone());
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
			onmounted: move |event| container_ref.set(Some(event.data())),
			..attributes,
			{children}
		}
	}
}

#[derive(Props, Clone, PartialEq)]
pub struct ToggleGroupItemProps {
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

	#[props(extends = button, extends = GlobalAttributes)]
	pub attributes: Vec<Attribute>,
	pub children: Element,
}

#[component]
pub fn ToggleGroupItem(props: ToggleGroupItemProps) -> Element {
	let context = use_context::<ToggleGroupContext>();
	let pressed = use_memo(move || *context.value.read().clone().unwrap_or_default() == *props.value.read());
	let interaction_state = use_context::<InteractionStateContext>();

	let is_disabled = use_memo(move || *interaction_state.disabled.read() || *props.disabled.read());
	use_context_provider::<Memo<bool>>(|| is_disabled);

	rsx! {
		Toggle {
			value: context.value.read().clone().unwrap_or("on".into()),
			pressed: pressed(),
			role: "radio",
			tabindex: if is_disabled() { -1 } else { 0 },
			disabled: is_disabled(),
			aria_orientation: &*context.orientation.read().to_string(),
			on_toggle_change: move |pressed: Option<bool>| {
					if pressed.is_some() {
							context.onselect(props.value.read().clone());
					} else {
							context.ondeselect();
					}
			},
			onblur: props.onblur,
			onfocus: props.onfocus,
			onkeydown: props.onkeydown,
			onkeyup: props.onkeyup,
			onmousedown: props.onmousedown,
			onmouseenter: props.onmouseenter,
			onmouseleave: props.onmouseleave,
			onmouseup: props.onmouseup,
			additional_attributes: props.attributes.clone(),
			{props.children}
		}
	}
}
