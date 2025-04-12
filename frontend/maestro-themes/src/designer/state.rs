// Designer state management

use {
	serde::{Deserialize, Serialize},
	std::collections::HashMap,
};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct DesignerState {
	/// Color palette for the theme
	pub color: ColorPalette,
	/// Typography settings
	pub typography: Typographysettings,
	/// SpacingScale
	pub spacing: SpacingScale,
	/// Border radius settings
	pub border_radius: BorderRadiusSettings,
	// Shadow settings
	pub shadow: ShadowSettings,
}

impl Default for DesignerState {
	fn default() -> Self {
		Self {
			color: ColorPalette::default(),
			typography: Typographysettings::default(),
			spacing: SpacingScale::default(),
			border_radius: BorderRadiusSettings::default(),
			shadow: Shadowsettings::default(),
		}
	}
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ColorPalette {
	pub primary: String,
	pub secondary: String,
	pub accent: String,
	pub background: String,
	pub foreground: String,
	pub card: String,
	pub card_foreground: String,
	pub border: String,
	pub ring: String,
	pub destructive: String,
	pub destructive_foreground: String,
	pub muted: String,
	pub muted_foreground: String,
}

impl Default for ColorPalette {
	fn default() -> Self {
		Self {
			primary: String::from("#0070f3"),
			secondary: String::from("#383838"),
			accent: String::from("#7928CA"),
			background: String::from("#fff"),
			foreground: String::from("#000"),
			card: String::from("#f9f9f9"),
			card_foreground: String::from("#111"),
			border: String::from("#eaeaea"),
			ring: String::from("#BBBBBB"),
			destructive: String::from("#ef4444"),
			destructive_foreground: String::from("#fff"),
			muted: String::from("#f3f4f6"),
			muted_foreground: String::from("#64748b"),
		}
	}
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Typographysettings {
	pub font_family: String,
	pub heading_font_family: String,
	pub base_size: String,
	pub line_height: String,
	pub font_weights: HashMap<String, u32>,
}

impl Default for Typographysettings {
	fn default() -> Self {
		let mut font_weights = HashMap::new();
		font_weights.insert(String::from("light"), 300);
		font_weights.insert(String::from("normal"), 400);
		font_weights.insert(String::from("medium"), 500);
		font_weights.insert(String::from("semibold"), 600);
		font_weights.insert(String::from("bold"), 700);

		Self {
			font_family: String::from("Inter, system-ui, sans-serif"),
			heading_font_family: String::from("Inter, system-ui, sans-serif"),
			base_size: String::from("16px"),
			line_height: String::from("1.5"),
			font_weights,
		}
	}
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SpacingScale {
	pub unit: String,
	pub scale: HashMap<String, String>,
}

impl Default for SpacingScale {
	fn default() -> Self {
		let mut scale = HashMap::new();
		scale.insert(String::from("0"), String::from("0px"));
		scale.insert(String::from("1"), String::from("0.25rem"));
		scale.insert(String::from("2"), String::from("0.5rem"));
		scale.insert(String::from("3"), String::from("0.75rem"));
		scale.insert(String::from("4"), String::from("1rem"));
		scale.insert(String::from("5"), String::from("1.25rem"));
		scale.insert(String::from("6"), String::from("1.5rem"));
		scale.insert(String::from("8"), String::from("2rem"));
		scale.insert(String::from("10"), String::from("2.5rem"));
		scale.insert(String::from("12"), String::from("3rem"));
		scale.insert(String::from("16"), String::from("4rem"));

		Self { unit: String::from("rem"), scale }
	}
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct BorderRadiusSettings {
	pub sm: String,
	pub md: String,
	pub lg: String,
	pub xl: String,
	pub full: String,
}

impl Default for BorderRadiusSettings {
	fn default() -> Self {
		Self { sm: String::from("0.125rem"), md: String::from("0.25rem"), lg: String::from("0.5rem"), xl: String::from("1rem"), full: String::from("9999px") }
	}
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ShadowSettings {
	pub sm: String,
	pub md: String,
	pub lg: String,
	pub xl: String,
	pub xxl: String,
}

impl Default for ShadowSettings {
	fn default() -> Self {
		Self {
			sm: String::from("0 1px 2px 0 rgb(0 0 0 / 0.05)"),
			md: String::from("0 4px 6px -1px rgb(0 0 0 / 0.1), 0 2px 4px -2px rgb(0 0 0 / 0.05)"),
			lg: String::from("0 10px 15px -3px rgb(0 0 0 / 0.1), 0 4px 6px -4px rgb(0 0 0 / 0.05)"),
			xl: String::from("0 20px 25px -5px rgb(0 0 0 / 0.1), 0 8px 10px -6px rgb(0 0 0 / 0.05)"),
			xxl: String::from("0 25px 50px -12px rgb(0 0 0 / 0.25)"),
		}
	}
}

#[derive(Clone, Debug)]
pub enum ThemedesignerAction {
	UpdateColor { key: String, value: String },
	UpdateFontFamiliy { value: String },
	UpdateHeadingFontFamily { value: String },
	UpdateFontSize { value: String },
	UpdateLineHeight { value: String },
	UpdateFontWight { name: String, value: u32 },
	UpdateSpacingUnit { value: String },
	UpdateSpacingValue { key: String, value: String },
	UpdateBorderRadius { key: String, value: String },
	UpdateShadow { key: String, value: String },
	ResetToDefaults,
}
impl DesignerState {
	pub fn apply_action(&mut self, action: ThemedesignerAction) {
		match action {
			ThemedesignerAction::UpdateColor { key, value } => match key.as_str() {
				"primary" => self.color.primary = value,
				"secondary" => self.color.secondary = value,
				"accent" => self.color.accent = value,
				"background" => self.color.background = value,
				"foreground" => self.color.foreground = value,
				"card" => self.color.card = value,
				"card_foreground" => self.color.card_foreground = value,
				"border" => self.color.border = value,
				"ring" => self.color.ring = value,
				"destructive" => self.color.destructive = value,
				"destructive_foreground" => self.color.destructive_foreground = value,
				"muted" => self.color.muted = value,
				"muted_foreground" => self.color.muted_foreground = value,
				_ => println!("Unknown color key: {}", key),
			},
			ThemedesignerAction::UpdateFontFamiliy { value } => {
				self.typography.font_family = value;
			},
			ThemedesignerAction::UpdateHeadingFontFamily { value } => {
				self.typography.heading_font_family = value;
			},
			ThemedesignerAction::UpdateFontSize { value } => {
				self.typography.base_size = value;
			},
			ThemedesignerAction::UpdateLineHeight { value } => {
				self.typography.line_height = value;
			},
			ThemedesignerAction::UpdateFontWight { name, value } => {
				self.typography.font_weights.insert(name, value);
			},
			ThemedesignerAction::UpdateSpacingUnit { value } => {
				self.spacing.unit = value;
			},
			ThemedesignerAction::UpdateSpacingValue { key, value } => {
				self.spacing.scale.insert(key, value);
			},
			ThemedesignerAction::UpdateBorderRadius { key, value } => match key.as_str() {
				"sm" => self.border_radius.sm = value,
				"md" => self.border_radius.md = value,
				"lg" => self.border_radius.lg = value,
				"xl" => self.border_radius.xl = value,
				"full" => self.border_radius.full = value,
				_ => println!("Unknown border radius key: {}", key),
			},
			ThemedesignerAction::UpdateShadow { key, value } => match key.as_str() {
				"sm" => self.shadow.sm = value,
				"md" => self.shadow.md = value,
				"lg" => self.shadow.lg = value,
				"xl" => self.shadow.xl = value,
				"xxl" => self.shadow.xxl = value,
				_ => println!("Unknown shadow key: {}", key),
			},
			ThemedesignerAction::ResetToDefaults => {
				*self = DesignerState::default();
			},
		}
	}
}
