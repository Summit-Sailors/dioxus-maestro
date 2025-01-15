use {
	super::{
		enums::feed::Feed,
		quotes_dtos::{QuotesDTO, QuotesRequestDTO},
	},
	crate::{
		alpaca_env::AlpacaUrls,
		routes::{EAlpacaRoute, EDataRoute},
	},
	chrono::{DateTime, Utc},
	reqwest::Client,
};

#[bon::builder]
pub async fn quotes_request(
	urls: AlpacaUrls,
	symbol: String,
	start: DateTime<Utc>,
	end: DateTime<Utc>,
	limit: Option<usize>,
	feed: Option<Feed>,
	page_token: Option<String>,
	#[builder(default = Client::new())] client: reqwest::Client,
) -> Result<QuotesDTO, reqwest::Error> {
	client
		.get(EAlpacaRoute::Data(EDataRoute::Quotes(symbol)).url_path(urls))
		.query(&QuotesRequestDTO { start, end, limit, feed, page_token })
		.send()
		.await?
		.json()
		.await
}
