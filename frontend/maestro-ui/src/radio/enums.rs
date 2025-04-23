use {
	dioxus::prelude::*,
	serde::{Deserialize, Serialize},
	tailwind_fuse::*,
};

#[derive(TwClass)]
#[tw(
	class = "group inline-flex items-center justify-center rounded-full text-primary border border-input data-[state=checked]:border-primary transition-colors ring-ring ring-offset-background focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-offset-2 disabled:pointer-events-none disabled:opacity-50 [&_svg]:pointer-events-none [&_svg]:size-4 [&_svg]:shrink-0 [&_span]:pointer-events-none [&_span]:size-4 [&_span]:shrink-0"
)]
pub struct RadioClass {
	pub size: RadioSize,
}

#[derive(TwVariant, PartialEq, Debug, Serialize, Deserialize, strum_macros::Display, strum_macros::EnumIter, strum_macros::EnumString)]
pub enum RadioSize {
	#[tw(class = "w-5 h-5")]
	Sm,
	#[tw(default, class = "w-6 h-6")]
	Md,
	#[tw(class = "w-7 h-7")]
	Lg,
}

#[derive(TwClass)]
#[tw(class = "h-4 w-4 transition-all")]
pub struct RadioIndicatorClass {
	pub variant: RadioIndicatorVariant,
}

#[derive(TwVariant, PartialEq, Debug, Serialize, Deserialize, strum_macros::Display, strum_macros::EnumIter, strum_macros::EnumString)]
pub enum RadioIndicatorVariant {
	#[tw(class = "text-primary")]
	Tick,
	#[tw(default, class = "rounded-full bg-primary")]
	Circle,
}
