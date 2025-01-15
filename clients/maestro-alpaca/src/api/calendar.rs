use {
	crate::{
		alpaca_env::AlpacaUrls,
		routes::{EAlpacaRoute, EApiRoute},
	},
	chrono::{NaiveDate, NaiveTime},
	reqwest::Client,
	serde::{Deserialize, Serialize},
};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct CalendarGetRequest {
	#[serde(skip_serializing_if = "Option::is_none")]
	pub start: Option<NaiveDate>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub end: Option<NaiveDate>,
}

#[bon::builder]
pub async fn assets_get_request(
	urls: AlpacaUrls,
	start: Option<NaiveDate>,
	end: Option<NaiveDate>,
	#[builder(default = Client::new())] client: Client,
) -> Result<OpenCloseDTO, reqwest::Error> {
	client.get(EAlpacaRoute::Api(EApiRoute::Calendar).url_path(urls)).query(&CalendarGetRequest { start, end }).send().await?.json().await
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct OpenCloseDTO {
	pub date: NaiveDate,
	pub open: NaiveTime,
	pub close: NaiveTime,
}
