// Theme preview component
use dioxus::prelude::*;

use crate::components::maestro_themes::{
	designer::{state::DesignerState, theme_components::components_library::get_components_section},
	exporter::{ExportFormat, ThemeOptions, export_theme},
	theme::prelude::{ThemeProvider, ThemeSelect},
};

#[derive(Props, PartialEq, Clone)]
pub struct ThemePreviewProps {
	state: DesignerState,
	with_doc_theme: bool,
	components_section_id: String,
}

#[component]
pub fn ThemePreview(props: ThemePreviewProps) -> Element {
	let state = props.state.clone();
	let components_id_clone = props.components_section_id.clone();
	let theme_options = ThemeOptions { with_doc_themes: false, format: ExportFormat::CSSVariable, components_id: props.components_section_id };

	// UI components being shoucased
	let content = if props.with_doc_theme {
		rsx! {
			ThemeProvider { default_theme: state.clone().doc_theme,
				div { class: "preview-container",
					nav { class: "mt-4", ThemeSelect {} }
					{get_components_section(&components_id_clone)}
				}
			}
		}
	} else {
		rsx! {
			div { class: "preview-container", {get_components_section(&components_id_clone)} }
		}
	};

	// Generate CSS variables
	let css_variables = export_theme(&state, &theme_options);

	rsx! {
		div { class: "theme-preview-container",
			main {
				style { "{css_variables}" }
				{content}
			}
		}
	}
}
