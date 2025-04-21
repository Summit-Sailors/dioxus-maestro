use dioxus::prelude::*;

use crate::router::Route;

#[derive(Clone, PartialEq, Props)]
pub struct ComponentSectionProps {
	pub title: &'static str,
	pub description: &'static str,
	pub children: Element,
	#[props(optional)]
	pub section_id: Option<String>,
}

#[component]
pub fn ComponentSection(props: ComponentSectionProps) -> Element {
	let navigator = use_navigator();

	let handle_theme_designer_click = {
		let section_id_clone = props.section_id.clone();
		move |_| {
			if let Some(id) = &section_id_clone {
				navigator.push(Route::ThemeDesigner { components_id: id.clone() });
			}
		}
	};

	rsx! {
		section { class: "mb-12 text-slate-200 text-center w-full",
			div { class: "flex justify-between items-center mb-2",
				h2 { class: "text-2xl font-semibold text-slate-100", {props.title} }
				{
						if props.section_id.is_some() {
								rsx! {
									button {
										class: "px-3 py-1 bg-blue-600 hover:bg-blue-700 text-white rounded-md flex items-center gap-2 transition-colors",
										onclick: handle_theme_designer_click,
										"Customize Theme"
									}
								}
						} else {
								rsx! {}
						}
				}
			}
			p { class: "text-slate-400 mb-6", {props.description} }
			div { class: "bg-slate-900 p-4 mt-4 rounded-lg shadow-sm border border-b-8 border-b-blue-100 border-slate-800",
				{props.children}
			}
		}
	}
}
