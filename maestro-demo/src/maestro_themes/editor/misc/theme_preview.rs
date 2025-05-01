// Theme preview component
use dioxus::prelude::*;

use crate::maestro_themes::{
	editor::{misc::components_library::get_components_section, state::DesignerState},
	exporter::{ExportFormat, ThemeOptions, export_theme},
};

#[derive(Props, PartialEq, Clone)]
pub struct ThemePreviewProps {
	with_doc_theme: bool,
	components_section_id: String,
}

#[component]
pub fn ThemePreview(props: ThemePreviewProps) -> Element {
	let state = use_context::<Signal<DesignerState>>();
	let components_id_clone = props.components_section_id.clone();
	let theme_options = ThemeOptions { with_doc_themes: false, format: ExportFormat::CSSVariable, components_id: props.components_section_id };

	let mut css_variables = use_signal(|| String::from(""));

	use_effect(move || {
		css_variables.set(export_theme(&state(), &theme_options).replace("@theme {", "").replace("\n", " ").replace("}", ""));
	});

	// UI components being shoucased
	let content = get_components_section(&components_id_clone);

	rsx! {
    div { id: "theme-preview-container", style: "{css_variables()}",
      main { id: "theme-previewer", {content} }
    }
  }
}
