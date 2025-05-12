use chrono::{DateTime, Utc};

use super::auctions_dtos::{AuctionsMultiRequestDTO, AuctionsMultiResponseDTO, AuctionsSingleRequestDTO, AuctionsSingleResponseDTO};
use crate::data::enums::feed::Feed;

#[bon::builder]
pub async fn auctions_request_single_builder(
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
) -> Result<AuctionsSingleResponseDTO, reqwest::Error> {
	auctions_request_single(
		symbol,
		AuctionsSingleRequestDTO::builder()
			.maybe_limit(limit)
			.maybe_start(start.map(|start| start.to_rfc3339()))
			.maybe_end(end.map(|start| start.to_rfc3339()))
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

pub async fn auctions_request_single(
	symbol: String,
	request: AuctionsSingleRequestDTO,
	client: reqwest::Client,
) -> Result<AuctionsSingleResponseDTO, reqwest::Error> {
	client.get(format!("https://data.alpaca.markets/v2/stocks/{symbol}/auctions")).query(&request).send().await?.json().await
}

#[bon::builder]
pub async fn auctions_request_multi_builder(
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
) -> Result<AuctionsMultiResponseDTO, reqwest::Error> {
	auctions_request_multi(
		AuctionsMultiRequestDTO::builder()
			.symbols(symbols)
			.maybe_limit(limit)
			.maybe_start(start.map(|start| start.to_rfc3339()))
			.maybe_end(end.map(|start| start.to_rfc3339()))
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

pub async fn auctions_request_multi(request: AuctionsMultiRequestDTO, client: reqwest::Client) -> Result<AuctionsMultiResponseDTO, reqwest::Error> {
	client.get("https://data.alpaca.markets/v2/stocks/auctions".to_string()).query(&request).send().await?.json().await
}
