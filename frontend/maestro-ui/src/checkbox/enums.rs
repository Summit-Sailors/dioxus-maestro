use {
	dioxus::prelude::*,
	serde::{Deserialize, Serialize},
	tailwind_fuse::*,
};

#[derive(TwClass)]
#[tw(
	class = "group inline-flex items-center justify-center text-foreground data-[state=checked]:bg-primary data-[state=checked]:text-primary-foreground border border-input data-[state=checked]:border-primary transition-colors ring-ring ring-offset-background focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-offset-2 disabled:pointer-events-none disabled:opacity-50 [&_svg]:pointer-events-none [&_svg]:size-4 [&_svg]:shrink-0 [&_span]:pointer-events-none [&_span]:size-4 [&_span]:shrink-0"
)]
pub struct CheckboxClass {
	pub size: CheckboxSize,
	pub round: CheckboxRound,
}

#[derive(PartialEq, TwVariant, Debug, Serialize, Deserialize, strum_macros::Display, strum_macros::EnumIter, strum_macros::EnumString)]
pub enum CheckboxRound {
	#[tw(class = "rounded-xs")]
	Sm,
	#[tw(default, class = "rounded-sm")]
	Md,
	#[tw(class = "rounded-md")]
	Lg,
}

impl TryFrom<&String> for CheckboxRound {
	type Error = String;

	fn try_from(v: &String) -> Result<Self, Self::Error> {
		match v.as_str() {
			"Sm" => Ok(CheckboxRound::Sm),
			"Md" => Ok(CheckboxRound::Md),
			"Lg" => Ok(CheckboxRound::Lg),
			_ => Err(format!("'{}' is not a valid CheckboxRound", v)),
		}
	}
}

#[derive(TwVariant, PartialEq, Debug, Serialize, Deserialize, strum_macros::Display, strum_macros::EnumIter, strum_macros::EnumString)]
pub enum CheckboxSize {
	#[tw(class = "w-5 h-5")]
	Sm,
	#[tw(default, class = "w-6 h-6")]
	Md,
	#[tw(class = "w-7 h-7")]
	Lg,
}

impl TryFrom<&String> for CheckboxSize {
	type Error = String;

	fn try_from(v: &String) -> Result<Self, Self::Error> {
		match v.as_str() {
			"Sm" => Ok(CheckboxSize::Sm),
			"Md" => Ok(CheckboxSize::Md),
			"Lg" => Ok(CheckboxSize::Lg),
			_ => Err(format!("'{}' is not a valid CheckboxSize", v)),
		}
	}
}

#[derive(TwClass)]
#[tw(class = "h-4 w-4 transition-all")]
pub struct CheckboxIndicatorClass {
	pub variant: CheckboxIndicatorVariant,
}

#[derive(TwVariant, PartialEq, Debug, Serialize, Deserialize, strum_macros::Display, strum_macros::EnumIter, strum_macros::EnumString)]
pub enum CheckboxIndicatorVariant {
	#[tw(default, class = "text-primary-foreground")]
	Tick,
	#[tw(class = "rounded-xs bg-primary-foreground")]
	Square,
}

impl TryFrom<&String> for CheckboxIndicatorVariant {
	type Error = String;

	fn try_from(v: &String) -> Result<Self, Self::Error> {
		match v.as_str() {
			"Tick" => Ok(CheckboxIndicatorVariant::Tick),
			"Square" => Ok(CheckboxIndicatorVariant::Square),
			_ => Err(format!("'{}' is not a valid CheckboxVariant", v)),
		}
	}
}
