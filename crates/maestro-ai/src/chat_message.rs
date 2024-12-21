use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ChatMessage {
	pub content: String,
	pub is_user: bool,
}

#[cfg(feature = "server")]
use misanthropic::prompt::{
	message::{Content, Role},
	Message,
};

#[cfg(feature = "server")]
impl<'a> From<ChatMessage> for Message<'a> {
	fn from(val: ChatMessage) -> Self {
		Message { role: if val.is_user { Role::User } else { Role::Assistant }, content: Content::<'a>::SinglePart(val.content.to_owned().into()) }
	}
}
