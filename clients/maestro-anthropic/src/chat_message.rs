use serde::{Deserialize, Serialize};

#[derive(strum_macros::AsRefStr, Clone, Copy, Debug, Deserialize, strum_macros::Display, PartialEq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum Role {
	User,
	Assistant,
}

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct ChatMessage {
	pub role: Role,
	pub content: String,
}
