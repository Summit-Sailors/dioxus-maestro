use {crate::AnthropicClient, dioxus::prelude::*};

pub async fn extract_anthropic_client() -> Result<AnthropicClient, ServerFnError> {
	let FromContext(client): FromContext<AnthropicClient> = extract().await?;
	Ok(client)
}
