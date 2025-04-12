// Shadow settings editor component
use {crate::designer::state::ShadowSettings, dioxus::prelude::*};

#[derive(Props, PartialEq, Clone)]
pub struct ShadowEditorProps {
	pub shadow: ShadowSettings,
	pub on_change: EventHandler<ShadowSettings>,
}

#[component]
pub fn ShadowEditor(props: ShadowEditorProps) -> Element {
	let shadow = props.shadow.clone();

	let update_shadow = move |key: &'static str, value: String| {
		let mut new_shadow = props.shadow.clone();
		match key {
			"sm" => new_shadow.sm = value,
			"md" => new_shadow.md = value,
			"lg" => new_shadow.lg = value,
			"xl" => new_shadow.xl = value,
			"xxl" => new_shadow.xxl = value,
			_ => {},
		}
		props.on_change.call(new_shadow);
	};

	rsx! {
		div { class: "shadow-editor",
			h3 { class: "text-lg font-medium mb-3", "Shadows" }
			div { class: "shadow-preview-grid space-y-4",
				div {
					label { class: "block text-sm font-medium mb-1", "Small (sm)" }
					div { class: "flex items-center gap-4",
						div {
							class: "shadow-preview w-16 h-16 bg-white rounded",
							style: "box-shadow: {shadow.sm};",
						}
						textarea {
							class: "flex-grow border rounded px-2 py-1 text-sm",
							rows: "2",
							value: "{shadow.sm}",
							oninput: move |event| update_shadow("sm", event.value().clone()),
						}
					}
				}
				div {
					label { class: "block text-sm font-medium mb-1", "Medium (md)" }
					div { class: "flex items-center gap-4",
						div {
							class: "shadow-preview w-16 h-16 bg-white rounded",
							style: "box-shadow: {shadow.md};",
						}
						textarea {
							class: "flex-grow border rounded px-2 py-1 text-sm",
							rows: "2",
							value: "{shadow.md}",
							oninput: move |event| update_shadow("md", event.value().clone()),
						}
					}
				}
				div {
					label { class: "block text-sm font-medium mb-1", "Large (lg)" }
					div { class: "flex items-center gap-4",
						div {
							class: "shadow-preview w-16 h-16 bg-white rounded",
							style: "box-shadow: {shadow.lg};",
						}
						textarea {
							class: "flex-grow border rounded px-2 py-1 text-sm",
							rows: "2",
							value: "{shadow.lg}",
							oninput: move |event| update_shadow("lg", event.value().clone()),
						}
					}
				}
				div {
					label { class: "block text-sm font-medium mb-1", "Extra Large (xl)" }
					div { class: "flex items-center gap-4",
						div {
							class: "shadow-preview w-16 h-16 bg-white rounded",
							style: "box-shadow: {shadow.xl};",
						}
						textarea {
							class: "flex-grow border rounded px-2 py-1 text-sm",
							rows: "2",
							value: "{shadow.xl}",
							oninput: move |event| update_shadow("xl", event.value().clone()),
						}
					}
				}
				div {
					label { class: "block text-sm font-medium mb-1", "Extra Extra Large (xxl)" }
					div { class: "flex items-center gap-4",
						div {
							class: "shadow-preview w-16 h-16 bg-white rounded",
							style: "box-shadow: {shadow.xxl};",
						}
						textarea {
							class: "flex-grow border rounded px-2 py-1 text-sm",
							rows: "2",
							value: "{shadow.xxl}",
							oninput: move |event| update_shadow("xxl", event.value().clone()),
						}
					}
				}
			}
		}
	}
}
