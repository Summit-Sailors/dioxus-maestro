use {
	crate::hooks::{InteractionStateContext, UseControllableStateParams, use_controllable_state, use_interaction_state},
	dioxus::prelude::*,
	std::fmt::Debug,
};

#[derive(Clone, PartialEq, Debug)]
pub struct RadioContext<T>
where
	T: Clone + PartialEq + Debug + Default + 'static,
{
	pub value: ReadOnlySignal<T>,
	pub name: String,
	pub onchange: Callback<Option<bool>>,
	pub checked: Memo<Option<bool>>,
	pub required: bool,
}

impl<T> RadioContext<T>
where
	T: Clone + PartialEq + Debug + Default + 'static,
{
	pub fn new(value: ReadOnlySignal<T>, onchange: Callback<Option<bool>>, checked: Memo<Option<bool>>, name: String, required: bool) -> Self {
		Self { value, onchange, checked, name, required }
	}
}

#[derive(Props, PartialEq, Debug, Clone)]
pub struct RadioProps<T>
where
	T: Clone + PartialEq + Debug + 'static,
{
	#[props(optional, default = ReadOnlySignal::new(Signal::new(false)))]
	pub disabled: ReadOnlySignal<bool>,
	pub value: ReadOnlySignal<T>,
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
pub fn Radio<T: Clone + PartialEq + Debug + Default + 'static>(props: RadioProps<T>) -> Element {
	let RadioProps { disabled, value, name, checked, default_checked, attributes, onchange, children, required, .. } = props;
	let is_controlled = use_hook(move || checked().is_some());
	let (checked, set_checked) =
		use_controllable_state(UseControllableStateParams { is_controlled, prop: checked, default_prop: default_checked, on_change: onchange });
	let radio_context = use_context_provider::<RadioContext<T>>(|| RadioContext::new(value, set_checked, checked, name, required));
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
			aria_checked: *radio_context.checked.read(),
			aria_required: *radio_context.checked.peek(),
			"data-state": if radio_context.checked.read().unwrap_or_default() { "checked" } else { "unchecked" },
			"data-disabled": *interaction_state.disabled.read(),
			aria_disabled: *interaction_state.disabled.read(),
			..attributes,
			{children}
		}
	}
}

#[derive(Props, PartialEq, Debug, Clone)]
pub struct RadioInputProps {
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
pub fn RadioInput<T: Clone + PartialEq + Debug + Default + 'static>(props: RadioInputProps) -> Element {
	let radio_context = use_context::<RadioContext<T>>();
	let mut interaction_state = use_context::<InteractionStateContext>();

	rsx! {
		input {
			style: "position:absolute;width:0px;height:0px;margin:0px;opacity:0;z-index:-20",
			tabindex: -1,
			r#type: "radio",
			checked: *radio_context.checked.read(),
			name: radio_context.name,
			disabled: *interaction_state.disabled.read(),
			aria_hidden: true,
			onchange: move |_| {
					match !radio_context.checked.peek().unwrap_or_default() {
							true => radio_context.onchange.call(Some(true)),
							false => radio_context.onchange.call(None),
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
			role: "radio",
			aria_checked: *radio_context.checked.read(),
			aria_selected: *radio_context.checked.read(),
			aria_required: radio_context.required,
			"data-state": if radio_context.checked.read().unwrap_or_default() { "checked" } else { "unchecked" },
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
pub struct RadioIndicatorProps {
	#[props(extends = GlobalAttributes)]
	pub attributes: Vec<Attribute>,
	#[props(default = None)]
	pub children: Option<Element>,
}

#[component]
pub fn RadioIndicator<T: Clone + PartialEq + Debug + Default + 'static>(props: RadioIndicatorProps) -> Element {
	let radio_context = use_context::<RadioContext<T>>();
	let interaction_state = use_context::<InteractionStateContext>();

	rsx! {
		span {
			"data-state": if radio_context.checked.read().unwrap_or_default() { "checked" } else { "unchecked" },
			"data-disabled": *interaction_state.disabled.read(),
			style: "pointer-events:none;position:relative;display:flex;justify-content:center;align-items:center;",
			..props.attributes,
			if radio_context.checked.read().unwrap_or_default() {
				if let Some(children) = props.children {
					{children}
				} else {
					span { style: "width:8px;height:8px;border:1px solid;rounded:100%;" }
				}
			}
		}
	}
}
