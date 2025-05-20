use serde::{Deserialize, Serialize};

#[derive(Clone, Copy, Debug, Default, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "lowercase")]
pub enum Adjustment {
	Raw,
	Split,
	Dividend,
	#[default]
	All,
}
