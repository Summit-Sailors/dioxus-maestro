use {
	bigdecimal::BigDecimal,
	serde::{Deserialize, Serialize},
};

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename = "stop_loss")]
struct StopLossSerde {
	stop_price: BigDecimal,
	limit_price: Option<BigDecimal>,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub enum StopLoss {
	Stop(BigDecimal),
	StopLimit(BigDecimal, BigDecimal),
}

impl From<StopLossSerde> for StopLoss {
	fn from(other: StopLossSerde) -> Self {
		if let Some(limit_price) = other.limit_price {
			Self::StopLimit(other.stop_price, limit_price)
		} else {
			Self::Stop(other.stop_price)
		}
	}
}

impl From<StopLoss> for StopLossSerde {
	fn from(other: StopLoss) -> Self {
		match other {
			StopLoss::Stop(stop_price) => Self { stop_price, limit_price: None },
			StopLoss::StopLimit(stop_price, limit_price) => Self { stop_price, limit_price: Some(limit_price) },
		}
	}
}
