use {
	super::{
		enums::feed::Feed,
		trades_dtos::{TradesDTO, TradesRequestDTO},
	},
	crate::{
		alpaca_env::AlpacaUrls,
		routes::{EAlpacaRoute, EDataRoute},
	},
	chrono::{DateTime, Utc},
	reqwest::Client,
};

#[bon::builder]
pub async fn trades_request(
	urls: AlpacaUrls,
	symbol: String,
	start: Option<DateTime<Utc>>,
	end: Option<DateTime<Utc>>,
	limit: Option<usize>,
	feed: Option<Feed>,
	page_token: Option<String>,
	#[builder(default = Client::new())] client: reqwest::Client,
) -> Result<TradesDTO, reqwest::Error> {
	client
		.get(EAlpacaRoute::Data(EDataRoute::Trades(symbol)).url_path(urls))
		.query(&TradesRequestDTO { start, end, limit, feed, page_token })
		.send()
		.await?
		.json()
		.await
}
