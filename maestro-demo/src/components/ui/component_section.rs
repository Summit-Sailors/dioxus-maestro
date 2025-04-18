use dioxus::prelude::*;

#[derive(Clone, PartialEq, Props)]
pub struct ComponentSectionProps {
	pub title: &'static str,
	pub description: &'static str,
	pub children: Element,
	pub extra_styles: Option<String>,
}

#[component]
pub fn ComponentSection(props: ComponentSectionProps) -> Element {
	rsx! {
		section { class: "mb-12 text-slate-200 text-center w-full",
			h2 { class: "text-2xl font-semibold mb-2 text-slate-100", {props.title} }
			p { class: "text-slate-400 mb-6", {props.description} }
			div { class: "bg-slate-900 p-4 mt-4 rounded-lg shadow-sm border border-b-8 border-b-blue-100 border-slate-800",
				{props.children}
			}
		}
	}
}
