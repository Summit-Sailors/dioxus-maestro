pub mod use_button;

use {
	crate::button::use_button::{use_button, UseButton},
	dioxus::{prelude::*, web::WebEventExt},
};

#[derive(Props, PartialEq, Debug, Clone)]
pub struct ButtonProps {
	#[props(default = false)]
	pub pending: bool,
	#[props(default = false)]
	pub disabled: bool,
	#[props(extends = button, extends = GlobalAttributes)]
	pub attributes: Vec<Attribute>,
	#[props(default = None)]
	pub onclick: Option<EventHandler<Event<MouseData>>>,
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
	pub onmounted: Option<EventHandler<Option<web_sys::Element>>>,
	#[props(default = None)]
	pub children: Element,
	#[props(default = None)]
	context: Option<UseButton>, // allaws to use this components in wrapper, use button's context in wrapper and pass default attrs, or use raw button component
}

#[component]
pub fn Button(props: ButtonProps) -> Element {
	let ButtonProps { pending, disabled, context, onclick, .. } = props;
	let mut x_button = context.unwrap_or(use_button(pending, disabled));

	let handle_click = move |event| {
		if x_button.is_allowed() {
			onclick.unwrap_or_default().call(event);
		}
	};

	rsx! {
		button {
			disabled: "{*x_button.disabled.read()}",
			onclick: handle_click,
			onmounted: move |event| {
					if let Some(element) = event.try_as_web_event() {
							x_button.self_ref.set(Some(element));
					}
					props.onmounted.unwrap_or_default().call(event.try_as_web_event());
			},
			onmousedown: move |event| {
					x_button.onmousedown();
					props.onmousedown.unwrap_or_default().call(event.clone());
			},
			onmouseup: move |event| {
					x_button.onmouseup();
					props.onmouseup.unwrap_or_default().call(event.clone());
			},
			onmouseenter: move |event| {
					x_button.onmouseenter();
					props.onmouseenter.unwrap_or_default().call(event);
			},
			onmouseleave: move |event| {
					x_button.onmouseleave();
					props.onmouseleave.unwrap_or_default().call(event);
			},
			onfocus: move |event| {
					x_button.onfocus();
					props.onfocus.unwrap_or_default().call(event);
			},
			onblur: move |event| {
					x_button.onblur();
					props.onblur.unwrap_or_default().call(event);
			},
			aria_disabled: "{!x_button.is_allowed()}",
			"data-disabled": *x_button.disabled.read(),
			"data-pressed": *x_button.is_pressed.read(),
			"data-hovered": *x_button.is_hovered.read(),
			"data-focused": *x_button.is_focused.read(),
			"data-focuse-visible": *x_button.is_focused.read(),
			"data-pending": *x_button.pending.read(),
			..props.attributes,
			{props.children}
		}
	}
}
