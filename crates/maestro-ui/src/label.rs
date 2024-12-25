use dioxus::prelude::*;
use tailwind_fuse::*;

#[derive(TwClass)]
#[tw(class = "flex w-full text-md relative flex-col gap-2")]
pub struct LabelClass {}

#[derive(PartialEq, Clone, Props)]
pub struct LabelProps {
	#[props(default = false)]
	pub id: Option<String>,
	pub class: Option<String>,
	pub style: Option<String>,
	pub label_text: Option<String>,
	pub children: Element,
}

#[component]
pub fn Label(props: LabelProps) -> Element {
	let class = LabelClass {}.with_class(props.class.unwrap_or_default());

	rsx! {
		label {
			id: props.id.unwrap_or_default(),
			class,
			style: props.style.unwrap_or_default(),
			{ match props.label_text {
				Some(val) => rsx! {span {{val}}},
				None => rsx! {}
			}
			},
			{props.children}
		}
	}
}
