use {
	super::order_status::OrderStatus,
	crate::api::order::OrderDTO,
	serde::{Deserialize, Serialize},
	std::borrow::Cow,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum StreamType {
	#[serde(rename = "trade_updates")]
	OrderUpdates,
}

#[derive(Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum AuthenticationStatus {
	#[serde(rename = "authorized")]
	Authorized,
	#[serde(rename = "unauthorized")]
	Unauthorized,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Authentication {
	pub status: AuthenticationStatus,
}

#[derive(Debug, Serialize, Deserialize)]
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

#[derive(Debug, Clone, PartialEq)]
pub struct OrderUpdate {
	pub event: OrderStatus,
	pub order: OrderDTO,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum OrderUpdates {}
