use {
	crate::hooks::{InteractionStateContext, use_interaction_state},
	dioxus::prelude::*,
	dioxus_free_icons::{Icon, icons::bs_icons::BsCheck},
	std::fmt::Debug,
};

#[derive(Clone, PartialEq, Debug)]
pub struct CheckboxContext {
	pub value: String,
	pub name: String,
	pub onchange: Option<Callback<bool>>,
	pub checked: Signal<bool>,
	pub disabled: Signal<bool>,
	pub required: bool,
}

impl CheckboxContext {
	pub fn new(value: String, onchange: Option<Callback<bool>>, checked: Signal<bool>, disabled: Signal<bool>, name: String, required: bool) -> Self {
		Self { value, onchange, disabled, checked, name, required }
	}
}

#[derive(Props, PartialEq, Debug, Clone)]
pub struct CheckboxProps {
	#[props(optional, default = Signal::new(false))]
	pub disabled: Signal<bool>,
	pub value: String,
	pub name: String,
	#[props(optional, default = Signal::new(false))]
	pub checked: Signal<bool>,
	#[props(extends = GlobalAttributes, extends = label)]
	pub attributes: Vec<Attribute>,
	#[props(default = None)]
	pub onchange: Option<Callback<bool>>,
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
pub fn Checkbox(props: CheckboxProps) -> Element {
	let CheckboxProps { disabled, value, name, checked, attributes, onchange, children, required, .. } = props;
	let checkbox_context = use_context_provider::<CheckboxContext>(|| CheckboxContext::new(value, onchange, checked, disabled, name, required));
	let mut interaction_state = use_interaction_state(Signal::new(false), props.disabled);

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
			"data-state": if *checkbox_context.checked.read() { "checked" } else { "unchecked" },
			"data-disabled": *checkbox_context.disabled.read(),
			aria_disabled: *checkbox_context.disabled.read(),
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
pub fn CheckboxInput(props: CheckboxInputProps) -> Element {
	let mut checkbox_context = use_context::<CheckboxContext>();
	let mut interaction_state = use_context::<InteractionStateContext>();

	rsx! {
		input {
			style: "position:absolute;width:0px;height:0px;margin:0px;opacity:0;z-index:-20",
			tabindex: -1,
			r#type: "checkbox",
			checked: *checkbox_context.checked.read(),
			name: *checkbox_context.checked.peek(),
			disabled: *checkbox_context.disabled.read(),
			aria_hidden: true,
			onchange: move |_| {
					let new_checked = !*checkbox_context.checked.peek();
					checkbox_context.checked.set(new_checked);
					if let Some(callback) = checkbox_context.onchange {
							callback.call(new_checked);
					}
			},
			..props.attributes,
		}
		div {
			tabindex: if !*checkbox_context.disabled.read() { "0" } else { "-1" },
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
			aria_required: *checkbox_context.checked.peek(),
			"data-state": if *checkbox_context.checked.read() { "checked" } else { "unchecked" },
			"data-disabled": *checkbox_context.disabled.read(),
			aria_disabled: *checkbox_context.disabled.read(),
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
pub fn CheckboxIndicator(props: CheckboxIndicatorProps) -> Element {
	let checkbox_context = use_context::<CheckboxContext>();

	rsx! {
		span {
			"data-state": if *checkbox_context.checked.read() { "checked" } else { "unchecked" },
			"data-disabled": *checkbox_context.disabled.read(),
			style: "pointer-events:none;",
			..props.attributes,
			if *checkbox_context.checked.read() {
				if let Some(children) = props.children {
					{children}
				} else {
					Icon { icon: BsCheck }
				}
			}
		}
	}
}
