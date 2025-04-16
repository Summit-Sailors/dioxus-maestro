// Simple example using the theme system
use dioxus::prelude::*;
use maestro_themes::{
	designer::{DesignerState, ThemeDesigner},
	theme::{provider::ThemeProvider, types::Theme},
};
use tailwind_fuse::tw_join;

fn main() {
	launch(App);
}

#[component]
fn App() -> Element {
	rsx! {
		ThemeProvider { default_theme: Some(Theme::Auto),
			div { class: "p-8 min-h-screen",
				h1 { class: "text-2xl font-bold mb-6", "Maestro Themes Demo" }

				Tabs {}
			}
		}
	}
}

#[component]
fn Tabs() -> Element {
	let mut active_tab = use_signal(|| "demo");

	rsx! {
		div { class: "flex border-b mb-4",
			button {
				class: tw_join!(
						"px-4 py-2 font-medium", if active_tab() == "demo" {
						"border-b-2 border-primary text-primary" } else { "text-muted-foreground" }
				),
				onclick: move |_| active_tab.set("demo"),
				"Demo Components"
			}
			button {
				class: tw_join!(
						"px-4 py-2 font-medium", if active_tab() == "theme" {
						"border-b-2 border-primary text-primary" } else { "text-muted-foreground" }
				),
				onclick: move |_| active_tab.set("theme"),
				"Theme Designer"
			}
		}

		// tab content
		div {
			div { class: tw_join!("tab-panel", if active_tab() == "demo" { "hidden" } else { "" }),
				DemoComponents {}
			}

			div { class: tw_join!("tab-panel", if active_tab() == "theme" { "hidden" } else { "" }),
				CustomizeTheme {}
			}
		}
	}
}

#[component]
fn DemoComponents() -> Element {
	rsx! {
		div { class: "grid grid-cols-1 md:grid-cols-2 gap-6",
			// cards with various components
			div { class: "bg-card p-4 rounded-lg border border-border shadow-md",
				h2 { class: "text-xl mb-4 text-card-foreground", "Buttons" }
				div { class: "flex flex-wrap gap-2",
					button { class: "px-4 py-2 bg-primary text-primary-foreground rounded-md",
						"Primary"
					}
					button { class: "px-4 py-2 bg-secondary text-secondary-foreground rounded-md",
						"Secondary"
					}
					button { class: "px-4 py-2 bg-accent text-accent-foreground rounded-md",
						"Accent"
					}
					button { class: "px-4 py-2 bg-destructive text-destructive-foreground rounded-md",
						"Destructive"
					}
					button { class: "px-4 py-2 border-border rounded-md", "Outline" }
				}
			}

			div { class: "bg-card p-4 rounded-lg border border-border shadow-md",
				h2 { class: "text-xl mb-4 text-card-foreground", "Typography" }

				div { class: "space-y-2",
					h1 { class: "text-2xl font-bold", "Heading 1" }
					h2 { class: "text-xl font-bold", "Heading 2" }
					h3 { class: "text-lg font-bold", "Heading 3" }
					p { class: "text-base", "Regular paragraph text" }
					p { class: "text-sm text-muted-foreground", "Small muted text" }
				}
			}

		// More components
		}
	}
}

#[component]
fn CustomizeTheme() -> Element {
	let on_export = move |state: DesignerState| {
		// Export to CSS
		// let css_result = export_theme(&state, ExportFormat::CssVariables);
		// let _ = save_export_result(&css_result, std::path::Path::new("./theme-export"));

		// // Export to Tailwind
		// let tailwind_result = export_theme(&state, ExportFormat::TailwindV4);
		// let _ = save_export_result(&tailwind_result, std::path::Path::new("./theme-export"));

		// // Export to Rust code
		// let rust_result = export_theme(&state, ExportFormat::RustCode);
		// let _ = save_export_result(&rust_result, std::path::Path::new("./theme-export"));

		// // Export to Tailwind Config
		// let tw_config_result = export_theme(&state, ExportFormat::TailwindConfig);
		// let _ = save_export_result(&tw_config_result, std::path::Path::new("./theme-export"));

		println!("Themes exported to the ./theme-export directory!")
	};
	rsx! {
		div {
			p { class: "mb-4",
				"Customize your themes and export it for use in your Dioxus Applications"
			}

			ThemeDesigner { on_export }
		}
	}
}
