use dioxus::prelude::*;
use tailwind_fuse::*;

use crate::input::InputVariant;

#[derive(TwClass)]
#[tw(
	class = "ease-linear w-full bg-transparent border-gray-700 placeholder:text-[color:var(--text-color)]500 focus-visible:outline-none focus-visible:ring-0 disabled:cursor-not-allowed disabled:opacity-50 disabled:pointer-events-none transition-colors resize-none max-h-48 min-h-11 n-scrollbar maestro-calendar-container"
)]
pub struct TextareaClass {
	pub variant: InputVariant,
}

#[derive(Clone, PartialEq, Props)]
pub struct TextareaProps {
	#[props(default = InputVariant::Default)]
	pub variant: InputVariant,
	pub onchange: Option<EventHandler<Event<FormData>>>,
	pub onenter: Option<EventHandler<Event<KeyboardData>>>,
	pub onfocus: Option<EventHandler<Event<FocusData>>>,
	pub onblur: Option<EventHandler<Event<FocusData>>>,
	#[props(default = String::new())]
	pub class: String,
	#[props(default = String::new())]
	pub wrapper_class: String,
	pub style: Option<String>,
	#[props(default = None)]
	pub children: Element,
	#[props(extends = GlobalAttributes, extends = textarea)]
	pub attributes: Vec<Attribute>,
}

// classes may be extended also by using "maestro-textarea__*" classname

#[component]
pub fn Textarea(props: TextareaProps) -> Element {
	let class = TextareaClass { variant: props.variant }.with_class(tw_merge!(&props.class, "maestro-textarea__textarea", {
		match props.variant {
			InputVariant::Default => "maestro-variant-default",
			InputVariant::Underlined => "maestro-variant-underlined",
		}
	}));

	rsx! {
		div {
			class: tw_merge!(
					"flex w-full relative", & props.wrapper_class, "maestro-textarea__wrapper"
			),
			textarea {
				class,
				style: props.style.unwrap_or_default(),
				oninput: move |event| {
						props.onchange.unwrap_or_default().call(event);
				},
				onfocus: move |event| {
						props.onfocus.unwrap_or_default().call(event);
				},
				onblur: move |event| {
						props.onblur.unwrap_or_default().call(event);
				},
				onkeypress: move |event| {
						if event.data().code() == Code::Enter {
								props.onenter.unwrap_or_default().call(event);
						}
				},
				..props.attributes,
			}
			{props.children}
		}
	}
}
