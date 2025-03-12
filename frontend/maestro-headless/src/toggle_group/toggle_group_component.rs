use {
	crate::{hooks::use_arrow_key_navigation, toggle::Toggle},
	dioxus::prelude::*,
	std::rc::Rc,
};

#[derive(Clone, PartialEq, Debug, Copy)]
pub enum ToggleGroupOrientation {
	Horizontal,
	Vertical,
}

#[derive(Clone, PartialEq, Debug, Copy)]
pub struct ToggleGroupContext {
	pub value: Signal<String>,
	pub on_value_change: Option<Callback<String>>,
	pub disabled: Signal<bool>,
	pub orientation: Signal<ToggleGroupOrientation>,
}

impl ToggleGroupContext {
	pub fn new(value: Signal<String>, on_value_change: Option<Callback<String>>, disabled: Signal<bool>, orientation: Signal<ToggleGroupOrientation>) -> Self {
		Self { value, on_value_change, disabled, orientation }
	}

	pub fn onselect(&mut self, value: String) {
		self.value.set(value.clone());
		if let Some(callback) = self.on_value_change {
			callback.call(value);
		}
	}

	pub fn ondeselect(&mut self) {
		self.value.set(String::default());
		if let Some(callback) = self.on_value_change {
			callback.call(String::default());
		}
	}
}

#[derive(Clone, PartialEq, Debug, Copy)]
pub struct ToggleGroupItemContext {
	pub value: Signal<String>,
	pub pressed: Signal<bool>,
	pub disabled: Signal<bool>,
}

impl ToggleGroupItemContext {
	pub fn new(value: Signal<String>, pressed: Signal<bool>, disabled: Signal<bool>) -> Self {
		Self { value, pressed, disabled }
	}
}

#[derive(Props, Clone, PartialEq)]
pub struct ToggleGroupProps {
	#[props(optional, default = Signal::new(String::new()))]
	pub value: Signal<String>,
	#[props(optional)]
	pub on_value_change: Option<Callback<String>>,
	#[props(optional, default = Signal::new(false))]
	pub disabled: Signal<bool>,
	#[props(optional, default = Signal::new(ToggleGroupOrientation::Horizontal))]
	pub orientation: Signal<ToggleGroupOrientation>,
	#[props(extends = div, extends = GlobalAttributes)]
	pub attributes: Vec<Attribute>,
	pub children: Element,
}

#[component]
pub fn ToggleGroup(props: ToggleGroupProps) -> Element {
	let ToggleGroupProps { value, on_value_change, disabled, orientation, children, attributes } = props;

	let toggle_group_context = use_context_provider::<ToggleGroupContext>(|| ToggleGroupContext::new(value, on_value_change, disabled, orientation));

	let mut container_ref = use_signal(|| None::<Rc<MountedData>>);

	let handle_key_down = use_arrow_key_navigation(container_ref, Some("[role='radio']:not([tabindex='-1'])".to_string()));

	rsx! {
		div {
			role: "group",
			aria_disabled: *toggle_group_context.disabled.read(),
			"data-disabled": *toggle_group_context.disabled.read(),
			onmounted: move |event| container_ref.set(Some(event.data())),
			onkeydown: handle_key_down,
			..attributes,
			{children}
		}
	}
}

#[derive(Props, Clone, PartialEq)]
pub struct ToggleGroupItemProps {
	pub value: String,
	#[props(optional, default = Signal::new(false))]
	pub disabled: Signal<bool>,
	#[props(extends = button, extends = GlobalAttributes)]
	pub attributes: Vec<Attribute>,
	pub children: Element,
}

#[component]
pub fn ToggleGroupItem(props: ToggleGroupItemProps) -> Element {
	let mut toggle_group_context = use_context::<ToggleGroupContext>();
	let mut toggle_group_item_context = use_context_provider::<ToggleGroupItemContext>(|| {
		ToggleGroupItemContext::new(Signal::new(props.value.clone()), Signal::new(*toggle_group_context.value.peek() == props.value.clone()), props.disabled)
	});

	let is_disabled = *toggle_group_context.disabled.read() || *toggle_group_item_context.disabled.read();

	use_effect(move || {
		if *toggle_group_context.value.read() != *toggle_group_item_context.value.peek() {
			toggle_group_item_context.pressed.set(false);
		}
	});

	rsx! {
		Toggle {
			role: "radio",
			tabindex: if is_disabled { -1 } else { 0 },
			disabled: Signal::new(is_disabled),
			pressed: toggle_group_item_context.pressed,
			on_toggle_change: move |pressed: bool| {
					let value = toggle_group_item_context.value.peek().clone();
					if pressed {
							toggle_group_context.onselect(value);
					} else {
							toggle_group_context.ondeselect();
					}
			},
			additional_attributes: props.attributes.clone(),
			{props.children}
		}
	}
}
