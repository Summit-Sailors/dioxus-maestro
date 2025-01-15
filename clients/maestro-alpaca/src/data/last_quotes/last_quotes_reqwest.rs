use {
	super::last_quotes_dtos::{LatestQuotesRequestDTO, LatestQuotesResponseDTO, QuoteResponseDTO},
	crate::data::enums::feed::Feed,
};

#[bon::builder]
pub async fn latest_quotes_request_builder(
	client: reqwest::Client,
	symbols: Vec<String>,
	#[builder(default)] feed: Feed,
	#[builder(default)] currency: String,
) -> Result<Vec<QuoteResponseDTO>, reqwest::Error> {
	latest_quotes_request(LatestQuotesRequestDTO::builder().symbols(symbols).feed(feed).currency(currency).build(), client).await
}

pub async fn latest_quotes_request(request: LatestQuotesRequestDTO, client: reqwest::Client) -> Result<Vec<QuoteResponseDTO>, reqwest::Error> {
	let response = client.get("https://data.alpaca.markets/v2/stocks/quotes/latest").query(&request).send().await?.json::<LatestQuotesResponseDTO>().await?;

	Ok(response.into())
}
