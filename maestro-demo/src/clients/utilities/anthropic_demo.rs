use std::fmt::Debug;

use dioxus::prelude::*;
use maestro_anthropic::chat_message::{ChatMessage, Role};
use tailwind_fuse::tw_join;

use crate::components::ui::features::Features;

#[component]
pub fn AnthropicDemo() -> Element {
	let mut messages = use_signal(std::vec::Vec::new);
	let mut user_input = use_signal(String::new);
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
					error.set(Some(format!("Error: {e:?}")));
					messages.write().pop();
					is_loading.set(false);
				},
			}
		});
	};

	rsx! {
		div { class: "container mx-auto p-4 flex flex-col max-h-screen",
			div { class: "flex flex-col gap-3",
				h1 { class: "text-[color:var(--text-color)] text-center text-3xl font-bold mb-2",
					"Maestro Anthropic"
				}
				p { class: "text-[color:var(--muted-text)] text-center",
					"A utility designed to make connecting to and using Anthropic with your Dioxus apps easier"
				}
			}

			div {
				id: "maestro-anthropic-features",
				class: "flex space-x-2 mt-4 mb-4",
				Features {
					title: "Features".to_string(),
					features: vec![
							"Easy to create and retrieve Anthropic cleint from server context".to_string(),
							"Ready to use Dioxus server functions".to_string(),
							"Possible to stream response from server".to_string(),
							"Comprehensive DTOs with proper typing".to_string(),
							"Simple integration with Dioxus".to_string(),
					],
				}
			}

			// settings panel
			div { class: "bg-[color:var(--card-bg)] p-4 border-b border-[color:var(--border)]",
				div { class: "flex flex-col md:flex-row gap-4",

					// system prompt input
					div { class: "flex-1",
						label { class: "block text-sm font-medium text-[color:var(--muted-foreground)]",
							"System Prompt"
						}
						input {
							class: "mt-1 block w-full rounded-md border border-[color:var(--border)] bg-[color:var(--card-bg)] text-[color:var(--text-color)] placeholder-[color:var(--muted-foreground)] focus:border-[color:var(--ring)] focus:ring-[color:var(--ring)]",
							placeholder: "Enter system prompt...",
							value: "{system_prompt}",
							oninput: move |e| system_prompt.set(e.value().clone()),
						}
					}

					// temperature slider
					div { class: "w-64",
						label { class: "block text-sm font-medium text-[color:var(--muted-foreground)]",
							"Temperature: {temperature}"
						}
						input {
							class: "mt-1 block w-full accent-[color:var(--primary)] border border-[color:var(--border)] bg-[color:var(--card-bg)] text-[color:var(--text-color)] focus:border-[color:var(--ring)] focus:ring-[color:var(--ring)]",
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


			div { class: "flex-1 overflow-y-auto p-4 space-y-4 bg-[color:var(--card-bg)] max-h-screen rounded-2xl",
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
															if is_user {
															"bg-[color:var(--primary)] text-[color:var(--primary-foreground)] p-3 rounded-lg max-w-3/4"
															} else {
															"bg-[color:var(--secondary)] text-[color:var(--secondary-foreground)] p-3 rounded-lg max-w-3/4"
															}
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
										div { class: "bg-[color:var(--destructive)] text-[color:var(--text-color)] p-3 my-2 rounded-lg",
											p { "{err_msg}" }
											button {
												class: "ml-2 px-2 py-1 bg-[color:var(--destructive)] rounded hover:bg-red-800",
												onclick: move |_| error.set(None),
												"Dismiss"
											}
										}
									}
							})
			}

			// input area
			div { class: "p-4 bg-[color:var(--card-bg)] border-t border-[color:var(--border)]",
				div { class: "flex space-x-2",
					input {
						class: "flex-1 rounded-md border border-[color:var(--border)] bg-[color:var(--card-bg)] text-[color:var(--text-color)] placeholder-[color:var(--muted-foreground)] shadow-sm focus:border-[color:var(--ring)] focus:ring-[color:var(--ring)]",
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
						class: "bg-[color:var(--primary)] border border-[color:var(--ring)] text-[color:var(--primary-foreground)] px-4 py-2 rounded-md hover:bg-blue-700 disabled:opacity-50 disabled:cursor-not-allowed",
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
	use futures::StreamExt;
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
