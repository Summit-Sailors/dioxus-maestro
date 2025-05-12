use super::ThemeOptions;
use crate::maestro_themes::{editor::state::DesignerState, exporter::component_styles::get_component_styles};

/// Generate Tailwind v4 CSS configuration with theme support
pub fn generate_tailwind_v4_css(state: &DesignerState, theme_options: ThemeOptions) -> String {
	let component_specific_styles = get_component_styles(&theme_options.components_id).expect("An error occurred while getting component styles");
	let base_elements_styles = format!(
		r#"/* Custom z-index values for toast elements */
@layer utilities {{
  .z-toast {{
    z-index: 9999 !important;
    }}

  .line-clamp-1,
  .line-clamp-2,
  .line-clamp-3,
  .line-clamp-4 {{
    -webkit-line-clamp: var(--tw-line-clamp);
    line-clamp: var(--tw-line-clamp);
    }}
    }}

@keyframes highlight {{
  0% {{
    background: #8f8;
    }}
  100% {{
    background: auto;
    }}
    }}

.highlight {{
  animation: highlight 1s;
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

  h1,
  h2,
  h3,
  h4,
  h5,
  h6 {{
    font-family: var(--font-heading);
    }}

  html {{
    scroll-behavior: smooth;
    height: 100%;
    min-height: 100vh;
    }}

  .container {{
    @apply mx-auto px-4;
    }}

  /* Container media queries remain unchanged */
  @media (min-width: 640px) {{
    .container {{
      max-width: 584px;
      padding: 0;
    }}
    }}

  @media (min-width: 768px) {{
    .container {{
      max-width: 768px;
    }}
    }}

  @media (min-width: 1024px) {{
    .container {{
      max-width: 854px;
    }}
    }}

  @media (min-width: 1280px) {{
    .container {{
      max-width: 1066px;
    }}
    }}

  @media (min-width: 1536px) {{
    .container {{
      max-width: 1260px;
    }}
    }}

  @media (min-width: 1728px) {{
    .container {{
      max-width: 1440px;
    }}
    }}
  }}
  {component_specific_styles}"#
	);
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
		state.typography.font_weights.iter().map(|(name, weight)| format!("  --font-weight-{name}: {weight};")).collect::<Vec<String>>().join("\n"),
		state.spacing.unit,
		state.spacing.scale.iter().map(|(key, value)| format!("  --spacing-{key}: {value};")).collect::<Vec<String>>().join("\n"),
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
	if theme_options.with_doc_themes {
		format!(
			r#"{base_theme}

/* Dark variant based on data-theme attribute */
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
    {base_elements_styles}
"#
		)
	} else {
		// just the base theme if theme support is not enabled
		format!(
			r#"{base_theme}
    {base_elements_styles}"#
		)
	}
}

/// Generate CSS variables from theme with support for light/dark themes
pub fn generate_theme_variables(state: &DesignerState) -> String {
	let base_theme = format!(
		r#"/**
* Generated theme CSS variables
* Compatible with Tailwind CSS v4
*/

/* Theme configuration using CSS variables */
@theme {{
/* Colors */
--primary: {};
--primary-foreground: white;
--secondary: {};
--secondary-foreground: white;
--accent: {};
--accent-foreground: white;
--bg-color: {};
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
		state.typography.font_weights.iter().map(|(name, weight)| format!("  --font-weight-{name}: {weight};")).collect::<Vec<String>>().join("\n"),
		state.spacing.unit,
		state.spacing.scale.iter().map(|(key, value)| format!("  --spacing-{key}: {value};")).collect::<Vec<String>>().join("\n"),
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
	base_theme
}

/// Generate Tailwind config from theme with theme mode support (for Tailwind v3  and below)
pub fn generate_tailwind_config(state: &DesignerState, with_themes: bool) -> String {
	// font weights mapping
	let font_weights = state.typography.font_weights.iter().map(|(name, weight)| format!(" '{name}': '{weight}',")).collect::<Vec<String>>().join("\n");

	// spacing values mapping
	let spacing_values = state.spacing.scale.iter().map(|(key, value)| format!(" '{key}': '{value}',")).collect::<Vec<String>>().join("\n");

	let base_config = format!(
		r#"/**
* Generated Tailwind CSS configuration
* For Tailwind v3 compatibility
*/
module.exports = {{
theme: {{
  extend: {{
    colors: {{
      primary: '{}',
      'primary-foreground': 'white',
      secondary: '{}',
      'secondary-foreground': 'white',
      accent: '{}',
      'accent-foreground': 'white',
      background: '{}',
      foreground: '{}',
      card: '{}',
      'card-foreground': '{}',
      border: '{}',
      ring: '{}',
      destructive: '{}',
      'destructive-foreground': '{}',
      muted: '{}',
      'muted-foreground': '{}',
      // Add theme UI mapping for consistent variables
      'hover-bg': 'color-mix(in srgb, var(--background) 95%, var(--foreground) 5%)',
      'highlight': 'rgba(90, 200, 90, 0.2)',
    }},
    fontFamily: {{
      sans: ['{}'],
      heading: ['{}'],
    }},
    fontSize: {{
      base: '{}',
    }},
    lineHeight: {{
      normal: '{}',
    }},
    fontWeight: {{
{}
    }},
    spacing: {{
      unit: '{}',
{}
    }},
    borderRadius: {{
      sm: '{}',
      DEFAULT: '{}',
      md: '{}',
      lg: '{}',
      xl: '{}',
      full: '{}',
    }},
    boxShadow: {{
      sm: '{}',
      DEFAULT: '{}',
      md: '{}',
      lg: '{}',
      xl: '{}',
      '2xl': '{}',
    }},
    // Add animation from v4 example
    keyframes: {{
      highlight: {{
        '0%': {{ backgroundColor: 'var(--highlight-color, rgba(90, 200, 90, 0.2))' }},
        '100%': {{ backgroundColor: 'transparent' }},
      }},
    }},
    animation: {{
      highlight: 'highlight 1s ease-out',
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
		font_weights,
		state.spacing.unit,
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

// Form element styling
plugins: [
  function(addComponents) {{
    addComponents({{
      '.card': {{
        backgroundColor: 'var(--card, white)',
        color: 'var(--card-foreground, #020617)',
        borderColor: 'var(--border, #e2e8f0)',
        boxShadow: 'var(--shadow-sm)',
      }},
      '.btn-primary': {{
        backgroundColor: 'var(--primary, #3b82f6)',
        color: 'var(--primary-foreground, white)',
      }},
      '.btn-secondary': {{
        backgroundColor: 'var(--secondary, #6b7280)',
        color: 'var(--secondary-foreground, white)',
      }},
      '.btn-accent': {{
        backgroundColor: 'var(--accent, #f59e0b)',
        color: 'var(--accent-foreground, white)',
      }},
      'input[type="range"]::-webkit-slider-thumb': {{
        cursor: 'pointer',
        backgroundColor: 'var(--range-thumb-bg, var(--background))',
        boxShadow: '0 0 0 2px var(--range-thumb-ring, var(--primary))',
      }},
      'input[type="range"]::-webkit-slider-runnable-track': {{
        width: '100%',
        height: '0.125rem',
        cursor: 'pointer',
        backgroundColor: 'var(--range-track-bg, color-mix(in srgb, var(--primary) 40%, var(--background) 60%))',
        borderRadius: '0.25rem',
      }},
      'input[type="range"]:hover::-webkit-slider-runnable-track': {{
        backgroundColor: 'var(--range-track-hover, color-mix(in srgb, var(--primary) 60%, var(--background) 40%))',
      }},
      '.highlight': {{
        animation: 'highlight 1s ease-out',
      }}
    }})
  }},
  function(addBase) {{
    addBase({{
      'body': {{
        fontFamily: 'var(--font-sans, {})',
        fontSize: 'var(--font-size-base, {})',
        lineHeight: 'var(--line-height-normal, {})',
        backgroundColor: 'var(--background, white)',
        color: 'var(--foreground, #020617)',
      }},
      'h1, h2, h3, h4, h5, h6': {{
        fontFamily: 'var(--font-heading, {})',
      }},
      // Add dark mode specific styles
      '[data-theme="dark"]': {{
        '--background': '#111827',
        '--foreground': '#f9fafb',
        '--border': '#374151',
        '--muted': '#1f2937',
        '--card': '#1f2937',
        '--card-foreground': '#f9fafb',
        '--muted-foreground': '#9ca3af',
        '--hover-bg': '#374151',
        '--highlight-color': 'rgba(34, 197, 94, 0.2)',
        '--range-thumb-bg': '#1f2937',
        '--range-track-bg': '#4b5563',
        '--range-track-hover': '#6b7280',
      }}
    }})
  }},
],
}}
"#,
			base_config, state.typography.font_family, state.typography.base_size, state.typography.line_height, state.typography.heading_font_family
		)
	} else {
		format!(
			r#"{}
plugins: [
  function(addBase) {{
    addBase({{
      'body': {{
        fontFamily: 'var(--font-sans, {})',
        fontSize: 'var(--font-size-base, {})',
        lineHeight: 'var(--line-height-normal, {})',
        backgroundColor: 'var(--background, white)',
        color: 'var(--foreground, #020617)',
      }},
      'h1, h2, h3, h4, h5, h6': {{
        fontFamily: 'var(--font-heading, {})',
      }}
    }})
  }},
],
}}
"#,
			base_config, state.typography.font_family, state.typography.base_size, state.typography.line_height, state.typography.heading_font_family
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
		.map(|(name, weight)| format!("    font_weights.insert(String::from(\"{name}\"), {weight});"))
		.collect::<Vec<String>>()
		.join("\n");

	let spacing_scale = state
		.spacing
		.scale
		.iter()
		.map(|(key, value)| format!("    scale.insert(String::from(\"{key}\"), String::from(\"{value}\"));"))
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
