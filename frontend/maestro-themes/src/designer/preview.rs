// Theme preview component
use {crate::designer::state::DesignerState, dioxus::prelude::*};

#[derive(Props, PartialEq, Clone)]
pub struct ThemePreviewProps {
	state: DesignerState,
}

#[component]
pub fn ThemePreview(props: ThemePreviewProps) -> Element {
	let state = props.state.clone();

	// Generate CSS variables
	let css_variables = format!(
		r#"
    :root{{
      /* Colors */
            --color-primary: {};
            --color-secondary: {};
            --color-accent: {};
            --color-background: {};
            --color-foreground: {};
            --color-card: {};
            --color-card-foreground: {};
            --color-border: {};
            --color-ring: {};
            --color-destructive: {};
            --color-destructive-foreground: {};
            --color-muted: {};
            --color-muted-foreground: {};
            
            /* Typography */
            --font-family: {};
            --heading-font-family: {};
            --base-size: {};
            --line-height: {};
            
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
            --shadow-xxl: {};
        }}
        "#,
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
		state.spacing.unit,
		state.spacing.scale.iter().map(|(key, value)| format!("--spacing-{}: {};", key, value)).collect::<Vec<String>>().join("\n            "),
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

	// The preview component styles
	let preview_styles = r#"
    .preview-container {
            font-family: var(--font-family);
            font-size: var(--base-size);
            line-height: var(--line-height);
            background-color: var(--color-background);
            color: var(--color-foreground);
            padding: var(--spacing-4);
        }
        
        .preview-container h1, .preview-container h2, .preview-container h3 {
            font-family: var(--heading-font-family);
        }
        
        .preview-card {
            background-color: var(--color-card);
            color: var(--color-card-foreground);
            border: 1px solid var(--color-border);
            border-radius: var(--radius-md);
            padding: var(--spacing-4);
            margin-bottom: var(--spacing-4);
            box-shadow: var(--shadow-sm);
        }
        
        .preview-btn {
            padding: var(--spacing-2) var(--spacing-4);
            border-radius: var(--radius-md);
            cursor: pointer;
            font-weight: 500;
            transition: all 0.2s;
        }
        
        .btn-primary {
            background-color: var(--color-primary);
            color: white;
            border: none;
        }
        
        .btn-secondary {
            background-color: var(--color-secondary);
            color: white;
            border: none;
        }
        
        .btn-outline {
            background-color: transparent;
            border: 1px solid var(--color-border);
            color: var(--color-foreground);
        }
        
        .btn-destructive {
            background-color: var(--color-destructive);
            color: var(--color-destructive-foreground);
            border: none;
        }
        
        .preview-alert {
            padding: var(--spacing-4);
            border-radius: var(--radius-md);
            margin-bottom: var(--spacing-4);
        }
        
        .alert-success {
            background-color: #d1fae5;
            color: #065f46;
            border-left: 4px solid #10b981;
        }
        
        .alert-warning {
            background-color: #fef3c7;
            color: #92400e;
            border-left: 4px solid #f59e0b;
        }
        
        .alert-error {
            background-color: #fee2e2;
            color: #b91c1c;
            border-left: 4px solid #ef4444;
        }
        
        .shadow-examples {
            display: flex;
            gap: var(--spacing-4);
            margin-bottom: var(--spacing-4);
        }
        
        .shadow-example {
            width: 60px;
            height: 60px;
            background-color: white;
            border-radius: var(--radius-md);
        }
        
        .shadow-sm-example {
            box-shadow: var(--shadow-sm);
        }
        
        .shadow-md-example {
            box-shadow: var(--shadow-md);
        }
        
        .shadow-lg-example {
            box-shadow: var(--shadow-lg);
        }
        
        .radius-examples {
            display: flex;
            gap: var(--spacing-4);
            margin-bottom: var(--spacing-4);
        }
        
        .radius-example {
            width: 60px;
            height: 60px;
            background-color: var(--color-muted);
        }
        
        .radius-sm-example {
            border-radius: var(--radius-sm);
        }
        
        .radius-md-example {
            border-radius: var(--radius-md);
        }
        
        .radius-lg-example {
            border-radius: var(--radius-lg);
        }
        
        .radius-xl-example {
            border-radius: var(--radius-xl);
        }
  "#;

	rsx! {
		div { class: "theme-preview-container",
			style {
				"{css_variables}"
				"{preview_styles}"
			}

			div { class: "preview-container",
				h1 { class: "text-2xl font-bold mb-4", "Theme Preview" }

				div { class: "preview-card",
					h2 { class: "text-xl font-bold mb-2", "Components" }
					p { class: "mb-4",
						"This preview shows how components will look with your selected theme"
					}

					h3 { class: "text-lg font-medium mb-2", "Buttons" }
					div { class: "flex gap-2 mb-4",
						button { class: "preview-btn btn-primary", "Primary" }
						button { class: "preview-btn btn-secondary", "Secondary" }
						button { class: "preview-btn btn-outline", "Outline" }
						button { class: "preview-btn btn-destructive", "Destructive" }
					}
					h3 { class: "text-lg font-medium mb-2", "Alerts" }
					div { class: "preview-alert alert-success",
						h4 { class: "font-bold", "Success" }
						p { "This is a success message." }
					}
					div { class: "preview-alert alert-warning",
						h4 { class: "font-bold", "Warning" }
						p { "This is a warning message." }
					}
					div { class: "preview-alert alert-error",
						h4 { class: "font-bold", "Error" }
						p { "This is an error message." }
					}
				}
				div { class: "preview-card",
					h2 { class: "text-xl font-bold mb-2", "Typography" }
					h1 { class: "text-3xl font-bold", "Heading 1" }
					h2 { class: "text-2xl font-bold", "Heading 2" }
					h3 { class: "text-xl font-bold", "Heading 3" }
					h4 { class: "text-lg font-bold", "Heading 4" }
					h5 { class: "text-base font-bold", "Heading 5" }
					h6 { class: "text-sm font-bold", "Heading 6" }
					p { class: "text-base", "This is a paragraph with regular text." }
					p { class: "text-sm", "This is smaller text." }
					p { class: "text-xs", "This is extra small text." }
					p {
						"Text can be "
						strong { "bold" }
						", "
						a { class: "text-blue-500 underline", "linked" }
						", or "
						em { "emphasized" }
						"."
					}
				}
				div { class: "preview-card",
					h2 { class: "text-xl font-bold mb-2", "Shadows" }
					div { class: "shadow-examples",
						div { class: "shadow-example shadow-sm-example" }
						div { class: "shadow-example shadow-md-example" }
						div { class: "shadow-example shadow-lg-example" }
					}
					h2 { class: "text-xl font-bold mb-2 mt-4", "Border Radius" }
					div { class: "radius-examples",
						div { class: "radius-example radius-sm-example" }
						div { class: "radius-example radius-md-example" }
						div { class: "radius-example radius-lg-example" }
						div { class: "radius-example radius-xl-example" }
					}
				}
			}
		}
	}
}
