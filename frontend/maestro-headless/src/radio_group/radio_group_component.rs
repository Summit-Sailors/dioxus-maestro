use {
	crate::{
		hooks::{UseControllableStateParams, use_arrow_key_navigation, use_controllable_state, use_interaction_state},
		radio::Radio,
		utils::EGroupOrientation,
	},
	dioxus::prelude::*,
	std::{fmt::Debug, rc::Rc},
};

#[derive(Clone, PartialEq, Debug)]
pub struct RadioGroupContext {
	pub name: String,
	pub value: Memo<Option<String>>,
	pub set_value: Callback<Option<String>>,
	pub orientation: ReadOnlySignal<EGroupOrientation>,
	pub disabled: ReadOnlySignal<bool>,
}

impl RadioGroupContext {
	pub fn new(
		value: Memo<Option<String>>,
		set_value: Callback<Option<String>>,
		orientation: ReadOnlySignal<EGroupOrientation>,
		name: String,
		disabled: ReadOnlySignal<bool>,
	) -> Self {
		Self { value, set_value, orientation, name, disabled }
	}

	pub fn on_select(&self, value: String) {
		self.set_value.call(Some(value));
	}

	pub fn on_deselect(&self) {
		self.set_value.call(None);
	}
}

#[derive(Props, Clone, PartialEq)]
pub struct RadioGroupProps {
	pub name: String,
	#[props(optional, default = ReadOnlySignal::new(Signal::new(None)))]
	pub value: ReadOnlySignal<Option<String>>,
	#[props(optional, default = None)]
	default_value: Option<String>,
	#[props(optional)]
	pub on_value_change: Option<Callback<Option<String>>>,
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
pub fn RadioGroup(props: RadioGroupProps) -> Element {
	let RadioGroupProps { name, value, default_value, on_value_change, disabled, orientation, children, attributes, .. } = props;
	let is_controlled = use_hook(move || value().is_some());
	let (value, set_value) = use_controllable_state(UseControllableStateParams {
		is_controlled,
		prop: value,
		default_prop: default_value.unwrap_or_default(),
		on_change: on_value_change,
	});

	use_context_provider::<RadioGroupContext>(|| RadioGroupContext::new(value, set_value, orientation, name, disabled));

	let mut container_ref = use_signal(|| None::<Rc<MountedData>>);

	let handle_key_down = use_arrow_key_navigation(container_ref, Some("[role='radio'][data-focusable='true']".to_string()), orientation());
	let mut interaction_state = use_interaction_state();

	rsx! {
		div {
			role: "group",
			aria_disabled: disabled(),
			"data-disabled": disabled(),
			"data-hovered": *interaction_state.is_hovered.read(),
			"data-focused": *interaction_state.is_focused.read(),
			"data-focuse-visible": *interaction_state.is_focused.read(),
			onmousedown: move |event| {
					if let Some(handler) = props.onmousedown {
							handler.call(event);
					}
			},
			onkeydown: move |event| {
					handle_key_down(event.clone());
					if let Some(handler) = props.onkeydown {
							handler.call(event);
					}
			},
			onkeyup: move |event| {
					if let Some(handler) = props.onkeyup {
							handler.call(event);
					}
			},
			onmouseup: move |event| {
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
pub struct RadioGroupItemProps {
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
pub fn RadioGroupItem(props: RadioGroupItemProps) -> Element {
	let context = use_context::<RadioGroupContext>();
	let checked = use_memo(move || context.value.read().clone().unwrap_or_default() == *props.value.read());

	let is_disabled = use_memo(move || *props.disabled.read() || *props.disabled.read());

	rsx! {
		Radio {
			name: context.name.clone(),
			value: props.value,
			disabled: is_disabled(),
			checked: checked(),
			tabindex: if is_disabled() || !checked() { -1 } else { 0 },
			aria_orientation: &*context.orientation.clone().read().to_string(),
			"data-focusable": !is_disabled(),
			on_change: move |checked: Option<bool>| {
					if checked.is_some() {
							context.on_select(props.value.read().clone());
					} else {
							context.on_deselect();
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
			extra_attributes: props.attributes.clone(),
			{props.children}
		}
	}
}
