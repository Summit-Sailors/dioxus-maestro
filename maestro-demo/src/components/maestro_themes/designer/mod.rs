// For visual theme customization

pub mod state;
mod theme_components;

use dioxus::prelude::*;
use maestro_ui::{
	select::{Select, SelectOption},
	toggle::{EToggleSwitchLabelPlacement, ToggleSwitch, ToggleSwitchLabelStatesProp},
};
pub use state::{BorderRadiusSettings, ColorPalette, DesignerState, ShadowSettings, SpacingScale, TypographySettings};
use strum::IntoEnumIterator;
use tailwind_fuse::tw_merge;
use theme_components::prelude::*;

use crate::components::{
	backdrop::Backdrop,
	maestro_themes::exporter::{ExportFormat, ThemeOptions},
};

#[derive(Props, PartialEq, Clone)]
pub struct ThemeDesignerProps {
	pub initial_state: DesignerState,
	pub components_container: Element,
}

#[component]
pub fn ThemeDesigner(props: ThemeDesignerProps) -> Element {
	let initial_state = props.initial_state.clone();
	let mut state = use_signal(|| initial_state);
	let mut active_tab = use_signal(|| "colors");

	let with_doc_theme = use_signal(|| false);
	let mut export_format = use_signal(|| ExportFormat::TailwindCSS);

	let mut show_theme_viewer = use_signal(|| false);
	let theme_options = use_signal(|| ThemeOptions { with_doc_themes: with_doc_theme(), format: export_format() });

	let reset_theme = move |_| {
		state.set(DesignerState::default());
	};

	let update_color_palette = move |colors: ColorPalette| {
		state.with_mut(|s| s.color = colors);
	};

	let update_typography = move |typography: TypographySettings| {
		state.with_mut(|s| s.typography = typography);
	};

	let update_spacing = move |spacing: SpacingScale| {
		state.with_mut(|s| s.spacing = spacing);
	};

	let update_border_radius = move |border_radius: BorderRadiusSettings| {
		state.with_mut(|s| s.border_radius = border_radius);
	};

	let update_shadow = move |shadow: ShadowSettings| {
		state.with_mut(|s| s.shadow = shadow);
	};

	rsx! {
		div {
			id: "theme-designer",
			class: "theme-designer-container grid grid-cols-1 lg:grid-cols-3 gap-6",
			div { class: "theme-designer-sidebar col-span-1 bg-white p-6 rounded-lg border",
				Backdrop { show: show_theme_viewer }
				h2 { class: "text-xl font-bold mb-4", "Theme Customization" }

				{
						if show_theme_viewer() {
								rsx! {
									div {
										ThemeViewer {
											state: state(),
											theme_options: theme_options(),
											show_generated_themes: show_theme_viewer,
										}
									}
								}
						} else {
								rsx! {}
						}
				}

				div { class: "theme-designer-tabs mb-6",
					ul { class: "flex border-b",
						li { class: "mr-2",
							button {
								class: tw_merge!(
										"px-4 py-2 font-medium", if active_tab() == "colors" {
										"border-b-2 border-blue-500 text-blue-500" } else { "text-gray-500" }
								),
								onclick: move |_| active_tab.set("colors"),
								"Colors"
							}
						}
						li { class: "mr-2",
							button {
								class: tw_merge!(
										"px-4 py-2 font-medium", if active_tab() == "typography" {
										"border-b-2 border-blue-500 text-blue-500" } else { "text-gray-500" }
								),
								onclick: move |_| active_tab.set("typography"),
								"Typography"
							}
						}
						li { class: "mr-2",
							button {
								class: tw_merge!(
										"px-4 py-2 font-medium", if active_tab() == "spacing" {
										"border-b-2 border-blue-500 text-blue-500" } else { "text-gray-500" }
								),
								onclick: move |_| active_tab.set("spacing"),
								"Spacing"
							}
						}
						li { class: "mr-2",
							button {
								class: tw_merge!(
										"px-4 py-2 font-medium", if active_tab() == "border-radius" {
										"border-b-2 border-blue-500 text-blue-500" } else { "text-gray-500" }
								),
								onclick: move |_| active_tab.set("border-radius"),
								"Border Radius"
							}
						}
						li { class: "mr-2",
							button {
								class: tw_merge!(
										"px-4 py-2 font-medium", if active_tab() == "shadows" {
										"border-b-2 border-blue-500 text-blue-500" } else { "text-gray-500" }
								),
								onclick: move |_| active_tab.set("shadows"),
								"Shadows"
							}
						}
					}
				}

				div { class: "theme-designer-tab-content",
					// Options
					div { id: "theming-config", class: "flex items-center",

						div {
							Select {
								options: ExportFormat::iter()
										.map(|format| {
												SelectOption {
														label: format.to_string(),
														value: match format {
																ExportFormat::TailwindConfig => ExportFormat::TailwindConfig,
																ExportFormat::CSSVariable => ExportFormat::CSSVariable,
																ExportFormat::RustCode => ExportFormat::RustCode,
																ExportFormat::TailwindCSS => ExportFormat::TailwindCSS,
														},
												}
										})
										.collect(),
								current_value: export_format(),
								onchange: move |value| export_format.set(value),
								label: "Export Format: ",
								placeholder: "Select an export format",
								placeholder_class: "text-slate-500",
								dropdown_class: "bg-slate-900 border border-slate-700",
								option_class: "hover:bg-slate-500 bg-slate-800 text-slate-100",
								label_class: "text-slate-200",
								button_class: "bg-slate-900 text-slate-200",
								is_searchable: false,
							}
						}

						div {
							ToggleSwitch {
								state: with_doc_theme,
								label_states: Some(ToggleSwitchLabelStatesProp {
										on: "Document Theme Enabled",
										off: "Document Theme Disabled",
								}),
								label_placement: Some(EToggleSwitchLabelPlacement::Right),
							}
						}
					}

					// Colors tab
					div { class: tw_merge!("tab-panel", if active_tab() != "colors" { "hidden" } else { "" }),
						ColorPicker {
							colors: state().color.clone(),
							on_change: update_color_palette,
						}
					}

					// Typography tab
					div { class: tw_merge!("tab-panel", if active_tab() != "typography" { "hidden" } else { "" }),
						FontSelector {
							typography: state().typography.clone(),
							on_change: update_typography,
						}
					}

					// Spacing tab
					div { class: tw_merge!("tab-panel", if active_tab() != "spacing" { "hidden" } else { "" }),
						SpacingEditor {
							spacing: state().spacing.clone(),
							on_change: update_spacing,
						}
					}

					// Border Radius tab
					div { class: tw_merge!("tab-panel", if active_tab() != "border-radius" { "hidden" } else { "" }),
						BorderRadiusEditor {
							border_radius: state().border_radius.clone(),
							on_change: update_border_radius,
						}
					}

					// Shadows tab
					div { class: tw_merge!("tab-panel", if active_tab() != "shadows" { "hidden" } else { "" }),
						ShadowEditor {
							shadow: state().shadow.clone(),
							on_change: update_shadow,
						}
					}
				}

				div { class: "theme-actions mt-6 flex space-x-4",
					button {
						class: "px-4 py-2 bg-blue-500 text-white rounded hover:bg-blue-600",
						onclick: move |_| show_theme_viewer.set(true),
						"Export"
					}
					button {
						class: "px-4 py-2 border border-gray-300 rounded hover:bg-gray-100",
						onclick: reset_theme,
						"Reset"
					}
				}
			}

			div { class: "theme-preview-wrapper col-span-1 lg:col-span-2",
				ThemePreview {
					state: state(),
					with_doc_theme: with_doc_theme(),
					component_section: props.components_container,
				}
			}
		}
	}
}
