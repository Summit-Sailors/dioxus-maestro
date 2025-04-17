// Theme preview component
use dioxus::prelude::*;

use crate::{designer::state::DesignerState, theme::provider::ThemeProvider};

#[derive(Props, PartialEq, Clone)]
pub struct ThemePreviewProps {
	state: DesignerState,
	with_doc_theme: bool,
}

#[component]
pub fn ThemePreview(props: ThemePreviewProps) -> Element {
	let state = props.state.clone();

	let content = if props.with_doc_theme {
		rsx! {
      ThemeProvider { default_theme: state.doc_theme,
        div { class: "preview-container" }
      }
    }
	} else {
		rsx! {
      div { class: "preview-container" }
    }
	};

	// Generate CSS variables
	let css_variables = format!(
		r#"

/* Theme configuration using CSS variables */
@theme {{
  /* Colors */
  --primary: {};
  --primary-foreground: white;
  --secondary: {};
  --secondary-foreground: white;
  --accent: {};
  --accent-foreground: white;
  --background: {};
  --foreground: {};
  --card: {};
  --card-foreground: {};
  --border: {};
  --ring: {};
  --destructive: {};
  --destructive-foreground: {};
  --muted: {};
  --muted-foreground: {};

  /* Typography */
  --font-sans: {};
  --font-heading: {};
  --font-size-base: {};
  --line-height-normal: {};

  /* Font Weights */
  {}

  /* Spacing */
  --spacing-unit: {};
  {}

  /* Border Radius */
  --radius-sm: {};
  --radius-md: {};
  --radius-lg: {};
  --radius-xl: {};
  --radius-full: {};

  /* Shadows */
  --shadow-sm: {};
  --shadow-md: {};
  --shadow-lg: {};
  --shadow-xl: {};
  --shadow-2xl: {};
}}"#,
		// Parameters remain the same
		state.color.primary,
		state.color.secondary,
		state.color.accent,
		state.color.background,
		state.color.foreground,
		state.color.card,
		state.color.card_foreground,
		state.color.border,
		state.color.ring,
		state.color.destructive,
		state.color.destructive_foreground,
		state.color.muted,
		state.color.muted_foreground,
		state.typography.font_family,
		state.typography.heading_font_family,
		state.typography.base_size,
		state.typography.line_height,
		state.typography.font_weights.iter().map(|(name, weight)| format!("  --font-weight-{}: {};", name, weight)).collect::<Vec<String>>().join("\n"),
		state.spacing.unit,
		state.spacing.scale.iter().map(|(key, value)| format!("  --spacing-{}: {};", key, value)).collect::<Vec<String>>().join("\n"),
		state.border_radius.sm,
		state.border_radius.md,
		state.border_radius.lg,
		state.border_radius.xl,
		state.border_radius.full,
		state.shadow.sm,
		state.shadow.md,
		state.shadow.lg,
		state.shadow.xl,
		state.shadow.xxl
	);

	rsx! {
    div { class: "theme-preview-container",
      style { "{css_variables}" }

      {content}
    }
  }
}

fn _render_component_showcase() -> Element {
	// This function will be called to render the current component group under design
	rsx! {
    div { class: "preview-container",
      h1 { class: "text-2xl font-bold mb-4", "Theme Preview" }

      // Input Controls Section
      div { class: "preview-card",
        h2 { class: "text-xl font-bold mb-2", "Input Controls" }

        // Input Fields
        h3 { class: "text-lg font-medium mb-2", "Input Fields" }
        div { class: "mb-4",
          div { class: "mb-2",
            label { class: "block mb-1", "Text Input" }
            input {
              class: "preview-input",
              r#type: "text",
              placeholder: "Enter text...",
            }
          }
          div { class: "mb-2",
            label { class: "block mb-1", "Disabled Input" }
            input {
              class: "preview-input",
              r#type: "text",
              placeholder: "Disabled",
              disabled: true,
            }
          }
        }

        // Checkbox & Radio
        h3 { class: "text-lg font-medium mb-2", "Selection Controls" }
        div { class: "flex gap-4 mb-4",
          div {
            div { class: "mb-2",
              input { r#type: "checkbox", id: "checkbox1" }
              label { r#for: "checkbox1", " Checkbox Option" }
            }
            div { class: "mb-2",
              input {
                r#type: "checkbox",
                id: "checkbox2",
                checked: true,
              }
              label { r#for: "checkbox2", " Checked Option" }
            }
          }
          div {
            div { class: "mb-2",
              input {
                r#type: "radio",
                name: "radio-group",
                id: "radio1",
              }
              label { r#for: "radio1", " Radio Option 1" }
            }
            div { class: "mb-2",
              input {
                r#type: "radio",
                name: "radio-group",
                id: "radio2",
                checked: true,
              }
              label { r#for: "radio2", " Radio Option 2" }
            }
          }
        }

        // Select
        h3 { class: "text-lg font-medium mb-2", "Select Component" }
        div { class: "mb-4",
          select { class: "preview-select",
            option { value: "option1", "Option 1" }
            option { value: "option2", "Option 2" }
            option { value: "option3", "Option 3" }
          }
        }

        // Range & Toggle
        h3 { class: "text-lg font-medium mb-2", "Range & Toggle" }
        div { class: "mb-4",
          div { class: "mb-2",
            label { class: "block mb-1", "Range Slider" }
            input {
              class: "preview-range",
              r#type: "range",
              min: "0",
              max: "100",
              value: "50",
            }
          }
          div { class: "mb-2",
            label { class: "block mb-1", "Toggle Switch" }
            div { class: "preview-toggle-wrapper",
              input {
                r#type: "checkbox",
                class: "preview-toggle",
                id: "toggle1",
              }
              label {
                r#for: "toggle1",
                class: "preview-toggle-label",
              }
            }
          }
        }

        // Textarea
        h3 { class: "text-lg font-medium mb-2", "Text Area" }
        div { class: "mb-4",
          textarea {
            class: "preview-textarea",
            placeholder: "Enter multiple lines of text...",
          }
        }
      }

      // Calendar & Date Controls
      div { class: "preview-card",
        h2 { class: "text-xl font-bold mb-2", "Date Components" }

        // Calendar (simplified mock)
        div { class: "preview-calendar mb-4",
          div { class: "calendar-header flex justify-between mb-2",
            button { class: "preview-btn btn-outline", "←" }
            div { "April 2025" }
            button { class: "preview-btn btn-outline", "→" }
          }
          div { class: "calendar-body grid grid-cols-7",
            // Day headers
            div { class: "text-center font-bold", "Su" }
            div { class: "text-center font-bold", "Mo" }
            div { class: "text-center font-bold", "Tu" }
            div { class: "text-center font-bold", "We" }
            div { class: "text-center font-bold", "Th" }
            div { class: "text-center font-bold", "Fr" }
            div { class: "text-center font-bold", "Sa" }

            // Calendar days (a simplified example)
            // Previous month days
            div { class: "text-center text-gray-400", "30" }
            div { class: "text-center text-gray-400", "31" }
            // Current month days
            div { class: "text-center", "1" }
            div { class: "text-center", "2" }
            div { class: "text-center", "3" }
            div { class: "text-center", "4" }
            div { class: "text-center", "5" }
                    // Additional days would continue...
          }
        }
      }

      // Buttons (already in your example)
      div { class: "preview-card",
        h2 { class: "text-xl font-bold mb-2", "Buttons" }
        div { class: "flex gap-2 mb-4",
          button { class: "preview-btn btn-primary", "Primary" }
          button { class: "preview-btn btn-secondary", "Secondary" }
          button { class: "preview-btn btn-outline", "Outline" }
          button { class: "preview-btn btn-destructive", "Destructive" }
        }

        // Spinner
        h3 { class: "text-lg font-medium mb-2", "Spinner" }
        div { class: "mb-4 flex items-center gap-2",
          div { class: "preview-spinner" }
          button { class: "preview-btn btn-primary flex items-center gap-2",
            div { class: "preview-spinner spinner-white" }
            "Loading..."
          }
        }
      }
    }
  }
}
