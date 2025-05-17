use serde::{Deserialize, Serialize};
use strum_macros::{Display, EnumIter, EnumString};

use super::timeframe::TimeFrame;

pub trait DataType: Serialize + Deserialize<'static> + Clone + PartialEq + Eq + std::fmt::Debug + Send + Sync + 'static {}

/// Scope of data to be fetched for the data type specified in the market data class
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, EnumIter, Display, EnumString)]
#[serde(rename_all = "lowercase")]
pub enum MarketDataScope {
	/// Historical data for multiple symbols
	Historical,
	/// Latest data for multiple symbols
	Latest,
	/// Historical data for single symbol
	#[serde(rename = "historical_single")]
	HistoricalSingle,
	/// Latest data for single symbol
	#[serde(rename = "latest_single")]
	LatestSingle,
	/// Historical and Latest data for multiple symbols
	#[default]
	All,
}

/// Majorly focused on OLHCV data AKA Bars data
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Hash, Default)]
#[serde(rename_all = "lowercase")]
pub enum AssetClass {
	/// In Alpaca API's context, ETFs are processed through the same endpoint as stocks, There isn't a separate ETF-specific URL - ETFs are traded on stock
	/// exchanges and for API purposes are treated as stocks
	#[default]
	Stocks,
	Crypto,
	Options,
	/// Foreign exchange data types
	Forex,
}

/// Types of market reference codes
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq, Hash)]
pub enum CodeType {
	/// Trading condition codes
	#[serde(rename = "conditions")]
	Condition {
		/// The type of ticks (required)
		ticktype: String,
		/// The one character name of the tape (required)
		tape: String,
	},
	/// Exchange codes (additional parameters)
	#[serde(rename = "exchanges")]
	Exchange,
}

impl Default for CodeType {
	fn default() -> Self {
		CodeType::Condition { ticktype: String::from("Trade"), tape: String::from("P") }
	}
}

/// Market Data structure that combines asset class and data type
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq, Hash, EnumIter)]
#[serde(tag = "class", content = "type")]
pub enum MarketDataClass {
	/// Equity securities representing ownership in companies
	#[serde(rename = "stocks")]
	Stock {
		/// Stock data type information
		data_type: StocksDataType,
	},
	/// Cryptocurrency data types
	#[serde(rename = "crypto")]
	Crypto {
		/// Crypto data type information
		data_type: CryptoDataType,
	},
	/// Options data types
	#[serde(rename = "options")]
	Option {
		/// Options data type information
		data_type: OptionsDataType,
	},
	/// Foreign exchange data types
	#[serde(rename = "forex")]
	Forex {
		/// Forex data type information
		data_type: ForexDataType,
	},
	/// Financial news data types
	#[serde(rename = "news")]
	News {
		/// News data type information
		data_type: NewsDataType,
	},
	/// Screener data types
	#[serde(rename = "screener")]
	Screener {
		/// Screener data type information
		data_type: ScreenerDataType,
	},
	/// Company logos data types
	#[serde(rename = "logos")]
	Logos {
		/// Logos data type information
		data_type: LogosDataType,
	},
	/// Corporate actions data types
	#[serde(rename = "corporate-actions")]
	CorporateActions {
		/// Corporate actions data type information
		data_type: CorporateActionsDataType,
	},
	/// Fixed income data types
	#[serde(rename = "fixed_income")]
	FixedIncome {
		/// Fixed income data type information
		data_type: FixedIncomeDataType,
	},
}

impl Default for MarketDataClass {
	fn default() -> Self {
		Self::Stock { data_type: StocksDataType::Bars { timeframe: TimeFrame::Day } }
	}
}

impl DataType for MarketDataClass {}

/// Data types available for stocks
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq, Hash)]
#[serde(tag = "data_type")]
pub enum StocksDataType {
	/// OHLCV bars/candles with specified timeframe
	#[serde(rename = "bars")]
	Bars { timeframe: TimeFrame },

	/// Individual trades executed on markets
	#[serde(rename = "trades")]
	Trades,

	/// Order book snapshots providing market depth (Level 2 data)
	#[serde(rename = "snapshots")]
	Snapshots,

	/// Results of aggregated buy and sell orders matched at a specific time
	#[serde(rename = "auctions")]
	Auctions,

	/// Security reference data and metadata
	#[serde(rename = "codes")]
	Codes {
		/// Type of code/symbol information requested
		code_type: CodeType,
	},

	/// Best bid and ask prices with associated sizes (Level 1 data)
	#[serde(rename = "quotes")]
	Quotes,
}

impl Default for StocksDataType {
	fn default() -> Self {
		Self::Bars { timeframe: TimeFrame::Day }
	}
}

/// Data types available for cryptocurrency
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq, Hash)]
#[serde(tag = "data_type")]
pub enum CryptoDataType {
	/// OHLCV bars/candles with specified timeframe
	#[serde(rename = "bars")]
	Bars { timeframe: TimeFrame, loc: String },

	/// Individual trades executed on exchanges
	#[serde(rename = "trades")]
	Trades,

	/// Order book snapshots
	#[serde(rename = "snapshots")]
	Snapshots,

	/// Best bid and ask prices with associated sizes
	#[serde(rename = "quotes")]
	Quotes,

	/// Complete or partial order book showing pending buy/sell orders
	#[serde(rename = "orderbooks")]
	OrderBooks,
}

impl Default for CryptoDataType {
	fn default() -> Self {
		Self::Bars { timeframe: TimeFrame::Day, loc: "us".to_string() }
	}
}

/// Data types available for options
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq, Hash)]
#[serde(tag = "data_type")]
pub enum OptionsDataType {
	/// OHLCV bars/candles with specified timeframe
	#[serde(rename = "bars")]
	Bars { timeframe: TimeFrame },

	/// Order book snapshots
	#[serde(rename = "snapshots")]
	Snapshots,

	/// Best bid and ask prices with associated sizes
	#[serde(rename = "quotes")]
	Quotes,

	/// Security reference data and metadata
	#[serde(rename = "codes")]
	Codes {
		/// Type of code/symbol information requested
		code_type: CodeType,
	},

	/// Options chain data showing available strike prices and expiration dates
	#[serde(rename = "option_chain")]
	OptionChain,

	/// Individual trades executed on markets
	#[serde(rename = "trades")]
	Trades,
}

impl Default for OptionsDataType {
	fn default() -> Self {
		Self::Bars { timeframe: TimeFrame::Day }
	}
}

/// Data types available for fixed income
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq, Hash, Default)]
#[serde(tag = "data_type")]
pub enum FixedIncomeDataType {
	/// Current or historical price data for fixed income securities
	#[serde(rename = "prices")]
	#[default]
	LatestPrices,
}

/// Data types available for corporate actions
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq, Hash, Default)]
#[serde(tag = "data_type")]
pub enum CorporateActionsDataType {
	/// Dividends, splits, mergers, acquisitions and other corporate events
	#[serde(rename = "corporate-actions")]
	#[default]
	CorporateActions,
}

/// Data types available for logos
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq, Hash, Default)]
#[serde(tag = "data_type")]
pub enum LogosDataType {
	/// Company logos and brand images
	#[serde(rename = "logos")]
	#[default]
	Logos,
}

/// Data types available for screeners
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq, Hash, Default)]
#[serde(tag = "data_type")]
pub enum ScreenerDataType {
	/// Securities with high trading activity or volume
	#[serde(rename = "most-actives")]
	#[default]
	MostActive,

	/// Top gainers, losers, and most active securities
	#[serde(rename = "movers")]
	MarketMovers,
}

/// Data types available for news
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq, Hash, Default)]
#[serde(tag = "data_type")]
pub enum NewsDataType {
	/// Financial news articles and press releases
	#[serde(rename = "news")]
	#[default]
	Articles,
}

/// Data types available for forex
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq, Hash, Default)]
#[serde(tag = "data_type")]
pub enum ForexDataType {
	/// Exchange rates between different currencies (latest and historical)
	#[serde(rename = "rates")]
	#[default]
	CurrencyRates,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq, Hash, Default)]
#[serde(tag = "data_type")]
pub enum MarketDataType {
	/// OHLCV bars/candles with specified timeframe
	#[serde(rename = "bars")]
	#[default]
	Bars,
	/// Individual trades executed on markets
	#[serde(rename = "trades")]
	Trades,

	/// Order book snapshots providing market depth (Level 2 data)
	#[serde(rename = "snapshots")]
	Snapshots,

	/// Results of aggregated buy and sell orders matched at a specific time
	#[serde(rename = "auctions")]
	Auctions,

	/// Security reference data and metadata
	#[serde(rename = "codes")]
	Codes,

	/// Best bid and ask prices with associated sizes (Level 1 data)
	#[serde(rename = "quotes")]
	Quotes,

	/// Complete or partial order book showing pending buy/sell orders
	#[serde(rename = "orderbooks")]
	OrderBooks,

	/// Options chain data showing available strike prices and expiration dates
	#[serde(rename = "option_chain")]
	OptionChain,

	/// Current or historical price data for fixed income securities
	#[serde(rename = "prices")]
	LatestPrices,

	#[serde(rename = "corporate-actions")]
	CorporateActions,

	/// Company logos and brand images
	#[serde(rename = "logos")]
	Logos,
	/// Securities with high trading activity or volume
	#[serde(rename = "most-actives")]
	MostActive,

	/// Top gainers, losers, and most active securities
	#[serde(rename = "movers")]
	MarketMovers,

	/// Financial news articles and press releases
	#[serde(rename = "news")]
	Articles,
	/// Exchange rates between different currencies (latest and historical)
	#[serde(rename = "rates")]
	CurrencyRates,
}
