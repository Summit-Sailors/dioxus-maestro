use dioxus::prelude::*;
use tailwind_fuse::*;

#[derive(TwClass)]
#[tw(
	class = "flex h-10 w-full bg-transparent py-2 text-sm file:border-0 file:bg-transparent file:text-sm file:font-medium placeholder:text-gray-500 border-gray-700 ring-offset-white ring-gray-700 focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-offset-2  disabled:cursor-not-allowed disabled:opacity-50 maestro-input__input"
)]
pub struct InputClass {
	pub variant: InputVariant,
}

#[derive(PartialEq, TwVariant)]
pub enum InputVariant {
	#[tw(default, class = "variant-default")]
	Default,
	#[tw(class = "variant-underlined")]
	Underlined,
}

#[derive(Clone, PartialEq, Props)]
pub struct InputProps {
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
	#[props(extends = GlobalAttributes, extends = input)]
	pub attributes: Vec<Attribute>,
}

// classes may be extended also by using "maestro-input__*" classname

#[component]
pub fn Input(props: InputProps) -> Element {
	let class = InputClass { variant: props.variant }.with_class(tw_merge!(&props.class, "maestro-input__input"));

	rsx! {
		div { class: tw_merge!("w-full relative", & props.wrapper_class, "maestro-input__wrapper"),
			input {
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
