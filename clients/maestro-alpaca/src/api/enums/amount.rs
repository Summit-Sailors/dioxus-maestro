use {
	bigdecimal::BigDecimal,
	serde::{Deserialize, Serialize},
};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum Amount {
	Quantity { quantity: BigDecimal },
	Notional { notional: BigDecimal },
}

impl Amount {
	pub fn quantity(amount: impl Into<BigDecimal>) -> Self {
		Self::Quantity { quantity: amount.into() }
	}

	pub fn notional(amount: impl Into<BigDecimal>) -> Self {
		Self::Notional { notional: amount.into() }
	}
}
