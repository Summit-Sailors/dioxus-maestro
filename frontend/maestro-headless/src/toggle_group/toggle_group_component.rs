use {
	crate::{
		hooks::{UseControllableStateParams, use_arrow_key_navigation, use_controllable_state},
		toggle::Toggle,
		utils::EOrientation,
	},
	dioxus::prelude::*,
	std::{fmt::Debug, rc::Rc},
};

#[derive(Clone, PartialEq, Debug, Copy)]
pub struct ToggleGroupContext {
	pub value: Memo<Option<String>>,
	pub set_value: Callback<Option<String>>,
	pub orientation: ReadOnlySignal<EOrientation>,
	pub disabled: ReadOnlySignal<bool>,
}

impl ToggleGroupContext {
	pub fn new(
		value: Memo<Option<String>>,
		set_value: Callback<Option<String>>,
		orientation: ReadOnlySignal<EOrientation>,
		disabled: ReadOnlySignal<bool>,
	) -> Self {
		Self { value, set_value, orientation, disabled }
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
	#[props(optional, default = ReadOnlySignal::new(Signal::new(EOrientation::Horizontal)))]
	pub orientation: ReadOnlySignal<EOrientation>,

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

	use_context_provider::<ToggleGroupContext>(|| ToggleGroupContext::new(value, set_value, orientation, disabled));

	let mut container_ref = use_signal(|| None::<Rc<MountedData>>);

	let on_key_down = use_arrow_key_navigation(container_ref, Some("[role='radio'][data-focusable='true']".to_string()), orientation());

	rsx! {
		div {
			role: "group",
			aria_disabled: disabled(),
			aria_orientation: &*orientation.read().to_string(),
			onkeydown: on_key_down,
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

	let is_disabled = use_memo(move || *context.disabled.read() || *props.disabled.read());
	use_context_provider::<Memo<bool>>(|| is_disabled);

	rsx! {
		Toggle {
			value: context.value.read().clone().unwrap_or("on".into()),
			pressed: pressed(),
			tabindex: if is_disabled() || !pressed() { -1 } else { 0 },
			disabled: is_disabled(),
			aria_orientation: &*context.orientation.clone().read().to_string(),
			"data-focusable": !is_disabled(),
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
			extra_attributes: props.attributes.clone(),
			{props.children}
		}
	}
}
