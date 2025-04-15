// Export to tailwind configs

use crate::designer::state::DesignerState;

/// Generate Tailwind v4 CSS configuration
pub fn generate_tailwind_v4_css(state: &DesignerState) -> String {
	format!(
		r#"/**
* Tailwind CSS v4 Configuration
* Generated with CSS-first approach
*/

@import "tailwindcss";

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
}}

/* Base element styles using cascade layers */
@layer base {{
body {{
  font-family: var(--font-sans);
  font-size: var(--font-size-base);
  line-height: var(--line-height-normal);
  background-color: var(--background);
  color: var(--foreground);
}}

h1, h2, h3, h4, h5, h6 {{
  font-family: var(--font-heading);
}}
}}

/* Optional explicit configuration using @config directive */
@config {{
theme: {{
  colors: {{
    primary: 'var(--primary)',
    'primary-foreground': 'var(--primary-foreground)',
    secondary: 'var(--secondary)',
    'secondary-foreground': 'var(--secondary-foreground)',
    accent: 'var(--accent)',
    'accent-foreground': 'var(--accent-foreground)',
    background: 'var(--background)',
    foreground: 'var(--foreground)',
    card: 'var(--card)',
    'card-foreground': 'var(--card-foreground)',
    border: 'var(--border)',
    ring: 'var(--ring)',
    destructive: 'var(--destructive)',
    'destructive-foreground': 'var(--destructive-foreground)',
    muted: 'var(--muted)',
    'muted-foreground': 'var(--muted-foreground)',
  }},
  fontFamily: {{
    sans: ['var(--font-sans)'],
    heading: ['var(--font-heading)'],
  }},
  fontSize: {{
    base: 'var(--font-size-base)',
  }},
  lineHeight: {{
    normal: 'var(--line-height-normal)',
  }},
  borderRadius: {{
    sm: 'var(--radius-sm)',
    DEFAULT: 'var(--radius-md)',
    md: 'var(--radius-md)',
    lg: 'var(--radius-lg)',
    xl: 'var(--radius-xl)',
    full: 'var(--radius-full)',
  }},
  boxShadow: {{
    sm: 'var(--shadow-sm)',
    DEFAULT: 'var(--shadow-md)',
    md: 'var(--shadow-md)',
    lg: 'var(--shadow-lg)',
    xl: 'var(--shadow-xl)',
    '2xl': 'var(--shadow-2xl)',
  }},
  /* Note: In v4, you can directly reference CSS variables in spacing config */
  spacing: {{
    /* Reference to the variables defined in @theme */
  }},
}},
plugins: [
  /* Any plugins can be included here */
]
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
	)
}

/// Generate CSS variables from theme
pub fn generate_theme_css(state: &DesignerState) -> String {
	format!(
		r#"/**
* Generated theme CSS variables
* Compatible with Tailwind CSS v4
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
--color-ring: {}; /* Note: In Tailwind v4, ring defaults to 1px */

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

/* Base element styles using modern cascade layers approach */
@layer base {{
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
		state.typography.font_weights.iter().map(|(name, weight)| format!("  --font-weight-{}: {};", name, weight)).collect::<Vec<String>>().join("\n  "),
		state.spacing.unit,
		state.spacing.scale.iter().map(|(key, value)| format!("  --spacing-{}: {};", key, value)).collect::<Vec<String>>().join("\n  "),
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

/// Generate Tailwind config from theme (For compatibility or projects not using CSS-first approach)
pub fn generate_tailwind_config(state: &DesignerState) -> String {
	let spacing_values = state.spacing.scale.iter().map(|(key, value)| format!("      '{}': '{}',", key, value)).collect::<Vec<String>>().join("\n");

	format!(
		r#"/**
* Generated Tailwind CSS configuration
* For Tailwind v4 compatibility
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
// In v4, the PostCSS plugin and CLI are separate
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
