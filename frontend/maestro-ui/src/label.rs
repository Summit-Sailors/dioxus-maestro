use {dioxus::prelude::*, tailwind_fuse::*};

#[derive(TwClass)]
#[tw(class = "flex w-full text-md relative flex-col gap-2")]
pub struct LabelClass {}

#[derive(Clone, PartialEq, Props)]
pub struct LabelProps {
	#[props(default = false)]
	pub class: Option<String>,
	pub style: Option<String>,
	pub text: Option<String>,
	pub children: Element,
}

#[component]
pub fn Label(props: LabelProps) -> Element {
	let class = LabelClass {}.with_class(props.class.unwrap_or_default());

	rsx! {
		label { class, style: props.style.unwrap_or_default(),
			{
					match props.text {
							Some(val) => rsx! {
								span { {val} }
							},
							None => rsx! {},
					}
			}
			{props.children}
		}
	}
}
