use {
	serde::{Deserialize, Serialize},
	tailwind_fuse::*,
};

#[derive(TwClass)]
#[tw(class = "flex items-center justify-center rounded-full bg-muted border border-border overflow-hidden")]
pub struct AvatarClass {
	pub size: AvatarSize,
}

#[derive(Debug, PartialEq, Serialize, Deserialize, strum_macros::EnumIter, strum_macros::Display, strum_macros::EnumString, TwVariant)]
pub enum AvatarSize {
	#[tw(class = "w-8 h-8")]
	Xs,
	#[tw(class = "w-9 h-9")]
	Sm,
	#[tw(default, class = "w-10 h-10")]
	Md,
	#[tw(class = "w-11 h-11")]
	Lg,
	#[tw(class = "w-12 h-12")]
	Xl,
}

impl TryFrom<&String> for AvatarSize {
	type Error = String;

	fn try_from(v: &String) -> Result<Self, Self::Error> {
		match v.as_str() {
			"Xs" => Ok(AvatarSize::Xs),
			"Sm" => Ok(AvatarSize::Sm),
			"Md" => Ok(AvatarSize::Md),
			"Lg" => Ok(AvatarSize::Lg),
			"Xl" => Ok(AvatarSize::Xl),
			_ => Err(format!("'{}' is not a valid ESize", v)),
		}
	}
}
