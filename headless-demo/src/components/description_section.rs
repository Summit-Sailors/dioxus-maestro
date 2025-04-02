use {dioxus::prelude::*, tailwind_fuse::tw_merge};

#[derive(Clone, PartialEq, Props)]
pub struct DescriptionSectionProps {
	pub title: String,
	#[props(optional, default = String::new())]
	pub description: String,
	#[props(default = String::new())]
	pub class: String,
	#[props(optional)]
	pub children: Element,
}

#[component]
pub fn DescriptionSection(props: DescriptionSectionProps) -> Element {
	let DescriptionSectionProps { title, description, class, children } = props;

	rsx! {
		section { class: tw_merge!("container flex flex-col gap-3 lg:py-6 py-4", class),
			h3 { class: "lg:text-xl text-lg font-medium text-orange-600", "{title}" }
			if !description.is_empty() {
				p { class: "text-neutral-300", "{description}" }
			}
			{children}
		}
	}
}
