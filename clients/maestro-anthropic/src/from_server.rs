use {crate::AnthropicClient, dioxus::prelude::*};

pub async fn extract_anthropic_client() -> Result<AnthropicClient, ServerFnError> {
	let FromContext(pool): FromContext<AnthropicClient> = extract().await?;
	Ok(pool)
}
