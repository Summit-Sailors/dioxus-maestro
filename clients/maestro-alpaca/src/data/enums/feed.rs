use serde::{Deserialize, Serialize};
use strum_macros::{Display, EnumIter, EnumString};

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, EnumIter, Display, EnumString)]
pub enum Feed {
	/// Use the Investors Exchange (IEX) as the data source.
	///
	/// This feed is available unconditionally, i.e., with the free and
	/// unlimited plans.
	#[serde(rename = "iex")]
	#[default]
	#[strum(to_string = "IEX")]
	Iex,
	/// Use CTA (administered by NYSE) and UTP (administered by Nasdaq)
	/// SIPs as the data source.
	///
	/// This feed is only usable with the unlimited market data plan.
	#[serde(rename = "sip")]
	#[strum(to_string = "SIP")]
	Sip,
}
