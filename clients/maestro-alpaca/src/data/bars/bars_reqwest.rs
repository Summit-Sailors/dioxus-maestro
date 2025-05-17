use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize, de::DeserializeOwned};
use serde_json::{Value, json};

use super::bars_dtos::{BarsDTO, BarsLatestSingleDTO, BarsMultiApiDTO, BarsMultiRequestDTO, BarsSingleRequestDTO};
use crate::data::enums::{adjustment::Adjustment, feed::Feed, market_data::AssetClass, timeframe::TimeFrame};

/// Generic wrapper for API responses
#[derive(Debug, Serialize, Deserialize)]
pub struct ResponseWrapper<T> {
	pub bars_data: T,
}

async fn wrap_response<T: DeserializeOwned>(response: reqwest::Response) -> Result<ResponseWrapper<T>, reqwest::Error> {
	// get the response JSON as a generic Value
	let value: Value = response.json().await?;
	// Wrap it in our custom structure
	let wrapper = json!({
			"bars_data": value
	});
	// Deserialize into our wrapper type
	Ok(serde_json::from_value(wrapper).unwrap())
}

/// Request Bars data for a single symbol with wrapped response
#[bon::builder]
pub async fn bars_request_single_builder(
	client: reqwest::Client,
	symbol: String,
	limit: Option<usize>,
	start: Option<DateTime<Utc>>,
	end: Option<DateTime<Utc>>,
	asof: Option<DateTime<Utc>>,
	currency: Option<String>,
	sort: Option<String>,
	adjustment: Option<Adjustment>,
	page_token: Option<String>,
	loc: Option<String>,
	#[builder(default)] timeframe: TimeFrame,
	#[builder(default)] feed: Feed,
	#[builder(default = AssetClass::Stocks)] asset_class: AssetClass,
) -> Result<ResponseWrapper<BarsDTO>, reqwest::Error> {
	bars_request_single(
		symbol,
		BarsSingleRequestDTO::builder()
			.maybe_limit(limit)
			.maybe_start(start.map(|start| start.to_rfc3339()))
			.maybe_end(end.map(|start| start.to_rfc3339()))
			.maybe_asof(asof.map(|date| date.to_rfc3339()))
			.maybe_currency(currency)
			.maybe_sort(sort)
			.maybe_adjustment(adjustment)
			.maybe_page_token(page_token)
			.maybe_loc(loc)
			.timeframe(timeframe)
			.feed(feed)
			.build(),
		client,
		asset_class,
	)
	.await
}

pub async fn bars_request_single(
	symbol: String,
	request: BarsSingleRequestDTO,
	client: reqwest::Client,
	asset_class: AssetClass,
) -> Result<ResponseWrapper<BarsDTO>, reqwest::Error> {
	let url = get_bars_url(&asset_class, "single", Some(&symbol), request.loc.as_deref());
	let response = client.get(url).query(&request).send().await?;
	wrap_response::<BarsDTO>(response).await
}

#[bon::builder]
pub async fn bars_request_multi_builder(
	client: reqwest::Client,
	symbols: Vec<String>,
	limit: Option<usize>,
	start: Option<DateTime<Utc>>,
	end: Option<DateTime<Utc>>,
	asof: Option<DateTime<Utc>>,
	currency: Option<String>,
	sort: Option<String>,
	adjustment: Option<Adjustment>,
	page_token: Option<String>,
	loc: Option<String>,
	#[builder(default)] timeframe: TimeFrame,
	#[builder(default)] feed: Feed,
	#[builder(default = AssetClass::Stocks)] asset_class: AssetClass,
) -> Result<ResponseWrapper<BarsMultiApiDTO>, reqwest::Error> {
	bars_request_multi(
		BarsMultiRequestDTO::builder()
			.symbols(symbols)
			.params(
				BarsSingleRequestDTO::builder()
					.maybe_limit(limit)
					.maybe_start(start.map(|start| start.to_rfc3339()))
					.maybe_end(end.map(|start| start.to_rfc3339()))
					.maybe_asof(asof.map(|date| date.to_rfc3339()))
					.maybe_currency(currency)
					.maybe_sort(sort)
					.maybe_adjustment(adjustment)
					.maybe_page_token(page_token)
					.maybe_loc(loc)
					.timeframe(timeframe)
					.feed(feed)
					.build(),
			)
			.build(),
		client,
		asset_class,
	)
	.await
}

pub async fn bars_request_multi(
	request: BarsMultiRequestDTO,
	client: reqwest::Client,
	asset_class: AssetClass,
) -> Result<ResponseWrapper<BarsMultiApiDTO>, reqwest::Error> {
	let url = get_bars_url(&asset_class, "multi", None, request.params.loc.as_deref());
	let response = client.get(url).query(&request).send().await?;
	wrap_response::<BarsMultiApiDTO>(response).await
}

#[bon::builder]
pub async fn bars_request_latest_builder(
	client: reqwest::Client,
	symbols: Vec<String>,
	currency: Option<String>,
	loc: Option<String>,
	#[builder(default)] feed: Feed,
	#[builder(default = AssetClass::Stocks)] asset_class: AssetClass,
) -> Result<ResponseWrapper<BarsMultiApiDTO>, reqwest::Error> {
	bars_request_latest(
		BarsMultiRequestDTO::builder().symbols(symbols).params(BarsSingleRequestDTO::builder().maybe_currency(currency).maybe_loc(loc).feed(feed).build()).build(),
		client,
		asset_class,
	)
	.await
}

pub async fn bars_request_latest(
	request: BarsMultiRequestDTO,
	client: reqwest::Client,
	asset_class: AssetClass,
) -> Result<ResponseWrapper<BarsMultiApiDTO>, reqwest::Error> {
	let url = get_bars_url(&asset_class, "latest", None, request.params.loc.as_deref());
	let response = client.get(url).query(&request).send().await?;
	wrap_response::<BarsMultiApiDTO>(response).await
}

#[bon::builder]
pub async fn bars_request_latest_single_builder(
	client: reqwest::Client,
	symbol: String,
	currency: Option<String>,
	#[builder(default = Feed::Sip)] feed: Feed,
) -> Result<ResponseWrapper<BarsLatestSingleDTO>, reqwest::Error> {
	bars_request_latest_single(symbol, BarsSingleRequestDTO::builder().maybe_currency(currency).feed(feed).build(), client).await
}

pub async fn bars_request_latest_single(
	symbol: String,
	request: BarsSingleRequestDTO,
	client: reqwest::Client,
) -> Result<ResponseWrapper<BarsLatestSingleDTO>, reqwest::Error> {
	let response = client.get(format!("https://data.alpaca.markets/v2/stocks/{symbol}/bars/latest")).query(&request).send().await?;
	wrap_response::<BarsLatestSingleDTO>(response).await
}

fn get_bars_url(asset_class: &AssetClass, endpoint_type: &str, symbol: Option<&str>, loc: Option<&str>) -> String {
	match asset_class {
		AssetClass::Stocks => match endpoint_type {
			"single" => format!("https://data.alpaca.markets/v2/stocks/{}/bars", symbol.unwrap_or("")),
			"multi" => "https://data.alpaca.markets/v2/stocks/bars".to_string(),
			"latest" =>
				if let Some(sym) = symbol {
					format!("https://data.alpaca.markets/v2/stocks/{sym}/bars/latest")
				} else {
					"https://data.alpaca.markets/v2/stocks/bars/latest".to_string()
				},
			_ => panic!("Unknown endpoint type: {endpoint_type}"),
		},
		AssetClass::Options => match endpoint_type {
			"multi" => "https://data.alpaca.markets/v1beta1/options/bars".to_string(),
			_ => panic!("Unsupported endpoint type for options: {endpoint_type}"),
		},
		AssetClass::Crypto => {
			let location = loc.unwrap_or("us");
			match endpoint_type {
				"multi" => format!("https://data.alpaca.markets/v1beta3/crypto/{location}/bars"),
				"latest" => format!("https://data.alpaca.markets/v1beta3/crypto/{location}/latest/bars"),
				_ => panic!("Unsupported endpoint type for crypto: {endpoint_type}"),
			}
		},
		_ => panic!("There is no Bars data for the provided AssetClass. The only supported Asset classes are Stocks/ETFs, Options and Crypto"),
	}
}
