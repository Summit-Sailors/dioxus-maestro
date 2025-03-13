use {
	crate::{
		hooks::{InteractionStateContext, UseControllableStateParams, use_arrow_key_navigation, use_controllable_state, use_interaction_state},
		toggle::Toggle,
		utils::GroupOrientation,
	},
	dioxus::prelude::*,
	dioxus_logger::tracing::info,
	std::{fmt::Debug, rc::Rc},
};

#[derive(Clone, PartialEq, Debug)]
pub struct ToggleGroupContext<T>
where
	T: Clone + PartialEq + Debug + Default + 'static,
{
	pub value: Memo<Option<T>>,
	pub on_value_change: Callback<Option<T>>,
	pub orientation: ReadOnlySignal<GroupOrientation>,
}

impl<T> ToggleGroupContext<T>
where
	T: Clone + PartialEq + Debug + Default + 'static,
{
	pub fn new(value: Memo<Option<T>>, on_value_change: Callback<Option<T>>, orientation: ReadOnlySignal<GroupOrientation>) -> Self {
		Self { value, on_value_change, orientation }
	}

	pub fn onselect(&self, value: T) {
		self.on_value_change.call(Some(value));
	}

	pub fn ondeselect(&self) {
		self.on_value_change.call(None);
	}
}

#[derive(Props, Clone, PartialEq)]
pub struct ToggleGroupProps<T>
where
	T: Clone + PartialEq + Debug + Default + 'static,
{
	#[props(optional, default = ReadOnlySignal::new(Signal::new(None)))]
	pub value: ReadOnlySignal<Option<T>>,
	#[props(optional, default = None)]
	default_value: Option<T>,
	#[props(optional)]
	pub on_value_change: Option<Callback<Option<T>>>,
	#[props(optional, default = ReadOnlySignal::new(Signal::new(false)))]
	pub disabled: ReadOnlySignal<bool>,
	#[props(optional, default = ReadOnlySignal::new(Signal::new(GroupOrientation::Horizontal)))]
	pub orientation: ReadOnlySignal<GroupOrientation>,
	#[props(extends = div, extends = GlobalAttributes)]
	pub attributes: Vec<Attribute>,
	pub children: Element,
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
}

#[component]
pub fn ToggleGroup<T: Clone + PartialEq + Debug + Default + 'static>(props: ToggleGroupProps<T>) -> Element {
	let ToggleGroupProps { value, default_value, on_value_change, disabled, orientation, children, attributes, .. } = props;
	let is_controlled = use_hook(move || value().is_some());
	let (value, set_value) = use_controllable_state(UseControllableStateParams {
		is_controlled,
		prop: value,
		default_prop: default_value.unwrap_or_default(),
		on_change: on_value_change,
	});

	use_context_provider::<ToggleGroupContext<T>>(|| ToggleGroupContext::new(value, set_value, orientation));

	let mut container_ref = use_signal(|| None::<Rc<MountedData>>);

	let handle_key_down = use_arrow_key_navigation(container_ref, Some("[role='radio']:not([tabindex='-1'])".to_string()));
	let mut interaction_state = use_interaction_state(ReadOnlySignal::new(Signal::new(false)), disabled);

	rsx! {
		div {
			role: "group",
			aria_disabled: *interaction_state.disabled.read(),
			"data-disabled": *interaction_state.disabled.read(),
			onmousedown: move |event| {
					interaction_state.onmousedown();
					if let Some(handler) = props.onmousedown {
							handler.call(event);
					}
			},
			onkeydown: move |event| {
					interaction_state.onkeydown();
					handle_key_down(event.clone());
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
			"data-pressed": *interaction_state.is_pressed.read(),
			"data-hovered": *interaction_state.is_hovered.read(),
			"data-focused": *interaction_state.is_focused.read(),
			"data-focuse-visible": *interaction_state.is_focused.read(),
			..attributes,
			{children}
		}
	}
}

#[derive(Props, Clone, PartialEq)]
pub struct ToggleGroupItemProps<T>
where
	T: Clone + PartialEq + Debug + Default + 'static,
{
	pub value: ReadOnlySignal<T>,
	#[props(optional, default = ReadOnlySignal::new(Signal::new(false)))]
	pub disabled: ReadOnlySignal<bool>,
	#[props(extends = button, extends = GlobalAttributes)]
	pub attributes: Vec<Attribute>,
	pub children: Element,
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
}

#[component]
pub fn ToggleGroupItem<T: Clone + PartialEq + Debug + Default + 'static>(props: ToggleGroupItemProps<T>) -> Element {
	let toggle_group_context = use_context::<ToggleGroupContext<T>>();
	let pressed = use_memo(move || toggle_group_context.value.read().clone().unwrap_or_default() == *props.value.read());
	let state_context = use_context::<InteractionStateContext>();

	let is_disabled = use_memo(move || *state_context.disabled.read() || *props.disabled.read());
	use_context_provider::<Memo<bool>>(|| is_disabled);

	rsx! {
		Toggle {
			role: "radio",
			tabindex: if is_disabled() { -1 } else { 0 },
			disabled: is_disabled(),
			pressed: pressed(),
			on_toggle_change: move |pressed: Option<bool>| {
					if pressed.is_some() {
							toggle_group_context.onselect(props.value.read().clone());
					} else {
							toggle_group_context.ondeselect();
					}
			},
			additional_attributes: props.attributes.clone(),
			onblur: props.onblur,
			onfocus: props.onfocus,
			onkeydown: props.onkeydown,
			onkeyup: props.onkeyup,
			onmousedown: props.onmousedown,
			onmouseenter: props.onmouseenter,
			onmouseleave: props.onmouseleave,
			onmouseup: props.onmouseup,
			{props.children}
		}
	}
}
