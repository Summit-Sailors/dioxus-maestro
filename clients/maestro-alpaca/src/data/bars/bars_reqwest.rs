use chrono::{DateTime, Utc};

use super::bars_dtos::{BarsDTO, BarsMultiApiDTO, BarsMultiRequestDTO, BarsSingleRequestDTO};
use crate::data::enums::{adjustment::Adjustment, feed::Feed, timeframe::TimeFrame};

#[bon::builder]
pub async fn bars_request_single_builder(
	client: reqwest::Client,
	symbol: String,
	limit: Option<usize>,
	start: Option<DateTime<Utc>>,
	end: Option<DateTime<Utc>>,
	adjustment: Option<Adjustment>,
	page_token: Option<String>,
	#[builder(default)] timeframe: TimeFrame,
	#[builder(default)] feed: Feed,
) -> Result<BarsDTO, reqwest::Error> {
	bars_request_single(
		symbol,
		BarsSingleRequestDTO::builder()
			.maybe_limit(limit)
			.maybe_start(start.map(|start| start.to_rfc3339()))
			.maybe_end(end.map(|start| start.to_rfc3339()))
			.maybe_adjustment(adjustment)
			.maybe_page_token(page_token)
			.timeframe(timeframe)
			.feed(feed)
			.build(),
		client,
	)
	.await
}

pub async fn bars_request_single(symbol: String, request: BarsSingleRequestDTO, client: reqwest::Client) -> Result<BarsDTO, reqwest::Error> {
	client.get(format!("https://data.alpaca.markets/v2/stocks/{symbol}/bars")).query(&request).send().await?.json::<BarsDTO>().await
}

#[bon::builder]
pub async fn bars_request_multi_builder(
	client: reqwest::Client,
	symbols: Vec<String>,
	limit: Option<usize>,
	start: Option<DateTime<Utc>>,
	end: Option<DateTime<Utc>>,
	adjustment: Option<Adjustment>,
	page_token: Option<String>,
	#[builder(default)] timeframe: TimeFrame,
	#[builder(default)] feed: Feed,
) -> Result<BarsMultiApiDTO, reqwest::Error> {
	bars_request_multi(
		BarsMultiRequestDTO::builder()
			.symbols(symbols)
			.params(
				BarsSingleRequestDTO::builder()
					.maybe_limit(limit)
					.maybe_start(start.map(|start| start.to_rfc3339()))
					.maybe_end(end.map(|start| start.to_rfc3339()))
					.maybe_adjustment(adjustment)
					.maybe_page_token(page_token)
					.timeframe(timeframe)
					.feed(feed)
					.build(),
			)
			.build(),
		client,
	)
	.await
}

pub async fn bars_request_multi(request: BarsMultiRequestDTO, client: reqwest::Client) -> Result<BarsMultiApiDTO, reqwest::Error> {
	client.get("https://data.alpaca.markets/v2/stocks/bars".to_string()).query(&request).send().await?.json::<BarsMultiApiDTO>().await
}
