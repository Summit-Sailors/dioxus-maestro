// Border radius editor component

use dioxus::prelude::*;

use crate::designer::state::BorderRadiusSettings;

#[derive(Props, PartialEq, Clone)]
pub struct BorderRadiusEditorProps {
	pub border_radius: BorderRadiusSettings,
	pub on_change: EventHandler<BorderRadiusSettings>,
}

#[component]
pub fn BorderRadiusEditor(props: BorderRadiusEditorProps) -> Element {
	let border_radius = props.border_radius.clone();

	let make_handler = |key: &'static str| {
		let on_change = props.on_change.clone();
		let base_border_radius = props.border_radius.clone();

		move |event: Event<FormData>| {
			let mut new_border_radius = base_border_radius.clone();
			match key {
				"sm" => new_border_radius.sm = event.value().clone(),
				"md" => new_border_radius.md = event.value().clone(),
				"lg" => new_border_radius.lg = event.value().clone(),
				"xl" => new_border_radius.xl = event.value().clone(),
				"full" => new_border_radius.full = event.value().clone(),
				_ => {},
			}
			on_change.call(new_border_radius);
		}
	};

	let sm_handler = make_handler("sm");
	let md_handler = make_handler("md");
	let lg_handler = make_handler("lg");
	let xl_handler = make_handler("xl");
	let full_handler = make_handler("full");

	rsx! {
		div { class: "border-radius-editor",
			h3 { class: "text-lg font-medium mb-3", "Border Radius" }
			div { class: "grid grid-cols-2 gap-4",
				div {
					label { class: "block text-sm font-medium mb-1", "Small (sm)" }
					div { class: "flex",
						input {
							r#type: "text",
							class: "flex-grow border rounded px-2 py-1",
							value: "{border_radius.sm}",
							oninput: move |event| sm_handler(event),
						}
						div { class: "border rounded ml-2 w-10 h-10 overflow-hidden",
							div {
								class: "w-full h-full bg-gray-200",
								style: "border-radius: {border_radius.sm};",
							}
						}
					}
				}
				div {
					label { class: "block text-sm font-medium mb-1", "Medium (md)" }
					div { class: "flex",
						input {
							r#type: "text",
							class: "flex-grow border rounded px-2 py-1",
							value: "{border_radius.md}",
							oninput: move |event| md_handler(event),
						}
						div { class: "border rounded ml-2 w-10 h-10 overflow-hidden",
							div {
								class: "w-full h-full bg-gray-200",
								style: "border-radius: {border_radius.md};",
							}
						}
					}
				}
				div {
					label { class: "block text-sm font-medium mb-1", "Large (lg)" }
					div { class: "flex",
						input {
							r#type: "text",
							class: "flex-grow border rounded px-2 py-1",
							value: "{border_radius.lg}",
							oninput: move |event| lg_handler(event),
						}
						div { class: "border rounded ml-2 w-10 h-10 overflow-hidden",
							div {
								class: "w-full h-full bg-gray-200",
								style: "border-radius: {border_radius.lg};",
							}
						}
					}
				}
				div {
					label { class: "block text-sm font-medium mb-1", "Extra Large (xl)" }
					div { class: "flex",
						input {
							r#type: "text",
							class: "flex-grow border rounded px-2 py-1",
							value: "{border_radius.xl}",
							oninput: move |event| xl_handler(event),
						}
						div { class: "border rounded ml-2 w-10 h-10 overflow-hidden",
							div {
								class: "w-full h-full bg-gray-200",
								style: "border-radius: {border_radius.xl};",
							}
						}
					}
				}
				div {
					label { class: "block text-sm font-medium mb-1", "Full" }
					div { class: "flex",
						input {
							r#type: "text",
							class: "flex-grow border rounded px-2 py-1",
							value: "{border_radius.full}",
							oninput: move |event| full_handler(event),
						}
						div { class: "border rounded ml-2 w-10 h-10 overflow-hidden",
							div {
								class: "w-full h-full bg-gray-200",
								style: "border-radius: {border_radius.full};",
							}
						}
					}
				}
			}
		}
	}
}
