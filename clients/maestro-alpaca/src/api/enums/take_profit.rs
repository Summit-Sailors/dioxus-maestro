use {
	bigdecimal::BigDecimal,
	serde::{Deserialize, Serialize},
};

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename = "take_profit")]
struct TakeProfitSerde {
	limit_price: BigDecimal,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub enum TakeProfit {
	Limit(BigDecimal),
}

impl From<TakeProfitSerde> for TakeProfit {
	fn from(other: TakeProfitSerde) -> Self {
		Self::Limit(other.limit_price)
	}
}

impl From<TakeProfit> for TakeProfitSerde {
	fn from(other: TakeProfit) -> Self {
		match other {
			TakeProfit::Limit(limit_price) => Self { limit_price },
		}
	}
}
