use dioxus::prelude::*;
use tailwind_fuse::*;

use crate::label::Label;

#[derive(TwClass)]
#[tw(
	class = "flex items-center justify-center w-6 h-6 transition-all ease-linear border border-gray-700 rounded-full bg-transparent shrink-0 ring-offset-white ring-gray-700 focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-offset-2"
)]
pub struct RadioClass {}

#[derive(Clone, PartialEq, Props)]
pub struct RadioProps {
	#[props(default = false)]
	pub disabled: bool,
	#[props(default = false)]
	pub checked: bool,
	pub onchange: Option<EventHandler<Event<FormData>>>,
	#[props(default = String::new())]
	pub class: String,
	pub style: Option<String>,
	pub label: String,
	#[props(default = String::new())]
	pub label_class: String,
	#[props(default = String::new())]
	pub inner_class: String,
	#[props(default = String::new())]
	pub checked_class: String,
	#[props(extends = GlobalAttributes, extends = input)]
	pub attributes: Vec<Attribute>,
}

// classes may be extended also by using "maestro-radio__*" classname

#[component]
pub fn Radio(props: RadioProps) -> Element {
	let class = RadioClass {}.with_class(tw_merge!(&props.class, "maestro-radio__radio"));

	rsx! {
		Label {
			class: tw_merge!(
					"flex-row-reverse items-center relative cursor-pointer group", & props
					.label_class, "maestro-radio__label", (props.disabled)
					.then_some("pointer-events-none opacity-50 cursor-not-allowed")
			),
			text: props.label,
			input {
				class: "absolute w-0 h-0 opacity-0 -z-20",
				r#type: "radio",
				checked: props.checked,
				onchange: move |event| props.onchange.unwrap_or_default().call(event),
				..props.attributes,
			}
			div { class, tabindex: 0,
				div {
					class: tw_merge!(
							"w-2 h-2 rounded-full transition-all ease-linear border border-gray-700", & props
							.inner_class, if props.checked {
							format!("opacity-100 border border-gray-700 bg-gray-700 group-hover:bg-gray-900 group-hover:border-gray-900 {} {}",
							& props.checked_class, "maestro-radio__checked") } else { "opacity-0".into() }
					),
				}
			}
		}
	}
}
