use {
	crate::chat_message::ChatMessage,
	dioxus::prelude::*,
	futures::StreamExt,
	server_fn::codec::{StreamingText, TextStream},
};
#[cfg(feature = "server")]
use {
	futures::pin_mut,
	misanthropic::{prompt::Message, Prompt},
	misanthropic::{stream::FilterExt, Client, Model},
	std::num::NonZero,
};

#[server(name=StreamChat,output = StreamingText)]
pub async fn stream_chat(messages: Vec<ChatMessage>, temperature: f32, system_prompt: String) -> Result<TextStream, ServerFnError> {
	let FromContext(client): FromContext<Client> = extract().await?;
	let (tx, rx) = futures::channel::mpsc::unbounded();
	tokio::spawn(async move {
		let stream = client
			.stream(
				Prompt::default()
					.model(Model::Sonnet35)
					.max_tokens(NonZero::new(8192).unwrap())
					.temperature(Some(temperature))
					.system(system_prompt)
					.messages(messages.into_iter().map(Message::from)),
			)
			.await
			.unwrap()
			.filter_rate_limit()
			.text();
		pin_mut!(stream);
		while let Some(Ok(text)) = stream.next().await {
			let _ = tx.unbounded_send(Ok(text.to_string()));
		}
	});
	Ok(TextStream::new(rx))
}
