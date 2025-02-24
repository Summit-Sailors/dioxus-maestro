use {
	crate::label::Label,
	dioxus::prelude::*,
	dioxus_free_icons::{icons::io_icons::IoCheckmarkOutline, Icon},
	tailwind_fuse::*,
};

#[derive(TwClass)]
#[tw(class = "flex items-center justify-center w-6 h-6 transition-all ease-linear border border-gray-700 rounded bg-transparent shrink-0")]
pub struct CheckboxClass {}

#[derive(Clone, PartialEq, Props)]
pub struct CheckboxProps {
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

// classes may be extended also by using "maestro-checkbox__*" classname

#[component]
pub fn Checkbox(props: CheckboxProps) -> Element {
	let class = CheckboxClass {}.with_class(tw_merge!(props.class.clone().unwrap_or_default(), "maestro-checkbox__checkbox"));

	rsx! {
		Label {
			class: tw_join!(
					"flex-row-reverse items-center relative cursor-pointer group", props.label_class
					.clone().unwrap_or_default(), "maestro-checkbox__label", (props.disabled)
					.then_some("pointer-events-none opacity-50 cursor-not-allowed")
			),
			text: props.label,
			input {
				class: "absolute w-0 h-0 opacity-0 -z-20",
				r#type: "checkbox",
				checked: props.checked,
				onchange: move |event| props.onchange.unwrap_or_default().call(event),
				..props.attributes,
			}
			div { class,
				Icon {
					width: 8,
					height: 8,
					icon: IoCheckmarkOutline {},
					class: tw_merge!(
							"fill-none w-2 h-2 transition-all ease-linear text-gray-700 shrink-0", props
							.inner_class.clone().unwrap_or_default(), if props.checked {
							format!("opacity-100 group-hover:text-gray-900 {} {}", props.checked_class
							.clone().unwrap_or_default(), "maestro-checkbox__checked") } else { "opacity-0"
							.into() }
					),
				}
			}
		}
	}
}
