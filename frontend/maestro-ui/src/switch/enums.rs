use {
	serde::{Deserialize, Serialize},
	tailwind_fuse::*,
};

#[derive(TwClass)]
#[tw(
	class = "peer data-[state=checked]:bg-primary data-[state=unchecked]:bg-muted focus-visible:ring-ring inline-flex shrink-0 items-center border border-transparent shadow-xs transition-all outline-none focus-visible:ring-2 focus-visible:ring-offset-2 focus-visible:ring-offset-background disabled:cursor-not-allowed disabled:pointer-events-none disabled:opacity-50 peer-has-checked:hidden"
)]
pub struct SwitchClass {
	pub size: SwitchSize,
	pub round: SwitchRound,
}

#[derive(Debug, PartialEq, Serialize, Deserialize, strum_macros::EnumIter, strum_macros::Display, strum_macros::EnumString, TwVariant)]
pub enum SwitchRound {
	#[tw(class = "rounded-xs")]
	Sm,
	#[tw(class = "rounded-sm")]
	Md,
	#[tw(class = "rounded-md")]
	Lg,
	#[tw(default, class = "rounded-full")]
	Full,
}

#[derive(Debug, PartialEq, Serialize, Deserialize, strum_macros::EnumIter, strum_macros::Display, strum_macros::EnumString, TwVariant)]
pub enum SwitchSize {
	#[tw(default, class = "w-8 h-4.5")]
	Sm,
	#[tw(class = "w-9 h-5")]
	Md,
	#[tw(class = "w-10 h-6")]
	Lg,
}
