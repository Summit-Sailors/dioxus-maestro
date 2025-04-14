use dioxus::prelude::*;

use super::last_quotes_dtos::{LatestQuotesRequestDTO, QuoteResponseDTO};

#[server]
pub async fn get_alpaca_latest_quotes_from_server(search_params: LatestQuotesRequestDTO) -> Result<Vec<QuoteResponseDTO>, ServerFnError> {
	Ok(super::last_quotes_reqwest::latest_quotes_request(search_params, crate::server_ctx::alpaca_client_from_ctx().await?).await?)
}
