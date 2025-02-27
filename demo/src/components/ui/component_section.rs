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
    section { class: "dark mb-12 dark:text-gray-200 text-center w-full",
      h2 { class: "text-2xl font-semibold mb-2 dark:text-gray-100", {props.title} }
      p { class: "text-gray-700 dark:text-gray-400 mb-6", {props.description} }
      div { class: "bg-gray-900 p-4 mt-4 rounded-lg shadow-sm border border-b-8 border-b-blue-100 border-gray-800", {props.children} }
    }
  }
}
