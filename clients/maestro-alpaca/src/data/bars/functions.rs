use dioxus::prelude::*;

use super::bars_dtos::{BarsDTO, BarsSingleRequestDTO};

#[server]
pub async fn get_alpaca_bars_from_server(symbol: String, search_params: BarsSingleRequestDTO) -> Result<BarsDTO, ServerFnError> {
	Ok(super::bars_reqwest::bars_request_single(symbol, search_params, crate::server_ctx::alpaca_client_from_ctx().await?).await?)
}
