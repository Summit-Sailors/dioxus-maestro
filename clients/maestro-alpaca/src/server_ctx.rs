use std::ops::Deref;

use dioxus::prelude::*;
use reqwest::Client;

use crate::get_client::AlpacaClient;

pub async fn alpaca_client_from_ctx() -> Result<Client, ServerFnError> {
	let FromContext(alpaca_client): FromContext<AlpacaClient> = extract().await?;
	Ok(alpaca_client.deref().clone())
}
