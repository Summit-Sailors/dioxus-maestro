// Theme preview component
use dioxus::prelude::*;

use crate::components::maestro_themes::{
	designer::DesignerState,
	exporter::{ExportFormat, ThemeOptions, export_theme},
	theme::prelude::ThemeProvider,
};

#[derive(Props, PartialEq, Clone)]
pub struct ThemePreviewProps {
	state: DesignerState,
	with_doc_theme: bool,
	component_section: Element,
}

#[component]
pub fn ThemePreview(props: ThemePreviewProps) -> Element {
	let state = props.state.clone();
	let theme_options = ThemeOptions { with_doc_themes: false, format: ExportFormat::CSSVariable };

	// UI components being shoucased
	let content = if props.with_doc_theme {
		rsx! {
			ThemeProvider { default_theme: state.clone().doc_theme,
				div { class: "preview-container", {props.component_section} }
			}
		}
	} else {
		rsx! {
			div { class: "preview-container", {props.component_section} }
		}
	};

	// Generate CSS variables
	let css_variables = export_theme(&state, &theme_options);

	rsx! {
		div { class: "theme-preview-container",
			style { "{css_variables}" }
			{content}
		}
	}
}

// TODO: A way to render a component sections conditionally based on what the user chooses for the theming process. (need a way to navigate between component
// sections)
