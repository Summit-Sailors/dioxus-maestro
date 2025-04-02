use {
	crate::{
		checkbox::{Checkbox, CheckboxIndicator},
		shared::{EOrientation, UseControllableStateParams, use_arrow_key_navigation, use_controllable_state},
	},
	dioxus::prelude::*,
	std::{fmt::Debug, rc::Rc},
};

#[derive(Clone, PartialEq, Debug)]
pub struct CheckboxGroupContext {
	pub name: String,
	pub value: Memo<Vec<String>>,
	pub set_value: Callback<Vec<String>>,
	pub orientation: ReadOnlySignal<EOrientation>,
	pub disabled: ReadOnlySignal<bool>,
}

impl CheckboxGroupContext {
	pub fn new(
		value: Memo<Vec<String>>,
		set_value: Callback<Vec<String>>,
		orientation: ReadOnlySignal<EOrientation>,
		name: String,
		disabled: ReadOnlySignal<bool>,
	) -> Self {
		Self { value, set_value, orientation, name, disabled }
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

#[derive(Props, Clone, PartialEq)]
pub struct CheckboxGroupProps {
	pub name: String,
	#[props(optional, default = ReadOnlySignal::new(Signal::new(None)))]
	pub value: ReadOnlySignal<Option<Vec<String>>>,
	#[props(optional, default = None)]
	default_value: Option<Vec<String>>,
	#[props(optional)]
	pub on_value_change: Option<Callback<Vec<String>>>,
	#[props(optional, default = ReadOnlySignal::new(Signal::new(false)))]
	pub disabled: ReadOnlySignal<bool>,
	#[props(optional, default = ReadOnlySignal::new(Signal::new(EOrientation::Horizontal)))]
	pub orientation: ReadOnlySignal<EOrientation>,

	#[props(extends = div, extends = GlobalAttributes)]
	pub attributes: Vec<Attribute>,
	pub children: Element,
}

#[component]
pub fn CheckboxGroup(props: CheckboxGroupProps) -> Element {
	let CheckboxGroupProps { name, value, default_value, on_value_change, disabled, orientation, children, attributes } = props;

	let is_controlled = use_hook(move || value().is_some());
	let (value, set_value) = use_controllable_state(UseControllableStateParams {
		is_controlled,
		prop: value,
		default_prop: default_value.unwrap_or_default(),
		on_change: on_value_change,
	});

	use_context_provider::<CheckboxGroupContext>(|| CheckboxGroupContext::new(value, set_value, orientation, name, disabled));

	let mut container_ref = use_signal(|| None::<Rc<MountedData>>);

	let handle_key_down = use_arrow_key_navigation(container_ref, Some("[role='checkbox']:not([tabindex='-1'])".to_string()), orientation());

	rsx! {
		div {
			role: "group",
			aria_disabled: disabled().then_some(Some(true)),
			"data-disabled": disabled().then_some(Some(true)),
			onkeydown: handle_key_down,
			onmounted: move |event| container_ref.set(Some(event.data())),
			..attributes,
			{children}
		}
	}
}

#[derive(Props, Clone, PartialEq)]
pub struct CheckboxGroupItemProps {
	pub value: ReadOnlySignal<String>,
	#[props(optional, default = ReadOnlySignal::new(Signal::new(false)))]
	pub disabled: ReadOnlySignal<bool>,
	#[props(extends = button, extends = GlobalAttributes)]
	pub attributes: Vec<Attribute>,
	#[props(default = None)]
	pub children: Option<Element>,
}

#[component]
pub fn CheckboxGroupItem(props: CheckboxGroupItemProps) -> Element {
	let CheckboxGroupItemProps { value, disabled, attributes, children } = props;

	let context = use_context::<CheckboxGroupContext>();
	let checked = use_memo(move || context.value.read().clone().contains(&value()));

	let is_disabled = use_memo(move || *context.disabled.read() || disabled());
	use_context_provider::<Memo<bool>>(|| is_disabled);

	rsx! {
		Checkbox {
			name: context.name.clone(),
			value,
			disabled: is_disabled(),
			checked: checked(),
			on_change: move |checked: bool| {
					if checked {
							context.on_select(value());
					} else {
							context.on_deselect(value());
					}
			},
			extra_attributes: attributes.clone(),
			if let Some(children) = children {
				{children}
			} else {
				{
						rsx! {
							CheckboxIndicator {}
						}
				}
			}
		}
	}
}
