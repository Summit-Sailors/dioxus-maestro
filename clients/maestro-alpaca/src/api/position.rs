use {
	super::{
		assets::{Class, Exchange},
		enums::side::SideLS,
	},
	bigdecimal::BigDecimal,
	serde::{Deserialize, Serialize},
	uuid::Uuid,
};

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct Position {
	pub asset_id: Uuid,
	pub symbol: String,
	pub exchange: Exchange,
	pub asset_class: Class,
	#[serde(rename = "avg_entry_price")]
	pub average_entry_price: BigDecimal,
	#[serde(rename = "qty")]
	pub quantity: BigDecimal,
	#[serde(rename = "qty_available")]
	pub quantity_available: BigDecimal,
	pub side: SideLS,
	pub market_value: Option<BigDecimal>,
	pub cost_basis: BigDecimal,
	#[serde(rename = "unrealized_pl")]
	pub unrealized_gain_total: Option<BigDecimal>,
	#[serde(rename = "unrealized_plpc")]
	pub unrealized_gain_total_percent: Option<BigDecimal>,
	#[serde(rename = "unrealized_intraday_pl")]
	pub unrealized_gain_today: Option<BigDecimal>,
	#[serde(rename = "unrealized_intraday_plpc")]
	pub unrealized_gain_today_percent: Option<BigDecimal>,
	pub current_price: Option<BigDecimal>,
	#[serde(rename = "lastday_price")]
	pub last_day_price: Option<BigDecimal>,
	pub change_today: Option<BigDecimal>,
}
