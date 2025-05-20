use {
	crate::{
		alpaca_env::AlpacaUrls,
		routes::{EAlpacaRoute, EApiRoute},
	},
	reqwest::Client,
	serde::{Deserialize, Serialize},
};

#[bon::builder]
pub async fn account_configurations_get_request(urls: AlpacaUrls, #[builder(default = Client::new())] client: Client) -> Result<Configuration, reqwest::Error> {
	client.get(EAlpacaRoute::Api(EApiRoute::AccountActivities).url_path(urls)).send().await?.json().await
}

#[derive(Clone, Copy, Debug, Deserialize, Eq, PartialEq, Serialize)]
enum TradeConfirmation {
	#[serde(rename = "all")]
	Email,
	#[serde(rename = "none")]
	None,
}

#[derive(Clone, Copy, Debug, Deserialize, Eq, PartialEq, Serialize)]
struct Configuration {
	#[serde(rename = "trade_confirm_email")]
	pub trade_confirmation: TradeConfirmation,
	#[serde(rename = "suspend_trade")]
	pub trading_suspended: bool,
	pub no_shorting: bool,
}
