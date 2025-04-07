use {
	dioxus::prelude::*,
	futures::StreamExt,
	maestro_anthropic::chat_message::{ChatMessage, Role},
	std::fmt::Debug,
	tailwind_fuse::tw_join,
};

#[component]
pub fn AnthropicDemo() -> Element {
	let mut messages = use_signal(|| vec![]);
	let mut user_input = use_signal(|| String::new());
	let mut is_loading = use_signal(|| false);
	let mut temperature = use_signal(|| 0.7f32);
	let mut system_prompt = use_signal(|| String::from("You are Claude, a helpful AI assistant."));

	let mut error = use_signal(|| None::<String>);

	let mut send_message = move || {
		let input = user_input();
		if input.trim().is_empty() {
			return;
		}

		if !matches!(system_prompt().as_str(), "You are Claude, a helpful AI assistant.") {
			messages.write().clear();
		}

		messages.write().push(ChatMessage { role: Role::User, content: input });

		messages.write().push(ChatMessage { role: Role::Assistant, content: String::new() });

		user_input.set(String::new());
		is_loading.set(true);

		spawn(async move {
			match stream_chat(messages().iter().take(messages().len() - 1).cloned().collect(), temperature(), system_prompt()).await {
				Ok(response) => {
					let chunks: Vec<_> = response.chars().collect();
					let chunk_size = 10;
					let mut accumulated = String::new();

					for i in (0..chunks.len()).step_by(chunk_size) {
						let end = (i + chunk_size).min(chunks.len());
						let chunk: String = chunks[i..end].iter().collect();
						accumulated.push_str(&chunk);

						let msg_len = messages().len();
						messages.write()[msg_len - 1].content = accumulated.clone();

						async_std::task::sleep(std::time::Duration::from_millis(20)).await;
					}

					is_loading.set(false);
				},
				Err(e) => {
					error.set(Some(format!("Error: {:?}", e)));
					messages.write().pop();
					is_loading.set(false);
				},
			}
		});
	};

	rsx! {
		div { class: "container mx-auto p-4 flex flex-col max-h-screen",
			header { class: "bg-gray-800 text-white p-4 rounded-t-lg border-b border-gray-700",
				h1 { class: "text-2xl font-bold", "Maestro Anthropic Demo" }
			}

			// settings panel
			div { class: "bg-gray-900 p-4 border-b border-gray-700",
				div { class: "flex flex-col md:flex-row gap-4",

					// system prompt input
					div { class: "flex-1",
						label { class: "block text-sm font-medium text-gray-300", "System Prompt" }
						input {
							class: "mt-1 block w-full rounded-md border border-gray-600 bg-gray-800 text-white placeholder-gray-500 focus:border-blue-500 focus:ring-blue-500",
							placeholder: "Enter system prompt...",
							value: "{system_prompt}",
							oninput: move |e| system_prompt.set(e.value().clone()),
						}
					}

					// temperature slider
					div { class: "w-64",
						label { class: "block text-sm font-medium text-gray-300",
							"Temperature: {temperature}"
						}
						input {
							class: "mt-1 block w-full accent-blue-500 border border-gray-600 bg-gray-800 text-white placeholder-gray-500 focus:border-blue-500 focus:ring-blue-500",
							r#type: "range",
							min: "0",
							max: "1",
							step: "0.1",
							value: "{temperature}",
							oninput: move |e| temperature.set(e.value().parse::<f32>().unwrap_or(0.7)),
						}
					}
				}
			}

			div { class: "flex-1 overflow-y-auto p-4 space-y-4 bg-gray-800 max-h-screen rounded-2xl",
				{
						messages
								.iter()
								.enumerate()
								.map(|(index, message)| {
										let is_user = matches!(message.role, Role::User);
										rsx! {
											div {
												key: "{index}",
												class: tw_join!(if is_user { "flex justify-end" } else { "flex justify-start" }),
												div {
													class: tw_join!(
															if is_user { "bg-blue-600 text-white p-3 rounded-lg max-w-3/4" } else {
															"bg-gray-700 text-gray-100 p-3 rounded-lg max-w-3/4" }
													),
													p { style: "white-space: pre-wrap;", "{message.content}" }
													{
															if is_loading() && !is_user && index == messages().len() - 1 {
																	rsx! {
																		div { class: "animate-pulse mt-2", "..." }
																	}
															} else {
																	rsx! {}
															}
													}
												}
											}
										}
								})
				}
			}

			{
					error()
							.map(|err_msg| {
									rsx! {
										div { class: "bg-red-600 text-white p-3 my-2 rounded-lg",
											p { "{err_msg}" }
											button {
												class: "ml-2 px-2 py-1 bg-red-700 rounded hover:bg-red-800",
												onclick: move |_| error.set(None),
												"Dismiss"
											}
										}
									}
							})
			}

			// input area
			div { class: "p-4 bg-gray-900 border-t border-gray-700",
				div { class: "flex space-x-2",
					input {
						class: "flex-1 rounded-md border border-gray-600 bg-gray-800 text-white placeholder-gray-500 shadow-sm focus:border-blue-500 focus:ring-blue-500",
						placeholder: "Type a message...",
						value: "{user_input}",
						oninput: move |e| user_input.set(e.value().clone()),
						onkeydown: move |e| {
								let key = e.key();
								if key.to_string() == "Enter" && key.to_string() != "Shift" {
										e.prevent_default();
										send_message();
								}
						},
					}
					button {
						class: "bg-blue-600 border border-blue-500 text-white px-4 py-2 rounded-md hover:bg-blue-700 disabled:opacity-50 disabled:cursor-not-allowed",
						disabled: "{is_loading() || user_input().trim().is_empty()}",
						onclick: move |_| send_message(),
						"Send"
					}
				}
			}
		}
	}
}

#[server]
async fn stream_chat(messages: Vec<ChatMessage>, temperature: f32, system_prompt: String) -> Result<String, ServerFnError> {
	let stream = maestro_anthropic::functions::stream_chat_from_server(messages, temperature, system_prompt).await?;

	let mut full_text = String::new();
	let mut stream = stream.into_inner();

	while let Some(chunk) = stream.next().await {
		if let Ok(text) = chunk {
			full_text.push_str(&text);
		}
	}

	Ok(full_text)
}
