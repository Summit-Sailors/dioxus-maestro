use dioxus::prelude::*;
use maestro_ui::{
	select::{Select, SelectOption},
	toggle::{EToggleSwitchLabelPlacement, ToggleSwitch, ToggleSwitchLabelStatesProp},
};
use strum::IntoEnumIterator;
use tailwind_fuse::tw_merge;

use super::components::prelude::*;
use crate::{
	components::backdrop::Backdrop,
	maestro_themes::{
		editor::{
			misc::{stylesheet_exporter::ThemeViewer, theme_preview::ThemePreview},
			state::{DesignerState, ThemedesignerAction},
		},
		exporter::{ExportFormat, ThemeOptions},
	},
};

#[derive(Debug, Clone, PartialEq, strum_macros::EnumIter, strum_macros::Display, strum_macros::EnumString)]
enum CurrentComponentGroup {
	#[strum(to_string = "buttons")]
	Buttons,
	#[strum(to_string = "select-and-multiselect")]
	Select,
	#[strum(to_string = "input-and-labels")]
	Input,
	#[strum(to_string = "toggle-and-radio")]
	Toggle,
	#[strum(to_string = "textarea-spinner-range")]
	TextArea,
}

impl CurrentComponentGroup {
	fn variant_as_string(&self) -> &'static str {
		match self {
			CurrentComponentGroup::Buttons => stringify!(Buttons),
			CurrentComponentGroup::Select => stringify!(Select),
			CurrentComponentGroup::Input => stringify!(Input),
			CurrentComponentGroup::Toggle => stringify!(Toggle),
			CurrentComponentGroup::TextArea => stringify!(TextArea),
		}
	}
}

#[derive(Props, PartialEq, Clone)]
pub struct ThemeDesignerProps {
	#[props(default = DesignerState::default())]
	pub state: Option<DesignerState>,
	pub components_id: String,
}

#[component]
pub fn ThemeEditor(props: ThemeDesignerProps) -> Element {
	use_context_provider(|| Signal::new(props.state.unwrap_or_default()));

	let components_id = props.components_id.clone();
	let router = use_context::<RouterContext>();

	let current_url = router.full_route_string();
	// create tab form URL if present
	let initial_tab = if current_url.contains("?tab=") {
		let parts: Vec<&str> = current_url.split("?tab=").collect();
		if parts.len() > 1 { parts[1].to_string() } else { "colors".to_string() }
	} else {
		"colors".to_string()
	};

	let mut active_tab = use_signal(|| initial_tab);
	let mut current_component_group = use_signal(|| components_id.clone());

	let with_doc_theme = use_signal(|| false);
	let mut export_format = use_signal(|| ExportFormat::TailwindCSS);

	let mut show_theme_viewer = use_signal(|| false);
	let mut theme_options = use_signal(|| ThemeOptions { with_doc_themes: with_doc_theme(), format: export_format(), components_id: components_id.clone() });

	let doc_theme_enabled = with_doc_theme();
	let export_fmt = export_format();
	let components_group = current_component_group();

	// to update URL based on active tab and component group
	let update_url = move |tab: String, component: String| {
		let new_url = format!("/themes/{}?tab={}", component, tab);
		router.replace(NavigationTarget::Internal(new_url));
	};

	// editor actions
	let reset_theme = move |_| {
		let mut state = use_context::<Signal<DesignerState>>();
		state.with_mut(|s| s.apply_action(ThemedesignerAction::ResetToDefaults));
	};

	let is_tab_active = move |tab: &str| -> bool { active_tab() == tab };

	let mut change_tab = move |new_tab: &'static str| {
		active_tab.set(new_tab.to_string());
		update_url(new_tab.to_string(), current_component_group());
	};

	let change_component_group = move |new_group: String| {
		current_component_group.set(new_group.clone());
		update_url(active_tab(), new_group);
	};

	use_effect(move || {
		theme_options.set(ThemeOptions { with_doc_themes: doc_theme_enabled, format: export_fmt.clone(), components_id: components_group.clone() });
	});

	let tab_content = {
		match active_tab().as_str() {
			"colors" => rsx! {
				div { class: "tab-panel p-4 bg-[color:var(--input-bg)] rounded-[var(--radius-md)] transition-all",
					ColorPicker {}
				}
			},
			"typography" => rsx! {
				div { class: "tab-panel p-4 bg-[color:var(--input-bg)] rounded-[var(--radius-md)] transition-all",
					FontSelector {}
				}
			},
			"spacing" => rsx! {
				div { class: "tab-panel p-4 bg-[color:var(--input-bg)] rounded-[var(--radius-md)] transition-all",
					SpacingEditor {}
				}
			},
			"border-radius" => rsx! {
				div { class: "tab-panel p-4 bg-[color:var(--input-bg)] rounded-[var(--radius-md)] transition-all",
					BorderRadiusEditor {}
				}
			},
			"shadows" => rsx! {
				div { class: "tab-panel p-4 bg-[color:var(--input-bg)] rounded-[var(--radius-md)] transition-all",
					ShadowEditor {}
				}
			},
			_ => rsx! {
				div {}
			},
		}
	};

	let theme_viewer = if show_theme_viewer() {
		rsx! {
			Backdrop { show: show_theme_viewer }
			div { class: "fixed top-1/2 left-1/2 transform -translate-x-1/2 -translate-y-1/2 w-[90%] max-w-3xl max-h-[80vh] overflow-y-auto bg-[color:var(--card-bg)] p-6 rounded-lg border border-[color:var(--border-color)] shadow-xl z-50 transition-appear",
				ThemeViewer {
					theme_options: theme_options(),
					show_generated_themes: show_theme_viewer,
				}
			}
		}
	} else {
		rsx! {}
	};

	let base_btn_classes = String::from("px-4 py-2 font-medium transition-all w-full text-left sm:text-center");
	let active_tab_classes = String::from(
		"border-b-2 sm:border-b-2 border-l-2 sm:border-l-0 border-[color:var(--primary-bg)] text-[color:var(--primary-bg)] bg-[color:var(--hover-bg)] sm:bg-transparent",
	);
	let inactive_tab_classes =
		String::from("text-[color:var(--text-color)] hover:text-[color:var(--primary-bg)] border-b sm:border-b-0 border-[color:var(--border-color)]");

	rsx! {
		div {
			id: "theme-designer",
			class: "theme-designer-container flex flex-col md:flex-row gap-6 overflow-auto",
			div { class: "theme-designer-sidebar col-span-1 bg-[color:var(--card-bg)] p-6 rounded-lg border border-[color:var(--border-color)] shadow-md overflow-y-auto w-full md:w-1/2",
				h2 { class: "text-xl font-semibold mb-6 text-[color:var(--card-text)]",
					"Theme Customization"
				}

				{theme_viewer}

				div { class: "theme-designer-tabs mb-6",
					ul { class: "flex flex-wrap sm:border-b border-[color:var(--border-color)]",
						li { class: "mr-2 w-full sm:w-auto",
							button {
								class: tw_merge!(
										base_btn_classes.clone(), if is_tab_active("colors") { active_tab_classes.clone()
										} else { inactive_tab_classes.clone() }
								),
								onclick: move |_| change_tab("colors"),
								"Colors"
							}
						}
						li { class: "mr-2 w-full sm:w-auto",
							button {
								class: tw_merge!(
										base_btn_classes.clone(), if is_tab_active("typography") { active_tab_classes
										.clone() } else { inactive_tab_classes.clone() }
								),
								onclick: move |_| change_tab("typography"),
								"Typography"
							}
						}
						li { class: "mr-2 w-full sm:w-auto",
							button {
								class: tw_merge!(
										base_btn_classes.clone(), if is_tab_active("spacing") { active_tab_classes
										.clone() } else { inactive_tab_classes.clone() }
								),
								onclick: move |_| change_tab("spacing"),
								"Spacing"
							}
						}
						li { class: "mr-2 w-full sm:w-auto",
							button {
								class: tw_merge!(
										base_btn_classes.clone(), if is_tab_active("border-radius") { active_tab_classes
										.clone() } else { inactive_tab_classes.clone() }
								),
								onclick: move |_| change_tab("border-radius"),
								"Border Radius"
							}
						}
						li { class: "mr-2 w-full sm:w-auto",
							button {
								class: tw_merge!(
										base_btn_classes.clone(), if is_tab_active("shadows") { active_tab_classes
										.clone() } else { inactive_tab_classes.clone() }
								),
								onclick: move |_| change_tab("shadows"),
								"Shadows"
							}
						}
					}
				}

				div { class: "component-group-selector mb-6 p-4 bg-[color:var(--bg-color)] rounded-[var(--radius-md)]",
					h3 { class: "text-lg font-medium mb-4 text-[color:var(--text-color)]",
						"Component Group"
					}

					Select {
						options: CurrentComponentGroup::iter()
								.map(|group| {
										SelectOption {
												label: group.variant_as_string().to_string(),
												value: group.to_string(),
										}
								})
								.collect(),
						current_value: current_component_group(),
						onchange: change_component_group,
						label: "Select Component Group: ",
						placeholder: "Select a component group",
						placeholder_class: "text-[color:var(--muted-text)]",
						dropdown_class: "bg-[color:var(--bg-color)] border border-[color:var(--border-color)] shadow-md rounded-[var(--radius-md)]",
						option_class: "hover:bg-[color:var(--hover-bg)] text-[color:var(--text-color)] py-2 px-3",
						label_class: "text-[color:var(--text-color)] font-medium",
						button_class: "bg-[color:var(--input-bg)] text-[color:var(--text-color)] border border-[color:var(--border-color)] rounded-[var(--radius-md)]",
						is_searchable: false,
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
					{tab_content}
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

			div { class: "theme-preview-wrapper col-span-2 lg:col-span-2 bg-[color:var(--bg-color)] p-6 rounded-lg border border-[color:var(--border-color)] shadow-lg w-full md:w-1/2",
				ThemePreview {
					with_doc_theme: with_doc_theme(),
					components_section_id: current_component_group(),
				}
			}
		}
	}
}
