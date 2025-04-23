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
		section { class: "mb-12 text-[color:var(--text-color)] text-center w-full",
			div { class: "flex sm:flex-col justify-between items-center mb-2",
				h2 { class: "text-2xl font-semibold text-[color:var(--text-color)]",
					"{props.title}"
				}
				{
						if props.section_id.is_some() {
								rsx! {
									button {
										class: "px-3 py-1 bg-[color:var(--primary-bg)] hover:bg-[color:oklch(0.52_0.19_263.83)] text-[color:var(--primary-text)] rounded-md flex items-center gap-2 transition-colors",
										onclick: handle_theme_designer_click,
										"Customize Theme"
									}
								}
						} else {
								rsx! {}
						}
				}
			}
			p { class: "text-[color:var(--muted-text)] mb-6", "{props.description}" }
			div { class: "p-4 mt-4 rounded-lg shadow-sm bg-[color:var(--card-bg)] text-[color:var(--card-text)] border border-[color:var(--border-color)]",
				{props.children}
			}
		}
	}
}
