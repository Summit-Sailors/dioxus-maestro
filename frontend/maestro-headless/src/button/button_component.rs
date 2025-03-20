use {crate::hooks::use_interaction_state, dioxus::prelude::*};

#[derive(Props, PartialEq, Debug, Clone)]
pub struct ButtonProps {
	#[props(default = ReadOnlySignal::new(Signal::new(false)))]
	pub pending: ReadOnlySignal<bool>,
	#[props(default = ReadOnlySignal::new(Signal::new(false)))]
	pub disabled: ReadOnlySignal<bool>,

	#[props(default = None)]
	pub onclick: Option<EventHandler<Event<MouseData>>>,
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
	#[props(optional, default = None)]
	pub onmounted: Option<EventHandler<Event<MountedData>>>,

	#[props(extends = GlobalAttributes, extends = button)]
	pub attributes: Vec<Attribute>,
	#[props(default = Vec::new())]
	pub extra_attributes: Vec<Attribute>,
	#[props(default = None)]
	pub children: Element,
}

#[component]
pub fn Button(props: ButtonProps) -> Element {
	let ButtonProps { pending, disabled, .. } = props;
	let mut interaction_state = use_interaction_state();
	let mut is_pressed = use_signal(|| false);
	rsx! {
		button {
			disabled: disabled(),
			aria_disabled: disabled() || pending(),
			"data-disabled": disabled(),
			"data-pressed": is_pressed(),
			"data-hovered": *interaction_state.is_hovered.read(),
			"data-focused": *interaction_state.is_focused.read(),
			"data-focuse-visible": *interaction_state.is_focused.read(),
			"data-pending": pending(),
			onclick: move |event| {
					if !disabled() && !pending() {
							if let Some(handler) = props.onclick {
									handler.call(event);
							}
					}
			},
			onmounted: move |event| {
					interaction_state.self_ref.set(Some(event.clone()));
					if let Some(handler) = props.onmounted {
							handler.call(event);
					}
			},
			onmousedown: move |event| {
					if !disabled() && !pending() {
							is_pressed.set(true);
							if let Some(handler) = props.onmousedown {
									handler.call(event);
							}
					}
			},
			onkeydown: move |event| {
					if !disabled() && !pending() {
							is_pressed.set(false);
							if let Some(handler) = props.onkeydown {
									handler.call(event);
							}
					}
			},

			onkeyup: move |event| {
					if !disabled() && !pending() {
							is_pressed.set(false);
							if let Some(handler) = props.onkeyup {
									handler.call(event);
							}
					}
			},
			onmouseup: move |event| {
					if !disabled() && !pending() {
							is_pressed.set(false);
							if let Some(handler) = props.onmouseup {
									handler.call(event);
							}
					}
			},

			onmouseenter: move |event| {
					if !disabled() && !pending() {
							interaction_state.onmouseenter();
							if let Some(handler) = props.onmouseenter {
									handler.call(event);
							}
					}
			},
			onmouseleave: move |event| {
					if !disabled() && !pending() {
							interaction_state.onmouseleave();
							if let Some(handler) = props.onmouseleave {
									handler.call(event);
							}
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
			..props.attributes,
			..props.extra_attributes,
			{props.children}
		}
	}
}
