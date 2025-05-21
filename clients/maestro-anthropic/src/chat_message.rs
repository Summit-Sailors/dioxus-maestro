use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize, strum_macros::Display, strum_macros::AsRefStr)]
#[serde(rename_all = "snake_case")]
pub enum Role {
	User,
	Assistant,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ChatMessage {
	pub role: Role,
	pub content: String,
}
