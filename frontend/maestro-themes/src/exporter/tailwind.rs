use crate::designer::state::DesignerState;

/// Generate Tailwind v4 CSS configuration with theme support
pub fn generate_tailwind_v4_css(state: &DesignerState, with_themes: bool) -> String {
	// Base theme configuration using CSS variables
	let base_theme = format!(
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

	// If theme support is enabled, build an integrated theme system
	if with_themes {
		format!(
			r#"{}

/* Define dark variant based on data-theme attribute */
@variant dark (&:where([data-theme="dark"], [data-theme="dark"] *));

/* Light theme (default) variables */
:root {{
  /* Map theme variables to UI variables */
  --bg-color: var(--background);
  --text-color: var(--foreground);
  --border-color: var(--border);
  --input-bg: var(--muted);
  --card-bg: var(--card);
  --card-text: var(--card-foreground);
  --accent-bg: var(--accent);
  --accent-text: var(--accent-foreground);
  --primary-bg: var(--primary);
  --primary-text: var(--primary-foreground);
  --secondary-bg: var(--secondary);
  --secondary-text: var(--secondary-foreground);
  --muted-text: var(--muted-foreground);
  
  /* Interactive states */
  --hover-bg: color-mix(in srgb, var(--background) 95%, var(--foreground) 5%);
  --focus-ring: var(--ring);
  --highlight-color: rgba(90, 200, 90, 0.2);
  
  /* Form Controls */
  --range-thumb-ring: var(--primary);
  --range-thumb-bg: var(--background);
  --range-track-bg: color-mix(in srgb, var(--primary) 40%, var(--background) 60%);
  --range-track-hover: color-mix(in srgb, var(--primary) 60%, var(--background) 40%);
}}

/* Dark theme specific variables */
[data-theme="dark"] {{
  /* Override with dark-specific values */
  --bg-color: #111827; /* Dark background */
  --text-color: #f9fafb; /* Light text */
  --border-color: #374151; /* Darker border */
  --input-bg: #1f2937; /* Darker input background */
  --card-bg: #1f2937; /* Darker card background */
  --card-text: #f9fafb; /* Light card text */
  --muted-text: #9ca3af; /* gray-400 */
  
  /* Interactive states for dark theme */
  --hover-bg: #374151; /* gray-700 */
  --highlight-color: rgba(34, 197, 94, 0.2); /* green-500 with opacity */
  
  /* Form Controls for dark theme */
  --range-thumb-ring: var(--primary);
  --range-thumb-bg: #1f2937; /* gray-800 */
  --range-track-bg: #4b5563; /* gray-600 */
  --range-track-hover: #6b7280; /* gray-500 */
}}

/* Base element styles using cascade layers */
@layer base {{
  body {{
      font-family: var(--font-sans);
      font-size: var(--font-size-base);
      line-height: var(--line-height-normal);
      background-color: var(--bg-color);
      color: var(--text-color);
  }}

  h1, h2, h3, h4, h5, h6 {{
      font-family: var(--font-heading);
  }}
}}

/* Component styling using theme variables */
@layer components {{
  /* Form elements */
  input, textarea, select {{
      background-color: var(--input-bg);
      color: var(--text-color);
      border-color: var(--border-color);
  }}
  
  /* Cards */
  .card {{
      background-color: var(--card-bg);
      color: var(--card-text);
      border-color: var(--border-color);
      box-shadow: var(--shadow-sm);
  }}
  
  /* Buttons */
  .btn-primary {{
      background-color: var(--primary-bg);
      color: var(--primary-text);
  }}
  
  .btn-secondary {{
      background-color: var(--secondary-bg);
      color: var(--secondary-text);
  }}
  
  .btn-accent {{
      background-color: var(--accent-bg);
      color: var(--accent-text);
  }}

  /* Form Range Input Styling */
  input[type="range"]::-webkit-slider-thumb {{
      cursor: pointer;
      background-color: var(--range-thumb-bg);
      box-shadow: 0 0 0 2px var(--range-thumb-ring);
  }}

  [data-theme="dark"] input[type="range"]::-webkit-slider-thumb {{
      box-shadow: 0 0 0 2px var(--range-thumb-ring), 0 0 0 1px #4b5563;
  }}

  input[type="range"]::-webkit-slider-runnable-track {{
      width: 100%;
      height: 0.125rem;
      cursor: pointer;
      background-color: var(--range-track-bg);
      border-radius: 0.25rem;
  }}

  input[type="range"]:hover::-webkit-slider-runnable-track {{
      background-color: var(--range-track-hover);
  }}
}}

/* Animation definitions */
@keyframes highlight {{
  0% {{ background-color: var(--highlight-color); }}
  100% {{ background-color: transparent; }}
}}

.highlight {{
  animation: highlight 1s ease-out;
}}"#,
			base_theme
		)
	} else {
		// Return just the base theme if theme support is not enabled
		format!(
			r#"{}

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
}}"#,
			base_theme
		)
	}
}

/// Generate CSS variables from theme with support for light/dark themes
pub fn generate_theme_css(state: &DesignerState, with_themes: bool) -> String {
	let base_theme = format!(
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
	);

	if with_themes {
		format!(
			r#"{base}

/* Light theme (default) */
:root {{
  /* Theme functional variables mapped to design tokens */
  --bg-color: var(--color-background);
  --text-color: var(--color-foreground);
  --border-color: var(--color-border);
  --input-bg: var(--color-muted);
  --card-bg: var(--color-card);
  --card-text: var(--color-card-foreground);
  
  /* Additional light theme variables */
  --highlight-color: rgba(90, 200, 90, 0.3);
  --shadow-color: rgba(0, 0, 0, 0.1);
}}

/* Dark theme specific variables */
[data-theme="dark"] {{
  /* Dark theme overrides */
  --bg-color: #111827; /* Dark background */
  --text-color: #f9fafb; /* Light text */
  --border-color: #374151; /* Darker border */
  --input-bg: #1f2937; /* Darker input background */
  --card-bg: #1f2937; /* Darker card background */
  --card-text: #f9fafb; /* Light card text */
  
  /* Additional dark theme variables */
  --highlight-color: rgba(90, 200, 90, 0.2);
  --shadow-color: rgba(0, 0, 0, 0.5);
}}

/* Base element styles */
@layer base {{
body {{
  font-family: var(--font-family);
  font-size: var(--base-size);
  line-height: var(--line-height);
  background-color: var(--bg-color);
  color: var(--text-color);
}}

h1, h2, h3, h4, h5, h6 {{
  font-family: var(--heading-font-family);
}}

/* Form controls with theme variables */
input, textarea, select {{
  background-color: var(--input-bg);
  color: var(--text-color);
  border-color: var(--border-color);
}}

/* Input customization */
input[type="range"]::-webkit-slider-thumb {{
  background-color: var(--input-bg);
}}

[data-theme="dark"] input[type="range"]::-webkit-slider-thumb {{
  @apply ring-offset-gray-800;
}}

[data-theme="light"] input[type="range"]::-webkit-slider-thumb {{
  @apply ring-offset-gray-100;
}}

/* Animations */
@keyframes highlight {{
  0% {{ background-color: var(--highlight-color); }}
  100% {{ background-color: transparent; }}
}}

.highlight {{
  animation: highlight 1s ease-out;
}}
}}
"#,
			base = base_theme
		)
	} else {
		format!(
			r#"{base}
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
			base = base_theme
		)
	}
}

/// Generate Tailwind config from theme with theme mode support
pub fn generate_tailwind_config(state: &DesignerState, with_themes: bool) -> String {
	let spacing_values = state.spacing.scale.iter().map(|(key, value)| format!("      '{}': '{}',", key, value)).collect::<Vec<String>>().join("\n");

	let base_config = format!(
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
	);

	if with_themes {
		format!(
			r#"{}
// Theme variants configuration
darkMode: ['class', '[data-theme="dark"]'],
// In v4, the PostCSS plugin and CLI are separate
plugins: [],
}}
"#,
			base_config
		)
	} else {
		format!(
			r#"{}
// In v4, the PostCSS plugin and CLI are separate
plugins: [],
}}
"#,
			base_config
		)
	}
}

/// Generate Rust code for theme
pub fn generate_rust_theme(state: &DesignerState) -> String {
	// Implementation remains the same...
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
