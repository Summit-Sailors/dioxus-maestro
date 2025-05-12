use chrono::{DateTime, Utc};

use super::quotes_dtos::{
	QuotesLatestRequestDTO, QuotesLatestResponseDTO, QuotesMultiRequestDTO, QuotesMultiResponseDTO, QuotesSingleRequestDTO, QuotesSingleResponseDTO,
};
use crate::data::enums::feed::Feed;

// Single symbol historical quotes request
#[bon::builder]
pub async fn quotes_request_single_builder(
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
) -> Result<QuotesSingleResponseDTO, reqwest::Error> {
	quotes_request_single(
		symbol,
		QuotesSingleRequestDTO::builder()
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

pub async fn quotes_request_single(
	symbol: String,
	request: QuotesSingleRequestDTO,
	client: reqwest::Client,
) -> Result<QuotesSingleResponseDTO, reqwest::Error> {
	client.get(format!("https://data.alpaca.markets/v2/stocks/{symbol}/quotes")).query(&request).send().await?.json().await
}

// Multi-symbol historical quotes request
#[bon::builder]
pub async fn quotes_request_multi_builder(
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
) -> Result<QuotesMultiResponseDTO, reqwest::Error> {
	quotes_request_multi(
		QuotesMultiRequestDTO::builder()
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

pub async fn quotes_request_multi(request: QuotesMultiRequestDTO, client: reqwest::Client) -> Result<QuotesMultiResponseDTO, reqwest::Error> {
	client.get("https://data.alpaca.markets/v2/stocks/quotes").query(&request).send().await?.json().await
}

// Latest quotes request
#[bon::builder]
pub async fn quotes_latest_builder(
	client: reqwest::Client,
	symbols: Vec<String>,
	currency: Option<String>,
	#[builder(default)] feed: Feed,
) -> Result<QuotesLatestResponseDTO, reqwest::Error> {
	quotes_latest(QuotesLatestRequestDTO::builder().symbols(symbols).maybe_currency(currency).feed(feed).build(), client).await
}

pub async fn quotes_latest(request: QuotesLatestRequestDTO, client: reqwest::Client) -> Result<QuotesLatestResponseDTO, reqwest::Error> {
	client.get("https://data.alpaca.markets/v2/stocks/quotes/latest").query(&request).send().await?.json().await
}
