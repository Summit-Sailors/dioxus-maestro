use super::snapshots_dtos::{SnapshotSingleRequestDTO, SnapshotSingleResponseDTO, SnapshotsMultiRequestDTO, SnapshotsMultiResponseDTO};
use crate::data::enums::feed::Feed;

#[bon::builder]
pub async fn snapshots_request_single_builder(
	client: reqwest::Client,
	symbol: String,
	#[builder(default)] feed: Feed,
	currency: Option<String>,
) -> Result<SnapshotSingleResponseDTO, reqwest::Error> {
	snapshots_request_single(symbol, SnapshotSingleRequestDTO::builder().feed(feed).maybe_currency(currency).build(), client).await
}

pub async fn snapshots_request_single(
	symbol: String,
	request: SnapshotSingleRequestDTO,
	client: reqwest::Client,
) -> Result<SnapshotSingleResponseDTO, reqwest::Error> {
	client.get(format!("https://data.alpaca.markets/v2/stocks/{symbol}/snapshot")).query(&request).send().await?.json::<SnapshotSingleResponseDTO>().await
}

#[bon::builder]
pub async fn snapshots_request_multi_builder(
	client: reqwest::Client,
	symbols: Vec<String>,
	#[builder(default)] feed: Feed,
	currency: Option<String>,
) -> Result<SnapshotsMultiResponseDTO, reqwest::Error> {
	snapshots_request_multi(SnapshotsMultiRequestDTO::builder().symbols(symbols).feed(feed).maybe_currency(currency).build(), client).await
}

pub async fn snapshots_request_multi(request: SnapshotsMultiRequestDTO, client: reqwest::Client) -> Result<SnapshotsMultiResponseDTO, reqwest::Error> {
	client.get("https://data.alpaca.markets/v2/stocks/snapshots").query(&request).send().await?.json::<SnapshotsMultiResponseDTO>().await
}
