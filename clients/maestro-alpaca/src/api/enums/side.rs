use {
	serde::{Deserialize, Serialize},
	std::ops::Not,
};

#[derive(Clone, Copy, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub enum SideBSSS {
	#[serde(rename = "buy")]
	Buy,
	#[serde(rename = "sell")]
	Sell,
	#[serde(rename = "sell_short")]
	ShortSell,
}

#[derive(Clone, Copy, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub enum SideBS {
	#[serde(rename = "buy")]
	Buy,
	#[serde(rename = "sell")]
	Sell,
}

impl Not for SideBS {
	type Output = Self;

	
	fn not(self) -> Self::Output {
		match self {
			Self::Buy => Self::Sell,
			Self::Sell => Self::Buy,
		}
	}
}

#[derive(Clone, Copy, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub enum SideLS {
	#[serde(rename = "long")]
	Long,
	#[serde(rename = "short")]
	Short,
}

impl Not for SideLS {
	type Output = Self;

	
	fn not(self) -> Self::Output {
		match self {
			Self::Long => Self::Short,
			Self::Short => Self::Long,
		}
	}
}
