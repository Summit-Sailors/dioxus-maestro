use {
	super::{
		assets::Symbol,
		enums::{amount::Amount, order_type::Type, side::SideBS, stop_loss::StopLoss, take_profit::TakeProfit, time_in_force::TimeInForce},
	},
	bigdecimal::BigDecimal,
	chrono::{DateTime, Utc},
	serde::{Deserialize, Serialize},
	uuid::Uuid,
};

#[derive(Clone, Copy, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub enum Status {
	#[serde(rename = "new")]
	New,
	#[serde(rename = "replaced")]
	Replaced,
	#[serde(rename = "partially_filled")]
	PartiallyFilled,
	#[serde(rename = "filled")]
	Filled,
	#[serde(rename = "done_for_day")]
	DoneForDay,
	#[serde(rename = "canceled")]
	Canceled,
	#[serde(rename = "expired")]
	Expired,
	#[serde(rename = "accepted")]
	Accepted,
	#[serde(rename = "pending_new")]
	PendingNew,
	#[serde(rename = "accepted_for_bidding")]
	AcceptedForBidding,
	#[serde(rename = "pending_cancel")]
	PendingCancel,
	#[serde(rename = "pending_replace")]
	PendingReplace,
	#[serde(rename = "stopped")]
	Stopped,
	#[serde(rename = "rejected")]
	Rejected,
	#[serde(rename = "suspended")]
	Suspended,
	#[serde(rename = "calculated")]
	Calculated,
	#[serde(rename = "held")]
	Held,
}

impl Status {
	pub fn is_terminal(self) -> bool {
		matches!(self, Self::Replaced | Self::Filled | Self::Canceled | Self::Expired | Self::Rejected)
	}
}

#[derive(Clone, Copy, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub enum Class {
	#[serde(rename = "simple")]
	Simple,
	#[serde(rename = "bracket")]
	Bracket,
	#[serde(rename = "oco")]
	OneCancelsOther,
	#[serde(rename = "oto")]
	OneTriggersOther,
}

impl Default for Class {
	fn default() -> Self {
		Self::Simple
	}
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct OrderPostRequest {
	pub symbol: Symbol,
	#[serde(flatten)]
	pub amount: Amount,
	pub side: SideBS,
	#[serde(rename = "order_class")]
	pub class: Class,
	#[serde(rename = "type")]
	pub type_: Type,
	pub time_in_force: TimeInForce,
	pub limit_price: Option<BigDecimal>,
	pub stop_price: Option<BigDecimal>,
	pub trail_price: Option<BigDecimal>,
	pub trail_percent: Option<BigDecimal>,
	pub take_profit: Option<TakeProfit>,
	pub stop_loss: Option<StopLoss>,
	pub extended_hours: bool,
	pub client_order_id: Option<String>,
}

#[derive(Clone, Debug, Default, Deserialize, Eq, PartialEq, Serialize)]
pub struct OrderPatchRequest {
	#[serde(rename = "qty")]
	pub quantity: Option<BigDecimal>,
	pub time_in_force: Option<TimeInForce>,
	pub limit_price: Option<BigDecimal>,
	pub stop_price: Option<BigDecimal>,
	pub trail: Option<BigDecimal>,
	pub client_order_id: Option<String>,
}

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct OrderDTO {
	pub id: Uuid,
	pub client_order_id: String,
	pub status: Status,
	pub created_at: DateTime<Utc>,
	pub updated_at: Option<DateTime<Utc>>,
	pub submitted_at: Option<DateTime<Utc>>,
	pub filled_at: Option<DateTime<Utc>>,
	pub expired_at: Option<DateTime<Utc>>,
	pub canceled_at: Option<DateTime<Utc>>,
	pub asset_class: Class,
	pub asset_id: Uuid,
	pub symbol: String,
	#[serde(flatten)]
	pub amount: Amount,
	#[serde(rename = "filled_qty")]
	pub filled_quantity: BigDecimal,
	#[serde(rename = "type")]
	pub type_: Type,
	#[serde(rename = "order_class")]
	pub class: Class,
	pub side: SideBS,
	pub time_in_force: TimeInForce,
	pub limit_price: Option<BigDecimal>,
	pub stop_price: Option<BigDecimal>,
	pub trail_price: Option<BigDecimal>,
	pub trail_percent: Option<BigDecimal>,
	#[serde(rename = "filled_avg_price")]
	pub average_fill_price: Option<BigDecimal>,
	pub extended_hours: bool,
	pub legs: Vec<OrderDTO>,
}
