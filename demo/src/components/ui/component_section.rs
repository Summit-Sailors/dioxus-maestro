use dioxus::prelude::*;

#[derive(Props, Clone, PartialEq)]
pub struct ComponentSectionProps {
	pub title: &'static str,
	pub description: &'static str,
	pub children: Element,
}

#[component]
pub fn ComponentSection(props: ComponentSectionProps) -> Element {
	rsx! {
    section { class: "mb-12 text-gray-800 text-center w-full bg-gray-900",
      h2 { class: "text-2xl font-semibold mb-2 dark:text-gray-50", {props.title} }
      p { class: "text-gray-600 dark:text-gray-400 mb-6", {props.description} }
      div { class: "bg-gray-900 p-4 rounded-lg shadow-sm border border-gray-800", {props.children} }
    }
  }
}
