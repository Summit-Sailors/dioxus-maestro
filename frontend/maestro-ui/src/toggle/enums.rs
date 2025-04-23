use {
	dioxus::prelude::*,
	serde::{Deserialize, Serialize},
	tailwind_fuse::*,
};

#[derive(TwClass)]
#[tw(
	class = "inline-flex items-center justify-center gap-2 whitespace-nowrap font-medium text-foreground transition-colors ring-ring ring-offset-background focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-offset-2 disabled:pointer-events-none disabled:opacity-50 [&_svg]:pointer-events-none [&_svg]:size-4 [&_svg]:shrink-0"
)]
pub struct ToggleClass {
	pub variant: ToggleVariant,
	pub size: ToggleSize,
	pub round: ToggleRound,
}

#[derive(PartialEq, TwVariant, Debug, Serialize, Deserialize, strum_macros::Display, strum_macros::EnumIter, strum_macros::EnumString)]
pub enum ToggleRound {
	#[tw(class = "rounded-xs")]
	Xs,
	#[tw(class = "rounded-sm")]
	Sm,
	#[tw(default, class = "rounded-md")]
	Md,
	#[tw(class = "rounded-lg")]
	Lg,
}

#[derive(TwVariant, PartialEq, Debug, Serialize, Deserialize, strum_macros::Display, strum_macros::EnumIter, strum_macros::EnumString)]
pub enum ToggleVariant {
	#[tw(default, class = "bg-background border border-border hover:bg-accent data-[state=on]:bg-primary data-[state=on]:text-primary-foreground")]
	Outline,
	#[tw(class = "hover:bg-accent data-[state=on]:bg-primary data-[state=on]:text-primary-foreground")]
	Ghost,
}

#[derive(TwVariant, PartialEq, Debug, Serialize, Deserialize, strum_macros::Display, strum_macros::EnumIter, strum_macros::EnumString)]
pub enum ToggleSize {
	#[tw(class = "h-6 w-6")]
	Xs,
	#[tw(class = "h-7 w-7")]
	Sm,
	#[tw(default, class = "h-8 w-8")]
	Md,
	#[tw(class = "h-9 w-9")]
	Lg,
	#[tw(class = "h-10 w-10")]
	Xl,
}
