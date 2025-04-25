// Theme preview component
use dioxus::prelude::*;

use crate::components::maestro_themes::{
	designer::{state::DesignerState, theme_components::components_library::get_components_section},
	exporter::{ExportFormat, ThemeOptions, export_theme},
};

#[derive(Props, PartialEq, Clone)]
pub struct ThemePreviewProps {
	state: Signal<DesignerState>,
	with_doc_theme: bool,
	components_section_id: String,
}

#[component]
pub fn ThemePreview(props: ThemePreviewProps) -> Element {
	let state = props.state;
	let components_id_clone = props.components_section_id.clone();
	let theme_options = ThemeOptions { with_doc_themes: false, format: ExportFormat::CSSVariable, components_id: props.components_section_id };

	// UI components being shoucased
	let content = get_components_section(&components_id_clone);

	// Generate CSS variables
	let css_variables = export_theme(&state(), &theme_options).replace("@theme", "#scoped-theme");

	rsx! {
		div { id: "theme-preview-container",
			main { id: "scoped-theme",
				style { "{css_variables}" }
				{content}
			}

		}
	}
}
