// For visual theme customization

mod color_picker;
mod font_selector;
mod preview;
mod spacing_editor;
mod state;

pub use {
	crate::theme::types::Theme,
	color_picker::ColorPicker,
	dioxus::prelude::*,
	font_selector::FontSelector,
	preview::ThemePreview,
	spacing_editor::SpacingEditor,
	state::{DesignerState, ThemedesignerAction},
};

#[derive(Props, PartialEq, Clone)]
pub struct ThemeDesignerProps {
	#[props(optional)]
	pub on_export: Option<EventHandler<DesignerState>>,
	#[props(optional)]
	pub initial_state: Option<DesignerState>,
}

#[component]
pub fn ThemeDesigner(props: ThemeDesignerProps) -> Element {
	let initial_state = props.initial_state.clone().unwrap_or_default();
	let state = use_signal(|| initial_state);

	let export_theme = move |_| {
		if let Some(on_export) = &props.on_export {
			on_export.call(state());
		}
	};

	rsx! {
		div { id: "theme-designer", class: "theme-designer-container",
			div { class: "theme-designer-sidebar",
				h2 { class: "text-xl font-bold mb-4", "Theme Customization" }
			}
		}
	}
}
