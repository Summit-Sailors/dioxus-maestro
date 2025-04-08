use {
	crate::{
		shared::{EOrientation, UseControllableStateParams, use_arrow_key_navigation, use_controllable_state},
		toggle::Toggle,
	},
	dioxus::prelude::*,
	std::{fmt::Debug, rc::Rc},
};

#[derive(Clone, PartialEq, Debug, Copy)]
pub struct ToggleGroupContext {
	pub value: Memo<String>,
	pub set_value: Callback<String>,
	pub orientation: ReadOnlySignal<EOrientation>,
	pub disabled: ReadOnlySignal<bool>,
}

impl ToggleGroupContext {
	pub fn new(value: Memo<String>, set_value: Callback<String>, orientation: ReadOnlySignal<EOrientation>, disabled: ReadOnlySignal<bool>) -> Self {
		Self { value, set_value, orientation, disabled }
	}

	pub fn onselect(&self, value: String) {
		self.set_value.call(value);
	}

	pub fn ondeselect(&self) {
		self.set_value.call(String::default());
	}
}

#[derive(Props, Clone, PartialEq)]
pub struct ToggleGroupRootProps {
	#[props(optional, default = ReadOnlySignal::new(Signal::new(None)))]
	pub value: ReadOnlySignal<Option<String>>,
	#[props(optional, default = None)]
	default_value: Option<String>,
	#[props(optional)]
	pub on_value_chenge: Option<Callback<String>>,

	#[props(optional, default = ReadOnlySignal::new(Signal::new(false)))]
	pub disabled: ReadOnlySignal<bool>,
	#[props(optional, default = ReadOnlySignal::new(Signal::new(EOrientation::Horizontal)))]
	pub orientation: ReadOnlySignal<EOrientation>,

	#[props(extends = div, extends = GlobalAttributes)]
	pub attributes: Vec<Attribute>,
	pub children: Element,
}

#[component]
pub fn ToggleGroupRoot(props: ToggleGroupRootProps) -> Element {
	let ToggleGroupRootProps { value, default_value, on_value_chenge, disabled, orientation, children, attributes } = props;

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
			aria_orientation: orientation().to_string(),
			"data-disabled": disabled(),
			"data-orientation": orientation().to_string(),
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

	#[props(extends = button, extends = GlobalAttributes)]
	pub attributes: Vec<Attribute>,
	pub children: Element,
}

#[component]
pub fn ToggleGroupItem(props: ToggleGroupItemProps) -> Element {
	let ToggleGroupItemProps { value, disabled, attributes, children } = props;

	let context = use_context::<ToggleGroupContext>();
	let pressed = use_memo(move || *context.value.read().clone() == value());

	let is_disabled = use_memo(move || *context.disabled.read() || disabled());
	use_context_provider::<Memo<bool>>(|| is_disabled);

	rsx! {
		Toggle {
			value: if context.value.read().clone().is_empty() { "on" } else { context.value.read().clone() },
			pressed: pressed(),
			tabindex: if is_disabled() || !pressed() { -1 } else { 0 },
			disabled: is_disabled(),
			aria_orientation: &*context.orientation.clone().read().to_string(),
			"data-orientation": &*context.orientation.clone().read().to_string(),
			"data-focusable": !is_disabled(),
			on_toggle_change: move |pressed: bool| {
					if pressed {
							context.onselect(value());
					} else {
							context.ondeselect();
					}
			},
			extra_attributes: attributes.clone(),
			{children}
		}
	}
}
