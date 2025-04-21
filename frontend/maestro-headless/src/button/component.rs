use dioxus::prelude::*;

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
	let ButtonProps {
		pending,
		disabled,
		onclick,
		onkeydown,
		onkeyup,
		onfocus,
		onblur,
		onmousedown,
		onmouseup,
		onmouseenter,
		onmouseleave,
		onmounted,
		attributes,
		extra_attributes,
		children,
	} = props;

	let mut is_pressed = use_signal(|| false);

	rsx! {
		button {
			disabled: (pending() || disabled()).then_some(true),
			aria_disabled: (disabled() || pending()).then_some(Some(true)),
			"data-disabled": disabled().then_some(Some(true)),
			"data-pressed": is_pressed().then_some(Some(true)),
			"data-pending": pending().then_some(Some(true)),
			pointer_events: (pending() || disabled()).then_some(Some("none")),
			cursor: "pointer",
			onclick: move |event| {
					if !disabled() && !pending() {
							if let Some(handler) = onclick {
									handler.call(event);
							}
					}
			},
			onmounted: move |event| {
					if let Some(handler) = onmounted {
							handler.call(event);
					}
			},
			onmousedown: move |event| {
					if !disabled() && !pending() {
							is_pressed.set(true);
							if let Some(handler) = onmousedown {
									handler.call(event);
							}
					}
			},
			onkeydown: move |event| {
					if !disabled() && !pending() {
							is_pressed.set(false);
							if let Some(handler) = onkeydown {
									handler.call(event);
							}
					}
			},

			onkeyup: move |event| {
					if !disabled() && !pending() {
							is_pressed.set(false);
							if let Some(handler) = onkeyup {
									handler.call(event);
							}
					}
			},
			onmouseup: move |event| {
					if !disabled() && !pending() {
							is_pressed.set(false);
							if let Some(handler) = onmouseup {
									handler.call(event);
							}
					}
			},
			onmouseenter: move |event| {
					if !disabled() && !pending() {
							if let Some(handler) = onmouseenter {
									handler.call(event);
							}
					}
			},
			onmouseleave: move |event| {
					if !disabled() && !pending() {
							if let Some(handler) = onmouseleave {
									handler.call(event);
							}
					}
			},
			onfocus: move |event| {
					if let Some(handler) = onfocus {
							handler.call(event);
					}
			},
			onblur: move |event| {
					if let Some(handler) = onblur {
							handler.call(event);
					}
			},
			..attributes,
			..extra_attributes,
			{children}
		}
	}
}
