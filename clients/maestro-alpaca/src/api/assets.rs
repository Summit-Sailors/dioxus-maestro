use {
	crate::{
		alpaca_env::AlpacaUrls,
		routes::{EAlpacaRoute, EApiRoute},
	},
	reqwest::Client,
	serde::{Deserialize, Serialize},
	std::fmt::{Display, Formatter, Result as FmtResult},
	uuid::{Error as UuidError, Uuid},
};

#[derive(Clone, Copy, Debug, Default, Deserialize, Eq, PartialEq, Serialize)]
pub enum Class {
	#[serde(rename = "us_equity")]
	#[default]
	UsEquity,
	#[serde(rename = "crypto")]
	Crypto,
}

#[derive(Clone, Copy, Debug, Default, Deserialize, Eq, PartialEq, Serialize)]
pub enum Status {
	#[serde(rename = "active")]
	#[default]
	Active,
	#[serde(rename = "inactive")]
	Inactive,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum ParseSymbolError {
	InvalidSymbol(char),
	UnknownExchange,
	UnknownClass,
	InvalidId(UuidError),
	InvalidFormat,
}

impl Display for ParseSymbolError {
	fn fmt(&self, fmt: &mut Formatter<'_>) -> FmtResult {
		match self {
			Self::InvalidSymbol(c) => write!(fmt, "the symbol contains an invalid character ('{c}')"),
			Self::UnknownExchange => fmt.write_str("the exchange is unknown"),
			Self::UnknownClass => fmt.write_str("the asset class is unknown"),
			Self::InvalidId(err) => write!(fmt, "failed to parse asset ID: {err}"),
			Self::InvalidFormat => fmt.write_str("the symbol is of an invalid format"),
		}
	}
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub enum Symbol {
	Sym(String),
	SymExchg(String, Exchange),
	SymExchgCls(String, Exchange, Class),
	Id(Uuid),
}

impl From<Uuid> for Symbol {
	fn from(symbol: Uuid) -> Self {
		Self::Id(symbol)
	}
}

#[derive(Clone, Copy, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub enum Exchange {
	#[serde(rename = "AMEX")]
	Amex,
	#[serde(rename = "ARCA")]
	Arca,
	#[serde(rename = "BATS")]
	Bats,
	#[serde(rename = "NASDAQ")]
	Nasdaq,
	#[serde(rename = "NYSE")]
	Nyse,
	#[serde(rename = "NYSEARCA")]
	Nysearca,
	#[serde(rename = "OTC")]
	Otc,
}

impl AsRef<str> for Exchange {
	fn as_ref(&self) -> &'static str {
		match *self {
			Exchange::Amex => "AMEX",
			Exchange::Arca => "ARCA",
			Exchange::Bats => "BATS",
			Exchange::Nasdaq => "NASDAQ",
			Exchange::Nyse => "NYSE",
			Exchange::Nysearca => "NYSEARCA",
			Exchange::Otc => "OTC",
		}
	}
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct Asset {
	pub id: Uuid,
	pub class: Class,
	pub exchange: Exchange,
	pub symbol: String,
	pub status: Status,
	pub tradable: bool,
	pub marginable: bool,
	pub shortable: bool,
	pub easy_to_borrow: bool,
	pub fractionable: bool,
}

#[derive(Clone, Copy, Debug, Default, Deserialize, Eq, PartialEq, Serialize)]
struct AssetsGetRequest {
	status: Option<Status>,
	asset_class: Option<Class>,
	exchange: Option<Exchange>,
}

#[bon::builder]
pub async fn assets_get_request(
	urls: AlpacaUrls,
	status: Option<Status>,
	asset_class: Option<Class>,
	exchange: Option<Exchange>,
	#[builder(default = Client::new())] client: Client,
) -> Result<Asset, reqwest::Error> {
	client.get(EAlpacaRoute::Api(EApiRoute::Assets).url_path(urls)).query(&AssetsGetRequest { status, asset_class, exchange }).send().await?.json().await
}
