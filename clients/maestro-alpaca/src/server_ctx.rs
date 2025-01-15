use {crate::get_client::AlpacaClient, dioxus::prelude::*, reqwest::Client, std::ops::Deref};

pub async fn alpaca_client_from_ctx() -> Result<Client, ServerFnError> {
	let FromContext(alpaca_client): FromContext<AlpacaClient> = extract().await?;
	Ok(alpaca_client.deref().clone())
}
