use {
	crate::{
		alpaca_env::AlpacaUrls,
		routes::{EAlpacaRoute, EApiRoute},
	},
	chrono::{DateTime, Utc},
	reqwest::Client,
	serde::{Deserialize, Serialize},
};

#[bon::builder]
pub async fn clock_get_request(urls: AlpacaUrls, #[builder(default = Client::new())] client: Client) -> Result<ClockDTO, reqwest::Error> {
	client.get(EAlpacaRoute::Api(EApiRoute::Clock).url_path(urls)).send().await?.json().await
}

#[derive(Clone, Copy, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct ClockDTO {
	#[serde(rename = "is_open")]
	pub open: bool,
	#[serde(rename = "timestamp")]
	pub current: DateTime<Utc>,
	pub next_open: DateTime<Utc>,
	pub next_close: DateTime<Utc>,
}
