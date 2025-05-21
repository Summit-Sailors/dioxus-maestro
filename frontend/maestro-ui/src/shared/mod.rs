pub use maestro_headless::shared::*;
use {
	serde::{Deserialize, Serialize},
	tailwind_fuse::*,
};

#[derive(TwClass)]
#[tw(
	class = "inline-flex w-fit px-3 py-2 items-center justify-center gap-2 whitespace-nowrap font-medium text-foreground transition-colors ring-ring ring-offset-background focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-offset-2 disabled:pointer-events-none disabled:opacity-50 [&_svg]:pointer-events-none [&_svg]:size-4 [&_svg]:shrink-0"
)]
pub struct EClass {
	pub variant: EVariant,
	pub size: ESize,
	pub round: ERound,
}

#[derive(Debug, PartialEq, Serialize, Deserialize, strum_macros::EnumIter, strum_macros::Display, strum_macros::EnumString, TwVariant)]
pub enum ERound {
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
	#[tw(class = "")]
	None,
}

#[derive(Debug, PartialEq, Serialize, Deserialize, strum_macros::EnumIter, strum_macros::Display, strum_macros::EnumString, TwVariant)]
pub enum EVariant {
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

#[derive(Debug, PartialEq, Serialize, Deserialize, strum_macros::EnumIter, strum_macros::Display, strum_macros::EnumString, TwVariant)]
pub enum ESize {
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
