use {
	dioxus::prelude::*,
	serde::{Deserialize, Serialize},
	tailwind_fuse::*,
};

#[derive(TwClass)]
#[tw(
	class = "inline-flex w-fit px-3 py-2 items-center justify-center gap-2 whitespace-nowrap font-medium text-foreground transition-colors ring-ring ring-offset-background focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-offset-2 disabled:pointer-events-none disabled:opacity-50 [&_svg]:pointer-events-none [&_svg]:size-4 [&_svg]:shrink-0"
)]
pub struct ButtonClass {
	pub variant: ButtonVariant,
	pub size: ButtonSize,
	pub round: ButtonRound,
}

#[derive(PartialEq, TwVariant, Debug, Serialize, Deserialize, strum_macros::Display, strum_macros::EnumIter, strum_macros::EnumString)]
pub enum ButtonRound {
	#[tw(class = "rounded-xs")]
	Xs,
	#[tw(class = "rounded-sm")]
	Sm,
	#[tw(default, class = "rounded-md")]
	Md,
	#[tw(class = "rounded-lg")]
	Lg,
	#[tw(class = "rounded-full")]
	Full,
}

impl TryFrom<&String> for ButtonRound {
	type Error = String;

	fn try_from(v: &String) -> Result<Self, Self::Error> {
		match v.as_str() {
			"Xs" => Ok(ButtonRound::Xs),
			"Sm" => Ok(ButtonRound::Sm),
			"Md" => Ok(ButtonRound::Md),
			"Lg" => Ok(ButtonRound::Lg),
			"Full" => Ok(ButtonRound::Full),
			_ => Err(format!("'{}' is not a valid ButtonRound", v)),
		}
	}
}

#[derive(TwVariant, PartialEq, Debug, Serialize, Deserialize, strum_macros::Display, strum_macros::EnumIter, strum_macros::EnumString)]
pub enum ButtonVariant {
	#[tw(default, class = "text-primary-foreground bg-primary hover:bg-primary/90")]
	Primary,
	#[tw(class = "text-secondary-foreground bg-secondary hover:bg-secondary/90")]
	Secondary,
	#[tw(class = "bg-background border border-border hover:bg-accent")]
	Outline,
	#[tw(class = "hover:bg-accent")]
	Ghost,
	#[tw(class = "text-link underline-offset-3 hover:underline")]
	Link,
	#[tw(class = "bg-danger hover:bg-danger/90")]
	Danger,
	#[tw(class = "text-muted-foreground bg-muted hover:bg-muted/90")]
	Muted,
}

impl TryFrom<&String> for ButtonVariant {
	type Error = String;

	fn try_from(v: &String) -> Result<Self, Self::Error> {
		match v.as_str() {
			"Primary" => Ok(ButtonVariant::Primary),
			"Secondary" => Ok(ButtonVariant::Secondary),
			"Outline" => Ok(ButtonVariant::Outline),
			"Ghost" => Ok(ButtonVariant::Ghost),
			"Link" => Ok(ButtonVariant::Link),
			"Danger" => Ok(ButtonVariant::Danger),
			"Muted" => Ok(ButtonVariant::Muted),
			_ => Err(format!("'{}' is not a valid ButtonVariant", v)),
		}
	}
}

#[derive(TwVariant, PartialEq, Debug, Serialize, Deserialize, strum_macros::Display, strum_macros::EnumIter, strum_macros::EnumString)]
pub enum ButtonSize {
	#[tw(class = "h-8")]
	Xs,
	#[tw(class = "h-9")]
	Sm,
	#[tw(default, class = "h-10")]
	Md,
	#[tw(class = "h-11")]
	Lg,
	#[tw(class = "h-12")]
	Xl,
}

impl TryFrom<&String> for ButtonSize {
	type Error = String;

	fn try_from(v: &String) -> Result<Self, Self::Error> {
		match v.as_str() {
			"Xs" => Ok(ButtonSize::Xs),
			"Sm" => Ok(ButtonSize::Sm),
			"Md" => Ok(ButtonSize::Md),
			"Lg" => Ok(ButtonSize::Lg),
			"Xl" => Ok(ButtonSize::Xl),
			_ => Err(format!("'{}' is not a valid ButtonSize", v)),
		}
	}
}
