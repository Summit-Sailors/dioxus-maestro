use {crate::label::Label, dioxus::prelude::*, tailwind_fuse::*};

#[derive(TwClass)]
#[tw(
	class = "flex items-center justify-center w-6 h-6 transition-all ease-linear border rounded-full bg-transparent border-input group-hover:border-primary shrink-0"
)]
pub struct RadioClass {}

#[derive(Clone, PartialEq, Props)]
pub struct RadioProps {
	#[props(default = false)]
	pub disabled: bool,
	#[props(default = false)]
	pub checked: bool,
	pub on_change: Option<EventHandler>,
	pub id: Option<String>,
	pub class: Option<String>,
	pub style: Option<String>,
	pub label: String,
	pub name: String,
	pub label_class: Option<String>,
}

#[component]
pub fn Radio(props: RadioProps) -> Element {
	let class = RadioClass {}.with_class(props.class.unwrap_or_default());

	rsx! {
		Label {
			class: tw_join!(
					"flex-row-reverse items-center relative cursor-pointer group", props.label_class
					.clone().unwrap_or_default(), (props.disabled)
					.then_some("pointer-events-none opacity-50 cursor-not-allowed")
			),
			label_text: props.label,
			input {
				class: "absolute w-0 h-0 opacity-0 -z-20",
				r#type: "radio",
				checked: props.checked,
				onchange: move |_| props.on_change.unwrap_or_default().call(()),
				name: props.name,
			}
			div { class,
				div {
					class: tw_join!(
							"w-2 h-2 rounded-full transition-all ease-linear border border-input", if props
							.checked {
							"opacity-100 border-input bg-input group-hover:bg-primary group-hover:border-primary"
							} else { "opacity-0" }
					),
				}
			}
		}
	}
}
