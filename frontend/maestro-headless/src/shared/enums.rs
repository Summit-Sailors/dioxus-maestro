use serde::{Deserialize, Serialize};

// TO DO: use for key nav
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize, strum_macros::Display)]
pub enum EOrientation {
	#[strum(to_string = "horizontal")]
	Horizontal,
	#[strum(to_string = "vertical")]
	Vertical,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, strum_macros::Display)]
pub enum ESide {
	#[strum(to_string = "top")]
	Top,
	#[strum(to_string = "right")]
	Right,
	#[strum(to_string = "bottom")]
	Bottom,
	#[strum(to_string = "left")]
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

impl TryFrom<&String> for ESide {
	type Error = String;

	fn try_from(v: &String) -> Result<Self, Self::Error> {
		match v.as_str() {
			"top" => Ok(ESide::Top),
			"right" => Ok(ESide::Right),
			"bottom" => Ok(ESide::Bottom),
			"left" => Ok(ESide::Left),
			_ => Err(format!("'{}' is not a valid ESide", v)),
		}
	}
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, strum_macros::Display)]
pub enum EAlign {
	#[strum(to_string = "start")]
	Start,
	#[strum(to_string = "center")]
	Center,
	#[strum(to_string = "end")]
	End,
}

impl TryFrom<&String> for EAlign {
	type Error = String;

	fn try_from(v: &String) -> Result<Self, Self::Error> {
		match v.as_str() {
			"start" => Ok(EAlign::Start),
			"center" => Ok(EAlign::Center),
			"end" => Ok(EAlign::End),
			_ => Err(format!("'{}' is not a valid EAlign", v)),
		}
	}
}
