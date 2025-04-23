use dioxus::prelude::*;
use maestro_ui::{
	select::{Select, SelectOption},
	toggle::{EToggleSwitchLabelPlacement, ToggleSwitch, ToggleSwitchLabelStatesProp},
};
use strum::IntoEnumIterator;
use tailwind_fuse::tw_merge;

use super::prelude::*;
use crate::{
	components::{
		backdrop::Backdrop,
		maestro_themes::{
			designer::state::{BorderRadiusSettings, ColorPalette, DesignerState, ShadowSettings, SpacingScale, TypographySettings},
			exporter::{ExportFormat, ThemeOptions},
		},
	},
	router::Route,
};

#[derive(Props, PartialEq, Clone)]
pub struct ThemeDesignerProps {
	pub components_id: String,
}

#[component]
pub fn ThemeDesigner(props: ThemeDesignerProps) -> Element {
	let initial_state = DesignerState::default();
	let components_id = props.components_id.clone();

	let components_id_clone = components_id.clone();

	let mut state = use_signal(|| initial_state);
	let mut active_tab = use_signal(|| "colors");

	let with_doc_theme = use_signal(|| false);
	let mut export_format = use_signal(|| ExportFormat::TailwindCSS);

	let mut show_theme_viewer = use_signal(|| false);
	let theme_options = use_signal(|| ThemeOptions { with_doc_themes: with_doc_theme(), format: export_format(), components_id });

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

	// to navigate home
	let navigator = use_navigator();

	// Function to check if a tab is active
	let is_tab_active = move |tab: &str| -> bool { active_tab() == tab };

	rsx! {
		div {
			id: "theme-designer",
			class: "theme-designer-container flex flex-col md:flex-row gap-6",
			div { class: "theme-designer-sidebar col-span-1 bg-[color:var(--card-bg)] p-6 rounded-lg border border-[color:var(--border-color)] shadow-md overflow-y-auto w-full md:w-3/4",
				h2 { class: "text-xl font-semibold mb-6 text-[color:var(--card-text)]",
					"Theme Customization"
				}

				button {
					class: "px-4 py-2 bg-[color:var(--primary-bg)] text-[color:var(--primary-text)] rounded-[var(--radius-md)] hover:bg-[color:var(--secondary-bg)] transition-all duration-200 flex items-center gap-2 mb-6 w-full sm:w-auto",
					onclick: move |_| {
							navigator.push(Route::HomePage {});
					},
					"Go Home"
				}

				{
						if show_theme_viewer() {
								rsx! {
									Backdrop { show: show_theme_viewer }
									div { class: "fixed top-1/2 left-1/2 transform -translate-x-1/2 -translate-y-1/2 w-[90%] max-w-3xl max-h-[80vh] overflow-y-auto bg-[color:var(--card-bg)] p-6 rounded-lg border border-[color:var(--border-color)] shadow-xl z-50 transition-appear",
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
					ul { class: "flex flex-wrap sm:border-b border-[color:var(--border-color)]",
						li { class: "mr-2 w-full sm:w-auto",
							button {
								class: tw_merge!(
										"px-4 py-2 font-medium transition-all w-full text-left sm:text-center", if
										is_tab_active("colors") {
										"border-b-2 sm:border-b-2 border-l-2 sm:border-l-0 border-[color:var(--primary-bg)] text-[color:var(--primary-bg)] bg-[color:var(--hover-bg)] sm:bg-transparent"
										} else {
										"text-[color:var(--text-color)] hover:text-[color:var(--primary-bg)] border-b sm:border-b-0 border-[color:var(--border-color)]"
										}
								),
								onclick: move |_| active_tab.set("colors"),
								"Colors"
							}
						}
						li { class: "mr-2 w-full sm:w-auto",
							button {
								class: tw_merge!(
										"px-4 py-2 font-medium transition-all w-full text-left sm:text-center", if
										is_tab_active("typography") {
										"border-b-2 sm:border-b-2 border-l-2 sm:border-l-0 border-[color:var(--primary-bg)] text-[color:var(--primary-bg)] bg-[color:var(--hover-bg)] sm:bg-transparent"
										} else {
										"text-[color:var(--text-color)] hover:text-[color:var(--primary-bg)] border-b sm:border-b-0 border-[color:var(--border-color)]"
										}
								),
								onclick: move |_| active_tab.set("typography"),
								"Typography"
							}
						}
						li { class: "mr-2 w-full sm:w-auto",
							button {
								class: tw_merge!(
										"px-4 py-2 font-medium transition-all w-full text-left sm:text-center", if
										is_tab_active("spacing") {
										"border-b-2 sm:border-b-2 border-l-2 sm:border-l-0 border-[color:var(--primary-bg)] text-[color:var(--primary-bg)] bg-[color:var(--hover-bg)] sm:bg-transparent"
										} else {
										"text-[color:var(--text-color)] hover:text-[color:var(--primary-bg)] border-b sm:border-b-0 border-[color:var(--border-color)]"
										}
								),
								onclick: move |_| active_tab.set("spacing"),
								"Spacing"
							}
						}
						li { class: "mr-2 w-full sm:w-auto",
							button {
								class: tw_merge!(
										"px-4 py-2 font-medium transition-all w-full text-left sm:text-center", if
										is_tab_active("border-radius") {
										"border-b-2 sm:border-b-2 border-l-2 sm:border-l-0 border-[color:var(--primary-bg)] text-[color:var(--primary-bg)] bg-[color:var(--hover-bg)] sm:bg-transparent"
										} else {
										"text-[color:var(--text-color)] hover:text-[color:var(--primary-bg)] border-b sm:border-b-0 border-[color:var(--border-color)]"
										}
								),
								onclick: move |_| active_tab.set("border-radius"),
								"Border Radius"
							}
						}
						li { class: "mr-2 w-full sm:w-auto",
							button {
								class: tw_merge!(
										"px-4 py-2 font-medium transition-all w-full text-left sm:text-center", if
										is_tab_active("shadows") {
										"border-b-2 sm:border-b-2 border-l-2 sm:border-l-0 border-[color:var(--primary-bg)] text-[color:var(--primary-bg)] bg-[color:var(--hover-bg)] sm:bg-transparent"
										} else {
										"text-[color:var(--text-color)] hover:text-[color:var(--primary-bg)] border-b sm:border-b-0 border-[color:var(--border-color)]"
										}
								),
								onclick: move |_| active_tab.set("shadows"),
								"Shadows"
							}
						}
					}
				}

				div { class: "theme-designer-tab-content",
					// Options
					div {
						id: "theming-config",
						class: "flex flex-col md:flex-row flex-wrap items-center justify-between gap-4 mb-4 p-4 bg-[color:var(--bg-color)] rounded-[var(--radius-md)]",

						div { class: "flex-1 min-w-[200px]",
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
								placeholder_class: "text-[color:var(--muted-text)]",
								dropdown_class: "bg-[color:var(--bg-color)] border border-[color:var(--border-color)] shadow-md rounded-[var(--radius-md)]",
								option_class: "hover:bg-[color:var(--hover-bg)] text-[color:var(--text-color)] py-2 px-3",
								label_class: "text-[color:var(--text-color)] font-medium",
								button_class: "bg-[color:var(--input-bg)] text-[color:var(--text-color)] border border-[color:var(--border-color)] rounded-[var(--radius-md)]",
								is_searchable: false,
							}
						}

						div { class: "flex-1 min-w-[200px] flex justify-end md:justify-start",
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
					div {
						class: tw_merge!(
								"tab-panel p-4 bg-[color:var(--input-bg)] rounded-[var(--radius-md)] transition-all",
								if active_tab() != "colors" { "hidden" } else { "" }
						),
						ColorPicker {
							colors: state().color.clone(),
							on_change: update_color_palette,
						}
					}

					// Typography tab
					div {
						class: tw_merge!(
								"tab-panel p-4 bg-[color:var(--input-bg)] rounded-[var(--radius-md)] transition-all",
								if active_tab() != "typography" { "hidden" } else { "" }
						),
						FontSelector {
							typography: state().typography.clone(),
							on_change: update_typography,
						}
					}

					// Spacing tab
					div {
						class: tw_merge!(
								"tab-panel p-4 bg-[color:var(--input-bg)] rounded-[var(--radius-md)] transition-all",
								if active_tab() != "spacing" { "hidden" } else { "" }
						),
						SpacingEditor {
							spacing: state().spacing.clone(),
							on_change: update_spacing,
						}
					}

					// Border Radius tab
					div {
						class: tw_merge!(
								"tab-panel p-4 bg-[color:var(--input-bg)] rounded-[var(--radius-md)] transition-all",
								if active_tab() != "border-radius" { "hidden" } else { "" }
						),
						BorderRadiusEditor {
							border_radius: state().border_radius.clone(),
							on_change: update_border_radius,
						}
					}

					// Shadows tab
					div {
						class: tw_merge!(
								"tab-panel p-4 bg-[color:var(--input-bg)] rounded-[var(--radius-md)] transition-all",
								if active_tab() != "shadows" { "hidden" } else { "" }
						),
						ShadowEditor {
							shadow: state().shadow.clone(),
							on_change: update_shadow,
						}
					}
				}

				div { class: "theme-actions mt-8 flex flex-wrap gap-4",
					button {
						class: "px-4 py-2 bg-[color:var(--primary-bg)] text-[color:var(--primary-text)] rounded-[var(--radius-md)] hover:bg-[color:var(--secondary-bg)] transition-all duration-200 shadow-sm flex-1",
						onclick: move |_| show_theme_viewer.set(true),
						"Export"
					}
					button {
						class: "px-4 py-2 bg-[color:var(--bg-color)] text-[color:var(--text-color)] border border-[color:var(--border-color)] rounded-[var(--radius-md)] hover:bg-[color:var(--hover-bg)] transition-all duration-200 flex-1",
						onclick: reset_theme,
						"Reset"
					}
				}
			}

			div { class: "theme-preview-wrapper col-span-1 lg:col-span-2 bg-[color:var(--bg-color)] p-6 rounded-lg border border-[color:var(--border-color)] shadow-lg w-full md:w-1/4",
				ThemePreview {
					state: state(),
					with_doc_theme: with_doc_theme(),
					components_section_id: components_id_clone,
				}
			}
		}
	}
}
