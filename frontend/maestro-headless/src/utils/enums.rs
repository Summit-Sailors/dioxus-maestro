use serde::{Deserialize, Serialize};

// TO DO: use for key nav
#[derive(Clone, PartialEq, Debug, Copy, Serialize, Deserialize, strum_macros::Display)]
pub enum EGroupOrientation {
	#[strum(to_string = "horizontal")]
	Horizontal,
	#[strum(to_string = "vertical")]
	Vertical,
}

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum ESide {
	Top,
	Right,
	Bottom,
	Left,
}

impl ESide {
	pub fn opposite(&self) -> Self {
		match self {
			ESide::Top => ESide::Bottom,
			ESide::Right => ESide::Left,
			ESide::Bottom => ESide::Top,
			ESide::Left => ESide::Right,
		}
	}
}

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum EAlign {
	Start,
	Center,
	End,
}
