use {crate::label::Label, dioxus::prelude::*, tailwind_fuse::*};

#[derive(TwClass)]
#[tw(class = "flex items-center justify-center w-6 h-6 transition-all ease-linear border border-gray-700 rounded-full bg-transparent shrink-0")]
pub struct RadioClass {}

#[derive(Clone, PartialEq, Props)]
pub struct RadioProps {
	#[props(default = false)]
	pub disabled: bool,
	#[props(default = false)]
	pub checked: bool,
	pub onchange: Option<EventHandler<Event<FormData>>>,
	pub class: Option<String>,
	pub style: Option<String>,
	pub label: String,
	pub label_class: Option<String>,
	pub inner_class: Option<String>,
	pub checked_class: Option<String>,
	#[props(extends = GlobalAttributes, extends = input)]
	pub attributes: Vec<Attribute>,
}

// classes may be extended also by using "maestro-radio__*" classname

#[component]
pub fn Radio(props: RadioProps) -> Element {
	let class = RadioClass {}.with_class(tw_merge!(props.class.clone().unwrap_or_default(), "maestro-radio__radio"));

	rsx! {
		Label {
			class: tw_join!(
					"flex-row-reverse items-center relative cursor-pointer group", props.label_class
					.clone().unwrap_or_default(), "maestro-radio__label", (props.disabled)
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
			div { class,
				div {
					class: tw_join!(
							"w-2 h-2 rounded-full transition-all ease-linear border border-gray-700", props
							.inner_class.clone().unwrap_or_default(), if props.checked {
							format!("opacity-100 border border-gray-700 bg-gray-700 group-hover:bg-gray-900 group-hover:border-gray-900 {} {}",
							props.checked_class.clone().unwrap_or_default(), "maestro-radio__checked") } else
							{ "opacity-0".into() }
					),
				}
			}
		}
	}
}
