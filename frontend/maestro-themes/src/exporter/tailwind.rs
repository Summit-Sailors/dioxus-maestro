// Export to tailwind configs
// Would need a way to export CSS variables as well (in tailwind@v all this is done in the main CSS file) -> maybe we need a new file?(css_vars.rs -> we'll see
// about that when we get there)
// Theme export functionality
use {
	crate::designer::state::DesignerState,
	std::{fs, path::Path},
};

/// Export theme to CSS file
pub fn export_theme_to_css(state: &DesignerState, path: &Path) -> Result<(), Box<dyn std::error::Error>> {
	let css = generate_theme_css(state);
	fs::write(path, css)?;
	Ok(())
}

/// Export theme to Tailwind config (JavaScript)
pub fn export_theme_to_tailwind(state: &DesignerState, path: &Path) -> Result<(), Box<dyn std::error::Error>> {
	let config = generate_tailwind_config(state);
	fs::write(path, config)?;
	Ok(())
}

/// Generate CSS variables from theme
pub fn generate_theme_css(state: &DesignerState) -> String {
	format!(
		r#"/**
* Generated theme CSS variables
*/
:root {{
  /* Colors */
  --color-primary: {};
  --color-primary-foreground: white;
  --color-secondary: {};
  --color-secondary-foreground: white;
  --color-accent: {};
  --color-accent-foreground: white;
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
  --shadow-xxl: {};
}}

/* Base element styles */
body {{
  font-family: var(--font-family);
  font-size: var(--base-size);
  line-height: var(--line-height);
  background-color: var(--color-background);
  color: var(--color-foreground);
}}

h1, h2, h3, h4, h5, h6 {{
  font-family: var(--heading-font-family);
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
		state.typography.font_weights.iter().map(|(name, weight)| format!("--font-weight-{}: {};", name, weight)).collect::<Vec<String>>().join("\n    "),
		state.spacing.unit,
		state.spacing.scale.iter().map(|(key, value)| format!("--spacing-{}: {};", key, value)).collect::<Vec<String>>().join("\n    "),
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
	)
}

/// Generate Tailwind config from theme
pub fn generate_tailwind_config(state: &DesignerState) -> String {
	let spacing_values = state.spacing.scale.iter().map(|(key, value)| format!("      '{}': '{}',", key, value)).collect::<Vec<String>>().join("\n");

	format!(
		r#"/**
* Generated Tailwind CSS configuration
*/
module.exports = {{
theme: {{
  extend: {{
    colors: {{
      'primary': '{}',
      'primary-foreground': 'white',
      'secondary': '{}',
      'secondary-foreground': 'white',
      'accent': '{}',
      'background': '{}',
      'foreground': '{}',
      'card': '{}',
      'card-foreground': '{}',
      'border': '{}',
      'ring': '{}',
      'destructive': '{}',
      'destructive-foreground': '{}',
      'muted': '{}',
      'muted-foreground': '{}',
    }},
    fontFamily: {{
      'sans': ['{}'],
      'heading': ['{}'],
    }},
    fontSize: {{
      'base': '{}',
    }},
    lineHeight: {{
      'normal': '{}',
    }},
    spacing: {{
{}
    }},
    borderRadius: {{
      'sm': '{}',
      'DEFAULT': '{}',
      'md': '{}',
      'lg': '{}',
      'xl': '{}',
      'full': '{}',
    }},
    boxShadow: {{
      'sm': '{}',
      'DEFAULT': '{}',
      'md': '{}',
      'lg': '{}',
      'xl': '{}',
      '2xl': '{}',
    }},
  }},
}},
plugins: [],
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
		spacing_values,
		state.border_radius.sm,
		state.border_radius.md,
		state.border_radius.md,
		state.border_radius.lg,
		state.border_radius.xl,
		state.border_radius.full,
		state.shadow.sm,
		state.shadow.md,
		state.shadow.md,
		state.shadow.lg,
		state.shadow.xl,
		state.shadow.xxl
	)
}

/// Export theme to Rust code (for embedding in Dioxus apps)
pub fn export_theme_to_rust(state: &DesignerState, path: &Path) -> Result<(), Box<dyn std::error::Error>> {
	let rust_code = generate_rust_theme(state);
	fs::write(path, rust_code)?;
	Ok(())
}

/// Generate Rust code for theme
pub fn generate_rust_theme(state: &DesignerState) -> String {
	let font_weights = state
		.typography
		.font_weights
		.iter()
		.map(|(name, weight)| format!("    font_weights.insert(String::from(\"{}\"), {});", name, weight))
		.collect::<Vec<String>>()
		.join("\n");

	let spacing_scale = state
		.spacing
		.scale
		.iter()
		.map(|(key, value)| format!("    scale.insert(String::from(\"{}\"), String::from(\"{}\"));", key, value))
		.collect::<Vec<String>>()
		.join("\n");

	format!(
		r#"use {{
  crate::designer::state::*,
  std::collections::HashMap,
}};

/// Generated theme from the theme designer
pub fn get_theme() -> DesignerState {{
  let mut font_weights = HashMap::new();
{}

  let mut scale = HashMap::new();
{}

  DesignerState {{
      color: ColorPalette {{
          primary: String::from("{}"),
          secondary: String::from("{}"),
          accent: String::from("{}"),
          background: String::from("{}"),
          foreground: String::from("{}"),
          card: String::from("{}"),
          card_foreground: String::from("{}"),
          border: String::from("{}"),
          ring: String::from("{}"),
          destructive: String::from("{}"),
          destructive_foreground: String::from("{}"),
          muted: String::from("{}"),
          muted_foreground: String::from("{}"),
      }},
      typography: TypographySettings {{
          font_family: String::from("{}"),
          heading_font_family: String::from("{}"),
          base_size: String::from("{}"),
          line_height: String::from("{}"),
          font_weights,
      }},
      spacing: SpacingScale {{
          unit: String::from("{}"),
          scale,
      }},
      border_radius: BorderRadiusSettings {{
          sm: String::from("{}"),
          md: String::from("{}"),
          lg: String::from("{}"),
          xl: String::from("{}"),
          full: String::from("{}"),
      }},
      shadow: ShadowSettings {{
          sm: String::from("{}"),
          md: String::from("{}"),
          lg: String::from("{}"),
          xl: String::from("{}"),
          xxl: String::from("{}"),
      }},
  }}
}}
"#,
		font_weights,
		spacing_scale,
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
	)
}
