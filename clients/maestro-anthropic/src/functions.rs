use {
	crate::{chat_message::ChatMessage, Prompt},
	dioxus::prelude::*,
	futures::StreamExt,
	server_fn::codec::{StreamingText, TextStream},
};

#[server(output = StreamingText)]
pub async fn stream_chat_from_server(messages: Vec<ChatMessage>, temperature: f32, system_prompt: String) -> Result<TextStream, ServerFnError> {
	use {crate::stream::FilterExt, futures::pin_mut};
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
