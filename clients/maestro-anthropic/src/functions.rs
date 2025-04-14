use dioxus::prelude::*;
use futures::StreamExt;
use server_fn::codec::{StreamingText, TextStream};

use crate::{Prompt, chat_message::ChatMessage};

#[server(output = StreamingText)]
pub async fn stream_chat_from_server(messages: Vec<ChatMessage>, temperature: f32, system_prompt: String) -> Result<TextStream, ServerFnError> {
	use futures::pin_mut;

	use crate::stream::FilterExt;
	let client = crate::extract_anthropic_client().await?;
	let (tx, rx) = futures::channel::mpsc::unbounded();
	tokio::spawn(async move {
		let stream = client.stream(Prompt::builder().temperature(temperature).system(system_prompt).messages(messages)).await.unwrap().filter_rate_limit().text();
		pin_mut!(stream);
		while let Some(Ok(text)) = stream.next().await {
			let _ = tx.unbounded_send(Ok(text.to_string()));
		}
	});
	Ok(TextStream::new(rx))
}
