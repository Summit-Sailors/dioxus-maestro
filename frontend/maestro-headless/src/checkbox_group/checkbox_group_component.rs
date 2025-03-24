use {
	crate::{
		checkbox::Checkbox,
		hooks::{UseControllableStateParams, use_arrow_key_navigation, use_controllable_state, use_interaction_state},
		utils::EOrientation,
	},
	dioxus::prelude::*,
	std::{fmt::Debug, rc::Rc},
};

#[derive(Clone, PartialEq, Debug)]
pub struct CheckboxGroupContext {
	pub name: String,
	pub value: Memo<Option<Vec<String>>>,
	pub set_value: Callback<Option<Vec<String>>>,
	pub orientation: ReadOnlySignal<EOrientation>,
	pub disabled: ReadOnlySignal<bool>,
}

impl CheckboxGroupContext {
	pub fn new(
		value: Memo<Option<Vec<String>>>,
		set_value: Callback<Option<Vec<String>>>,
		orientation: ReadOnlySignal<EOrientation>,
		name: String,
		disabled: ReadOnlySignal<bool>,
	) -> Self {
		Self { value, set_value, orientation, name, disabled }
	}

	pub fn on_select(&self, value: String) {
		let mut values = self.value.peek().clone().unwrap_or_default();
		if values.contains(&value) {
			values.push(value);
			self.set_value.call(Some(values));
		}
	}

	pub fn on_deselect(&self, value: String) {
		let mut values = self.value.peek().clone().unwrap_or_default();
		values.retain(|v| v != &value);
		self.set_value.call(Some(values));
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
	pub on_value_change: Option<Callback<Option<Vec<String>>>>,
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
pub fn CheckboxGroup(props: CheckboxGroupProps) -> Element {
	let CheckboxGroupProps { name, value, default_value, on_value_change, disabled, orientation, children, attributes, .. } = props;
	let is_controlled = use_hook(move || value().is_some());
	let (value, set_value) = use_controllable_state(UseControllableStateParams {
		is_controlled,
		prop: value,
		default_prop: default_value.unwrap_or_default(),
		on_change: on_value_change,
	});

	use_context_provider::<CheckboxGroupContext>(|| CheckboxGroupContext::new(value, set_value, orientation, name, disabled));

	let mut container_ref = use_signal(|| None::<Rc<MountedData>>);

	let handle_key_down = use_arrow_key_navigation(container_ref, Some("[role='radio']:not([tabindex='-1'])".to_string()), orientation());
	let mut interaction_state = use_interaction_state();

	rsx! {
		div {
			role: "group",
			aria_disabled: disabled,
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
pub struct CheckboxGroupItemProps {
	pub value: ReadOnlySignal<String>,
	#[props(optional, default = ReadOnlySignal::new(Signal::new(false)))]
	pub disabled: ReadOnlySignal<bool>,
	#[props(default = String::default())]
	pub class: String,

	#[props(extends = button, extends = GlobalAttributes)]
	pub attributes: Vec<Attribute>,
	pub children: Element,
}

#[component]
pub fn CheckboxGroupItem(props: CheckboxGroupItemProps) -> Element {
	let context = use_context::<CheckboxGroupContext>();
	let checked = use_memo(move || context.value.read().clone().unwrap_or_default().contains(&*props.value.read()));

	let is_disabled = use_memo(move || *context.disabled.read() || *props.disabled.read());
	use_context_provider::<Memo<bool>>(|| is_disabled);

	rsx! {
		Checkbox {
			name: context.name.clone(),
			value: props.value,
			class: props.class.clone(),
			disabled: is_disabled(),
			checked: checked(),
			on_change: move |checked: Option<bool>| {
					if checked.is_some() {
							context.on_select(props.value.read().clone());
					} else {
							context.on_deselect(props.value.read().clone());
					}
			},
			extra_attributes: props.attributes.clone(),
			{props.children}
		}
	}
}
