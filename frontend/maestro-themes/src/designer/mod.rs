// For visual theme customization

mod border_radius;
mod color_picker;
mod font_selector;
mod preview;
mod shadow_editor;
mod spacing_editor;
pub mod state;

pub use border_radius::BorderRadiusEditor;
pub use color_picker::ColorPicker;
pub use dioxus::prelude::*;
pub use font_selector::FontSelector;
pub use preview::ThemePreview;
pub use shadow_editor::ShadowEditor;
pub use spacing_editor::SpacingEditor;
pub use state::{BorderRadiusSettings, ColorPalette, DesignerState, ShadowSettings, SpacingScale, ThemedesignerAction, TypographySettings};
pub use tailwind_fuse::tw_merge;

pub use crate::theme::types::Theme;

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
	let mut state = use_signal(|| initial_state);
	let mut active_tab = use_signal(|| "colors");

	let export_theme = move |_| {
		if let Some(on_export) = &props.on_export {
			on_export.call(state());
		}
	};

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
				h2 { class: "text-xl font-bold mb-4", "Theme Customization" }

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
						onclick: export_theme,
						"Export Theme"
					}
					button {
						class: "px-4 py-2 border border-gray-300 rounded hover:bg-gray-100",
						onclick: reset_theme,
						"Reset to Default"
					}
				}
			}

			div { class: "theme-preview-wrapper col-span-1 lg:col-span-2",
				ThemePreview { state: state() }
			}
		}
	}
}
