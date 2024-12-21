use {
	crate::{chat_message::ChatMessage, stream_ai::stream_chat},
	dioxus::prelude::*,
	futures::StreamExt,
};

pub fn use_ai_chat(
	init_input: impl FnOnce() -> String, init_messages: impl FnOnce() -> Vec<ChatMessage>, init_temperature: impl FnOnce() -> f32,
	init_system_prompt: impl FnOnce() -> String, init_extra: impl FnOnce() -> String,
) -> AiChatState {
	let user_input = use_signal(init_input);
	let extra = use_signal(init_extra);
	let error = use_signal(|| None);
	let messages = use_signal(init_messages);
	let temperature = use_signal(init_temperature);
	let system_prompt = use_signal(init_system_prompt);
	AiChatState { user_input, extra, error, messages, temperature, system_prompt }
}

#[derive(Clone, Copy)]
pub struct AiChatState {
	pub user_input: Signal<String>,
	pub extra: Signal<String>,
	pub error: Signal<Option<ServerFnError>>,
	pub messages: Signal<Vec<ChatMessage>>,
	pub temperature: Signal<f32>,
	pub system_prompt: Signal<String>,
}

impl AiChatState {
	pub fn clear_chat_history(&mut self) {
		self.user_input.set("".to_string());
		self.error.set(None);
		self.messages.clear();
	}

	pub async fn stream_chat(&mut self) {
		let mut messages = self.messages.write();
		let mut server_messages = messages.clone();
		messages.extend([ChatMessage { is_user: true, content: self.user_input.to_string() }, ChatMessage { is_user: false, content: "".into() }]);
		server_messages
			.extend([ChatMessage { is_user: true, content: format!("{}\n\n{}", self.extra, self.user_input) }, ChatMessage { is_user: false, content: "".into() }]);
		match crate::stream_ai::stream_chat(server_messages.to_owned(), *self.temperature.read(), self.system_prompt.read().clone()).await {
			Ok(stream_wrapper) => {
				self.user_input.set(String::new());
				let mut stream = stream_wrapper.into_inner();
				while let Some(Ok(text)) = stream.next().await {
					if let Some(last) = messages.last_mut() {
						last.content.push_str(&text);
					}
				}
			},
			Err(e) => self.error.set(Some(e)),
		}
	}
}
