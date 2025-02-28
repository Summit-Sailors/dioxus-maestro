use dioxus::prelude::*;

#[derive(Clone, PartialEq, Props)]
pub struct ComponentSectionProps {
	pub title: &'static str,
	pub description: &'static str,
	pub children: Element,
}

#[component]
pub fn ComponentSection(props: ComponentSectionProps) -> Element {
	rsx! {
    section { class: "mb-12 text-gray-800 text-center w-full",
      h2 { class: "text-2xl font-semibold mb-2 dark:text-gray-50", {props.title} }
      p { class: "text-gray-600 dark:text-gray-400 mb-6", {props.description} }
      div { class: "bg-gray-200 p-4 rounded-lg shadow-sm border", {props.children} }
    }
  }
}
