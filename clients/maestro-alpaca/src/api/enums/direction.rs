use serde::{Deserialize, Serialize};

#[derive(Clone, Copy, Debug, Default, Deserialize, Eq, PartialEq, Serialize)]
pub enum Direction {
	#[serde(rename = "desc")]
	#[default]
	Descending,
	#[serde(rename = "asc")]
	Ascending,
}
