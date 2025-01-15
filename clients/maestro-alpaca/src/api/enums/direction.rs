use serde::{Deserialize, Serialize};

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum Direction {
	#[serde(rename = "desc")]
	#[default]
	Descending,
	#[serde(rename = "asc")]
	Ascending,
}
