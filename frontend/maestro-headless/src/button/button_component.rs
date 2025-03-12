use {
	crate::button::use_button::{ButtonContext, use_button},
	dioxus::prelude::*,
};

#[derive(Props, PartialEq, Debug, Clone)]
pub struct ButtonProps {
	#[props(default = Signal::new(false))]
	pub pending: Signal<bool>,
	#[props(default = Signal::new(false))]
	pub disabled: Signal<bool>,
	#[props(extends = GlobalAttributes, extends = button)]
	pub attributes: Vec<Attribute>,
	#[props(default = Vec::new())]
	pub additional_attributes: Vec<Attribute>,
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
	#[props(default = None)]
	pub children: Element,
	#[props(default = None)]
	context: Option<ButtonContext>, /* allaws to use this components in wrapper: use button's context in wrapper and pass default attrs, or use raw button
	                                 * component */
}

#[component]
pub fn Button(props: ButtonProps) -> Element {
	let ButtonProps { pending, disabled, context, .. } = props;
	let mut x_button = context.unwrap_or(use_button(pending, disabled));

	rsx! {
		button {
			disabled: "{*x_button.disabled.read()}",
			onclick: move |event| {
					if x_button.is_allowed() {
							if let Some(handler) = props.onclick {
									handler.call(event);
							}
					}
			},
			onmounted: move |event| {
					x_button.self_ref.set(Some(event.clone()));
					if let Some(handler) = props.onmounted {
							handler.call(event);
					}
			},
			onmousedown: move |event| {
					x_button.onmousedown();
					if let Some(handler) = props.onmousedown {
							handler.call(event);
					}
			},
			onkeydown: move |event| {
					x_button.onkeydown();
					if let Some(handler) = props.onkeydown {
							handler.call(event);
					}
			},
			onkeyup: move |event| {
					x_button.onkeyup();
					if let Some(handler) = props.onkeyup {
							handler.call(event);
					}
			},
			onmouseup: move |event| {
					x_button.onmouseup();
					if let Some(handler) = props.onmouseup {
							handler.call(event);
					}
			},
			onmouseenter: move |event| {
					x_button.onmouseenter();
					if let Some(handler) = props.onmouseenter {
							handler.call(event);
					}
			},
			onmouseleave: move |event| {
					x_button.onmouseleave();
					if let Some(handler) = props.onmouseleave {
							handler.call(event);
					}
			},
			onfocus: move |event| {
					x_button.onfocus();
					if let Some(handler) = props.onfocus {
							handler.call(event);
					}
			},
			onblur: move |event| {
					x_button.onblur();
					if let Some(handler) = props.onblur {
							handler.call(event);
					}
			},
			aria_disabled: "{!x_button.is_allowed()}",
			"data-disabled": *x_button.disabled.read(),
			"data-pressed": *x_button.is_pressed.read(),
			"data-hovered": *x_button.is_hovered.read(),
			"data-focused": *x_button.is_focused.read(),
			"data-focuse-visible": *x_button.is_focused.read(),
			"data-pending": *x_button.pending.read(),
			..props.attributes,
			..props.additional_attributes,
			{props.children}
		}
	}
}
