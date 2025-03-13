use {
	crate::hooks::{InteractionStateContext, UseControllableStateParams, use_controllable_state, use_interaction_state},
	dioxus::prelude::*,
	dioxus_free_icons::{Icon, icons::bs_icons::BsCheck},
	std::fmt::Debug,
};

#[derive(Clone, PartialEq, Debug)]
pub struct CheckboxContext<T>
where
	T: Clone + PartialEq + Debug + 'static,
{
	pub value: T,
	pub name: String,
	pub onchange: Callback<Option<bool>>,
	pub checked: Memo<Option<bool>>,
	pub required: bool,
}

impl<T> CheckboxContext<T>
where
	T: Clone + PartialEq + Debug + 'static,
{
	pub fn new(value: T, onchange: Callback<Option<bool>>, checked: Memo<Option<bool>>, name: String, required: bool) -> Self {
		Self { value, onchange, checked, name, required }
	}
}

#[derive(Props, PartialEq, Debug, Clone)]
pub struct CheckboxProps<T>
where
	T: Clone + PartialEq + Debug + 'static,
{
	#[props(optional, default = ReadOnlySignal::new(Signal::new(false)))]
	pub disabled: ReadOnlySignal<bool>,
	pub value: T,
	pub name: String,
	#[props(optional, default = ReadOnlySignal::new(Signal::new(None)))]
	pub checked: ReadOnlySignal<Option<bool>>,
	#[props(optional, default = false)]
	pub default_checked: bool,
	#[props(extends = GlobalAttributes, extends = label)]
	pub attributes: Vec<Attribute>,
	#[props(default = None)]
	pub onchange: Option<Callback<Option<bool>>>,
	pub children: Element,
	#[props(default = false)]
	pub required: bool,
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
pub fn Checkbox<T: Clone + PartialEq + Debug + 'static>(props: CheckboxProps<T>) -> Element {
	let CheckboxProps { disabled, value, name, checked, default_checked, attributes, onchange, children, required, .. } = props;
	let is_controlled = use_hook(move || checked().is_some());
	let (checked, set_checked) =
		use_controllable_state(UseControllableStateParams { is_controlled, prop: checked, default_prop: default_checked, on_change: onchange });
	let checkbox_context = use_context_provider::<CheckboxContext<T>>(|| CheckboxContext::new(value, set_checked, checked, name, required));
	let mut interaction_state = use_interaction_state(ReadOnlySignal::new(Signal::new(false)), disabled);

	rsx! {
		label {
			style: "position:relative;",
			onmousedown: move |event| {
					interaction_state.onmousedown();
					if let Some(handler) = props.onmousedown {
							handler.call(event);
					}
			},
			onkeydown: move |event| {
					interaction_state.onkeydown();
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
			"data-pressed": *interaction_state.is_pressed.read(),
			"data-hovered": *interaction_state.is_hovered.read(),
			"data-focused": *interaction_state.is_focused.read(),
			"data-focuse-visible": *interaction_state.is_focused.read(),
			aria_checked: *checkbox_context.checked.read(),
			aria_required: *checkbox_context.checked.peek(),
			"data-state": if checkbox_context.checked.read().unwrap_or_default() { "checked" } else { "unchecked" },
			"data-disabled": *interaction_state.disabled.read(),
			aria_disabled: *interaction_state.disabled.read(),
			..attributes,
			{children}
		}
	}
}

#[derive(Props, PartialEq, Debug, Clone)]
pub struct CheckboxInputProps {
	#[props(extends = GlobalAttributes, extends = input)]
	pub attributes: Vec<Attribute>,
	pub children: Element,
	#[props(default = String::default())]
	pub class: String,
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
pub fn CheckboxInput<T: Clone + PartialEq + Debug + 'static>(props: CheckboxInputProps) -> Element {
	let checkbox_context = use_context::<CheckboxContext<T>>();
	let mut interaction_state = use_context::<InteractionStateContext>();

	rsx! {
		input {
			style: "position:absolute;width:0px;height:0px;margin:0px;opacity:0;z-index:-20",
			tabindex: -1,
			r#type: "checkbox",
			checked: *checkbox_context.checked.read(),
			name: *checkbox_context.checked.peek(),
			disabled: *interaction_state.disabled.read(),
			aria_hidden: true,
			onchange: move |_| {
					match !checkbox_context.checked.peek().unwrap_or_default() {
							true => checkbox_context.onchange.call(Some(true)),
							false => checkbox_context.onchange.call(None),
					};
			},
			..props.attributes,
		}
		div {
			tabindex: if !*interaction_state.disabled.read() { "0" } else { "-1" },
			onmousedown: move |event| {
					interaction_state.onmousedown();
					if let Some(handler) = props.onmousedown {
							handler.call(event);
					}
			},
			onkeydown: move |event| {
					interaction_state.onkeydown();
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
			class: props.class.clone(),
			role: "checkbox",
			aria_checked: *checkbox_context.checked.read(),
			aria_selected: *checkbox_context.checked.read(),
			aria_required: checkbox_context.required,
			"data-state": if checkbox_context.checked.read().unwrap_or_default() { "checked" } else { "unchecked" },
			"data-disabled": *interaction_state.disabled.read(),
			aria_disabled: *interaction_state.disabled.read(),
			"data-pressed": *interaction_state.is_pressed.read(),
			"data-hovered": *interaction_state.is_hovered.read(),
			"data-focused": *interaction_state.is_focused.read(),
			"data-focuse-visible": *interaction_state.is_focused.read(),
			{props.children}
		}
	}
}

#[derive(Props, PartialEq, Debug, Clone)]
pub struct CheckboxIndicatorProps {
	#[props(extends = GlobalAttributes)]
	pub attributes: Vec<Attribute>,
	#[props(default = None)]
	pub children: Option<Element>,
}

#[component]
pub fn CheckboxIndicator<T: Clone + PartialEq + Debug + 'static>(props: CheckboxIndicatorProps) -> Element {
	let checkbox_context = use_context::<CheckboxContext<T>>();
	let interaction_state = use_context::<InteractionStateContext>();

	rsx! {
		span {
			"data-state": if checkbox_context.checked.read().unwrap_or_default() { "checked" } else { "unchecked" },
			"data-disabled": *interaction_state.disabled.read(),
			style: "pointer-events:none;position:relative;display:flex;justify-content:center;align-items:center",
			..props.attributes,
			if checkbox_context.checked.read().unwrap_or_default() {
				if let Some(children) = props.children {
					{children}
				} else {
					Icon { icon: BsCheck }
				}
			}
		}
	}
}
