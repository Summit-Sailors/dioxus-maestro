use {
	crate::{
		checkbox::{CheckboxIndicator, CheckboxRoot},
		shared::{EOrientation, UseControllableStateParams, use_arrow_key_navigation, use_controllable_state},
	},
	dioxus::prelude::*,
	std::{fmt::Debug, rc::Rc},
};

#[derive(Debug, Clone, PartialEq)]
pub struct CheckboxGroupContext {
	pub value: Memo<Vec<String>>,
	pub set_value: Callback<Vec<String>>,
	pub orientation: ReadOnlySignal<EOrientation>,
	pub disabled: ReadOnlySignal<bool>,
	pub required: ReadOnlySignal<bool>,
}

impl CheckboxGroupContext {
	pub fn new(
		value: Memo<Vec<String>>,
		set_value: Callback<Vec<String>>,
		orientation: ReadOnlySignal<EOrientation>,
		disabled: ReadOnlySignal<bool>,
		required: ReadOnlySignal<bool>,
	) -> Self {
		Self { value, set_value, orientation, disabled, required }
	}

	pub fn on_select(&self, value: String) {
		let mut values = self.value.peek().clone();
		if !values.contains(&value) {
			values.push(value);
			self.set_value.call(values);
		}
	}

	pub fn on_deselect(&self, value: String) {
		let mut values = self.value.peek().clone();
		values.retain(|v| v != &value);
		self.set_value.call(values);
	}
}

#[derive(Clone, PartialEq, Props)]
pub struct CheckboxGroupProps {
	#[props(optional, default = ReadOnlySignal::new(Signal::new(None)))]
	pub value: ReadOnlySignal<Option<Vec<String>>>,
	#[props(optional, default = None)]
	default_value: Option<Vec<String>>,
	#[props(optional)]
	pub on_value_change: Option<Callback<Vec<String>>>,
	#[props(optional, default = ReadOnlySignal::new(Signal::new(false)))]
	pub disabled: ReadOnlySignal<bool>,
	#[props(optional, default = ReadOnlySignal::new(Signal::new(false)))]
	pub required: ReadOnlySignal<bool>,
	#[props(optional, default = ReadOnlySignal::new(Signal::new(EOrientation::Horizontal)))]
	pub orientation: ReadOnlySignal<EOrientation>,

	#[props(extends = div, extends = GlobalAttributes)]
	pub attributes: Vec<Attribute>,
	#[props(default = Vec::new())]
	pub extra_attributes: Vec<Attribute>,
	pub children: Element,
}

#[component]
pub fn CheckboxGroup(props: CheckboxGroupProps) -> Element {
	let CheckboxGroupProps { value, default_value, on_value_change, disabled, required, orientation, children, attributes, extra_attributes } = props;

	let is_controlled = use_hook(move || value().is_some());
	let (value, set_value) = use_controllable_state(UseControllableStateParams {
		is_controlled,
		prop: value,
		default_prop: default_value.unwrap_or_default(),
		on_change: on_value_change,
	});

	use_context_provider::<CheckboxGroupContext>(|| CheckboxGroupContext::new(value, set_value, orientation, disabled, required));

	let mut container_ref = use_signal(|| None::<Rc<MountedData>>);

	let handle_key_down = use_arrow_key_navigation(container_ref, Some("[role='checkbox']:not([tabindex='-1'])".to_string()), orientation());

	rsx! {
		div {
			role: "group",
			aria_disabled: disabled().then_some(Some(true)),
			"data-disabled": disabled().then_some(Some(true)),
			aria_required: required().then_some(Some(true)),
			"data-required": required().then_some(Some(true)),
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
pub struct CheckboxGroupItemProps {
	pub value: ReadOnlySignal<String>,
	#[props(optional, default = ReadOnlySignal::new(Signal::new(false)))]
	pub disabled: ReadOnlySignal<bool>,
	#[props(extends = button, extends = GlobalAttributes)]
	pub attributes: Vec<Attribute>,
	#[props(default = Vec::new())]
	pub extra_attributes: Vec<Attribute>,
	#[props(default = None)]
	pub children: Option<Element>,
}

#[component]
pub fn CheckboxGroupItem(props: CheckboxGroupItemProps) -> Element {
	let CheckboxGroupItemProps { value, disabled, attributes, extra_attributes, children } = props;

	let context = use_context::<CheckboxGroupContext>();
	let checked = use_memo(move || context.value.read().clone().contains(&value()));

	let is_disabled = use_memo(move || *context.disabled.read() || disabled());
	use_context_provider::<Memo<bool>>(|| is_disabled);

	let mut attrs = attributes.clone();
	attrs.extend(extra_attributes);

	rsx! {
		CheckboxRoot {
			value,
			required: context.required,
			disabled: is_disabled(),
			checked: checked(),
			on_change: move |checked: bool| {
					if checked {
							context.on_select(value());
					} else {
							context.on_deselect(value());
					}
			},
			extra_attributes: attrs.clone(),
			{children}
		}
	}
}

#[derive(Debug, Clone, PartialEq, Props)]
pub struct CheckboxGroupIndicatorProps {
	#[props(extends = GlobalAttributes)]
	pub attributes: Vec<Attribute>,
	#[props(default = None)]
	pub children: Option<Element>,
	#[props(default = Vec::new())]
	pub extra_attributes: Vec<Attribute>,
}

#[component]
pub fn CheckboxGroupIndicator(props: CheckboxGroupIndicatorProps) -> Element {
	let mut attrs = props.attributes.clone();
	attrs.extend(props.extra_attributes.clone());

	if let Some(children) = props.children {
		rsx! {
			CheckboxIndicator { extra_attributes: attrs.clone(), {children} }
		}
	} else {
		rsx! {
			CheckboxIndicator { extra_attributes: attrs.clone() }
		}
	}
}
