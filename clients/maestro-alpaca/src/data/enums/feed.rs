use {
	serde::{Deserialize, Serialize},
	strum_macros::{Display, EnumIter, EnumString},
};

#[derive(Clone, Copy, Debug, Default, Deserialize, Display, EnumIter, EnumString, Eq, PartialEq, Serialize)]
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
