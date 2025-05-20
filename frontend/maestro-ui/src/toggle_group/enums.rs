use {
	crate::{shared::EOrientation, toggle::ToggleSize},
	dioxus::prelude::*,
	serde::{Deserialize, Serialize},
	tailwind_fuse::*,
};

#[derive(TwClass)]
#[tw(
	class = "min-w-0 flex-1 flex items-center justify-center shrink-0 rounded-none shadow-none transition-colors focus-visible:outline-none focus-visible:bg-accent disabled:pointer-events-none disabled:opacity-50 [&_svg]:pointer-events-none [&_svg]:size-4 [&_svg]:shrink-0"
)]
pub struct ToggleItemClass {
	pub variant: ToggleItemVariant,
	pub size: ToggleSize,
	pub round: ToggleItemRound,
}

#[derive(Debug, Deserialize, strum_macros::Display, strum_macros::EnumIter, strum_macros::EnumString, PartialEq, Serialize, TwVariant)]
pub enum ToggleItemRound {
	#[tw(class = "first:rounded-l-xs last:rounded-r-xs")]
	Xs,
	#[tw(class = "first:rounded-l-sm last:rounded-r-sm")]
	Sm,
	#[tw(default, class = "first:rounded-l-md last:rounded-r-md")]
	Md,
	#[tw(class = "first:rounded-l-lg last:rounded-r-lg")]
	Lg,
}

#[derive(Debug, Deserialize, strum_macros::Display, strum_macros::EnumIter, strum_macros::EnumString, PartialEq, Serialize, TwVariant)]
pub enum ToggleItemVariant {
	#[tw(
		default,
		class = "bg-background border border-border hover:bg-accent border-l-0 first:border-l data-[state=on]:bg-primary data-[state=on]:text-primary-foreground"
	)]
	Outline,
	#[tw(class = "hover:bg-accent data-[state=on]:bg-primary data-[state=on]:text-primary-foreground")]
	Ghost,
}
