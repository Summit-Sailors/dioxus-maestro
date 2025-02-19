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

// classes may be extended also by using "maestro-label" classname

#[component]
pub fn Label(props: LabelProps) -> Element {
	let class = LabelClass {}.with_class(tw_merge!(props.class.clone().unwrap_or_default(), "maestro_label"));

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
