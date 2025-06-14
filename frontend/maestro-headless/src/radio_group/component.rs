use {
	crate::{
		radio::{Radio, RadioIndicator},
		shared::{EOrientation, UseControllableStateParams, use_arrow_key_navigation, use_controllable_state},
	},
	dioxus::prelude::*,
	std::{fmt::Debug, rc::Rc},
};

#[derive(Debug, Clone, PartialEq)]
pub struct RadioGroupContext {
	pub value: Memo<String>,
	pub set_value: Callback<String>,
	pub orientation: ReadOnlySignal<EOrientation>,
	pub disabled: ReadOnlySignal<bool>,
	pub required: bool,
}

impl RadioGroupContext {
	pub fn new(
		value: Memo<String>,
		set_value: Callback<String>,
		orientation: ReadOnlySignal<EOrientation>,
		disabled: ReadOnlySignal<bool>,
		required: bool,
	) -> Self {
		Self { value, set_value, orientation, disabled, required }
	}

	pub fn on_select(&self, value: String) {
		self.set_value.call(value);
	}

	pub fn on_deselect(&self) {
		self.set_value.call(String::new());
	}
}

#[derive(Clone, PartialEq, Props)]
pub struct RadioGroupRootProps {
	#[props(optional, default = ReadOnlySignal::new(Signal::new(None)))]
	pub value: ReadOnlySignal<Option<String>>,
	#[props(optional, default = None)]
	default_value: Option<String>,
	#[props(optional)]
	pub on_value_change: Option<Callback<String>>,
	#[props(optional, default = ReadOnlySignal::new(Signal::new(false)))]
	pub disabled: ReadOnlySignal<bool>,
	#[props(optional, default = false)]
	pub required: bool,
	#[props(optional, default = ReadOnlySignal::new(Signal::new(EOrientation::Horizontal)))]
	pub orientation: ReadOnlySignal<EOrientation>,

	#[props(extends = div, extends = GlobalAttributes)]
	pub attributes: Vec<Attribute>,
	#[props(default = Vec::new())]
	pub extra_attributes: Vec<Attribute>,
	pub children: Element,
}

#[component]
pub fn RadioGroupRoot(props: RadioGroupRootProps) -> Element {
	let RadioGroupRootProps { value, default_value, on_value_change, disabled, required, orientation, children, attributes, extra_attributes } = props;

	let is_controlled = use_hook(move || value().is_some());
	let (value, set_value) = use_controllable_state(UseControllableStateParams {
		is_controlled,
		prop: value,
		default_prop: default_value.unwrap_or_default(),
		on_change: on_value_change,
	});

	use_context_provider::<RadioGroupContext>(|| RadioGroupContext::new(value, set_value, orientation, disabled, required));

	let mut container_ref = use_signal(|| None::<Rc<MountedData>>);

	let handle_key_down = use_arrow_key_navigation(container_ref, Some("[role='radio'][data-focusable='true']".to_string()), orientation());

	rsx! {
		div {
			role: "group",
			aria_disabled: disabled(),
			"data-disabled": disabled(),
			aria_required: required,
			"data-required": required,
			aria_orientation: orientation().to_string(),
			"data-orientation": orientation().to_string(),
			onkeydown: handle_key_down,
			onmounted: move |event| container_ref.set(Some(event.data())),
			..attributes,
			..extra_attributes,
			{children}
		}
	}
}

#[derive(Clone, PartialEq, Props)]
pub struct RadioGroupItemProps {
	pub value: ReadOnlySignal<String>,
	#[props(optional, default = ReadOnlySignal::new(Signal::new(false)))]
	pub disabled: ReadOnlySignal<bool>,

	#[props(extends = button, extends = GlobalAttributes)]
	pub attributes: Vec<Attribute>,
	#[props(default = Vec::new())]
	pub extra_attributes: Vec<Attribute>,
	pub children: Element,
}

#[component]
pub fn RadioGroupItem(props: RadioGroupItemProps) -> Element {
	let RadioGroupItemProps { value, disabled, attributes, extra_attributes, children } = props;

	let context = use_context::<RadioGroupContext>();
	let checked = use_memo(move || context.value.read().clone() == value());

	let is_disabled = use_memo(move || *context.disabled.read() || disabled());

	let mut attrs = attributes.clone();
	attrs.extend(extra_attributes);

	rsx! {
		Radio {
			value,
			disabled: is_disabled(),
			required: context.required,
			checked: checked(),
			tabindex: if is_disabled() { -1 } else { 0 },
			aria_orientation: &*context.orientation.clone().read().to_string(),
			"data-focusable": !is_disabled(),
			on_change: move |checked: bool| {
					if checked {
							context.on_select(value());
					} else {
							context.on_deselect();
					}
			},
			extra_attributes: attrs.clone(),
			{children}
		}
	}
}

#[derive(Debug, Clone, PartialEq, Props)]
pub struct RadioGroupIndicatorProps {
	#[props(extends = GlobalAttributes)]
	pub attributes: Vec<Attribute>,
	#[props(default = Vec::new())]
	pub extra_attributes: Vec<Attribute>,
	#[props(default = None)]
	pub children: Option<Element>,
}

#[component]
pub fn RadioGroupIndicator(props: RadioGroupIndicatorProps) -> Element {
	let mut attrs = props.attributes.clone();
	attrs.extend(props.extra_attributes);

	if let Some(children) = props.children {
		rsx! {
			RadioIndicator { extra_attributes: attrs.clone(), {children} }
		}
	} else {
		rsx! {
			RadioIndicator { extra_attributes: attrs.clone() }
		}
	}
}
