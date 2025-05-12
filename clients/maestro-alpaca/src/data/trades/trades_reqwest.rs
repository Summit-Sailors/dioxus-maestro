use chrono::{DateTime, Utc};

use super::trades_dtos::{
	TradesLatestRequestDTO, TradesLatestResponseDTO, TradesMultiRequestDTO, TradesMultiResponseDTO, TradesSingleRequestDTO, TradesSingleResponseDTO,
};
use crate::data::enums::feed::Feed;

// Single symbol historical trades request
#[bon::builder]
pub async fn trades_request_single_builder(
	client: reqwest::Client,
	symbol: String,
	limit: Option<usize>,
	start: Option<DateTime<Utc>>,
	end: Option<DateTime<Utc>>,
	asof: Option<DateTime<Utc>>,
	currency: Option<String>,
	sort: Option<String>,
	page_token: Option<String>,
	#[builder(default)] feed: Feed,
) -> Result<TradesSingleResponseDTO, reqwest::Error> {
	trades_request_single(
		symbol,
		TradesSingleRequestDTO::builder()
			.maybe_limit(limit)
			.maybe_start(start.map(|start| start.to_rfc3339()))
			.maybe_end(end.map(|end| end.to_rfc3339()))
			.maybe_asof(asof.map(|date| date.to_rfc3339()))
			.maybe_currency(currency)
			.maybe_sort(sort)
			.maybe_page_token(page_token)
			.feed(feed)
			.build(),
		client,
	)
	.await
}

pub async fn trades_request_single(
	symbol: String,
	request: TradesSingleRequestDTO,
	client: reqwest::Client,
) -> Result<TradesSingleResponseDTO, reqwest::Error> {
	client.get(format!("https://data.alpaca.markets/v2/stocks/{symbol}/trades")).query(&request).send().await?.json().await
}

// Multi-symbol historical trades request
#[bon::builder]
pub async fn trades_request_multi_builder(
	client: reqwest::Client,
	symbols: Vec<String>,
	limit: Option<usize>,
	start: Option<DateTime<Utc>>,
	end: Option<DateTime<Utc>>,
	asof: Option<DateTime<Utc>>,
	currency: Option<String>,
	sort: Option<String>,
	page_token: Option<String>,
	#[builder(default)] feed: Feed,
) -> Result<TradesMultiResponseDTO, reqwest::Error> {
	trades_request_multi(
		TradesMultiRequestDTO::builder()
			.symbols(symbols)
			.maybe_limit(limit)
			.maybe_start(start.map(|start| start.to_rfc3339()))
			.maybe_end(end.map(|end| end.to_rfc3339()))
			.maybe_asof(asof.map(|date| date.to_rfc3339()))
			.maybe_currency(currency)
			.maybe_sort(sort)
			.maybe_page_token(page_token)
			.feed(feed)
			.build(),
		client,
	)
	.await
}

pub async fn trades_request_multi(request: TradesMultiRequestDTO, client: reqwest::Client) -> Result<TradesMultiResponseDTO, reqwest::Error> {
	client.get("https://data.alpaca.markets/v2/stocks/trades").query(&request).send().await?.json().await
}

// Latest trades request
#[bon::builder]
pub async fn trades_latest_builder(
	client: reqwest::Client,
	symbols: Vec<String>,
	currency: Option<String>,
	#[builder(default)] feed: Feed,
) -> Result<TradesLatestResponseDTO, reqwest::Error> {
	trades_latest(TradesLatestRequestDTO::builder().symbols(symbols).maybe_currency(currency).feed(feed).build(), client).await
}

pub async fn trades_latest(request: TradesLatestRequestDTO, client: reqwest::Client) -> Result<TradesLatestResponseDTO, reqwest::Error> {
	client.get("https://data.alpaca.markets/v2/stocks/trades/latest").query(&request).send().await?.json().await
}
