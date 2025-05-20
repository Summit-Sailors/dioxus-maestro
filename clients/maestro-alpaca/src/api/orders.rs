use {
	super::order::OrderDTO,
	crate::{
		alpaca_env::AlpacaUrls,
		routes::{EAlpacaRoute, EApiRoute},
	},
	reqwest::Client,
	serde::{Deserialize, Serialize},
};

#[derive(Clone, Copy, Debug, Default, Deserialize, Eq, PartialEq, Serialize)]
pub enum Status {
	#[serde(rename = "open")]
	Open,
	#[serde(rename = "closed")]
	Closed,
	#[serde(rename = "all")]
	#[default]
	All,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct OrdersGetRequest {
	pub symbols: Vec<String>,
	pub status: Status,
	pub limit: Option<usize>,
	pub nested: bool,
}

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
struct OrdersDTO {
	orders: Vec<OrderDTO>,
}

#[bon::builder]
pub async fn orders_get_request(
	urls: AlpacaUrls,
	symbols: Vec<String>,
	limit: Option<usize>,
	nested: bool,
	#[builder(default)] status: Status,
	#[builder(default = Client::new())] client: Client,
) -> Result<OrdersDTO, reqwest::Error> {
	client.get(EAlpacaRoute::Api(EApiRoute::Orders).url_path(urls)).query(&OrdersGetRequest { symbols, status, limit, nested }).send().await?.json().await
}
