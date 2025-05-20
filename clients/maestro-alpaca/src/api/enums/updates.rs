use {
	super::order_status::OrderStatus,
	crate::api::order::OrderDTO,
	serde::{Deserialize, Serialize},
	std::borrow::Cow,
};

#[derive(Clone, Copy, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub enum StreamType {
	#[serde(rename = "trade_updates")]
	OrderUpdates,
}

#[derive(Debug, Deserialize, Eq, PartialEq, Serialize)]
pub enum AuthenticationStatus {
	#[serde(rename = "authorized")]
	Authorized,
	#[serde(rename = "unauthorized")]
	Unauthorized,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Authentication {
	pub status: AuthenticationStatus,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(tag = "action")]
pub enum Authenticate<'d> {
	#[serde(rename = "auth")]
	Request {
		#[serde(rename = "key")]
		key_id: Cow<'d, str>,
		#[serde(rename = "secret")]
		secret: Cow<'d, str>,
	},
}

#[derive(Clone, Debug, PartialEq)]
pub struct OrderUpdate {
	pub event: OrderStatus,
	pub order: OrderDTO,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum OrderUpdates {}
