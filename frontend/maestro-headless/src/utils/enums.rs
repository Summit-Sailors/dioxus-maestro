use serde::{Deserialize, Serialize};

// TO DO: use for key nav
#[derive(Clone, PartialEq, Debug, Copy, Serialize, Deserialize, strum_macros::Display)]
pub enum EGroupOrientation {
	#[strum(to_string = "horizontal")]
	Horizontal,
	#[strum(to_string = "vertical")]
	Vertical,
}
