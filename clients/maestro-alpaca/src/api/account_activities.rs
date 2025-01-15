use {
	super::enums::{activity_type::ActivityType, direction::Direction, side::SideBSSS},
	crate::{
		alpaca_env::AlpacaUrls,
		routes::{EAlpacaRoute, EApiRoute},
	},
	bigdecimal::BigDecimal,
	chrono::{DateTime, Utc},
	reqwest::Client,
	serde::{Deserialize, Serialize},
	uuid::Uuid,
};

#[derive(Debug, Default, Clone, PartialEq, Eq, Serialize, Deserialize)]
struct ActivityRequest {
	#[serde(rename = "activity_types")]
	types: Vec<ActivityType>,
	#[serde(skip_serializing_if = "Option::is_none")]
	direction: Option<Direction>,
	#[serde(skip_serializing_if = "Option::is_none")]
	until: Option<DateTime<Utc>>,
	#[serde(skip_serializing_if = "Option::is_none")]
	after: Option<DateTime<Utc>>,
	#[serde(skip_serializing_if = "Option::is_none")]
	page_size: Option<usize>,
	#[serde(skip_serializing_if = "Option::is_none")]
	page_token: Option<String>,
}

#[bon::builder]
pub async fn account_activities_get_request(
	urls: AlpacaUrls,
	types: Vec<ActivityType>,
	direction: Option<Direction>,
	until: Option<DateTime<Utc>>,
	after: Option<DateTime<Utc>>,
	page_size: Option<usize>,
	page_token: Option<String>,
	#[builder(default = Client::new())] client: Client,
) -> Result<Activity, reqwest::Error> {
	client
		.get(EAlpacaRoute::Api(EApiRoute::AccountActivities).url_path(urls))
		.query(&ActivityRequest { types, direction, until, after, page_size, page_token })
		.send()
		.await?
		.json()
		.await
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct TradeActivity {
	pub id: String,
	pub transaction_time: DateTime<Utc>,
	pub symbol: String,
	pub order_id: Uuid,
	pub side: SideBSSS,
	#[serde(rename = "qty")]
	pub quantity: BigDecimal,
	#[serde(rename = "cum_qty")]
	pub cumulative_quantity: BigDecimal,
	#[serde(rename = "leaves_qty")]
	pub unfilled_quantity: BigDecimal,
	/// The per-share price that the trade was executed at.
	pub price: BigDecimal,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct NonTradeActivity {
	pub id: String,
	#[serde(rename = "activity_type")]
	pub type_: ActivityType,
	pub date: DateTime<Utc>,
	pub net_amount: BigDecimal,
	pub symbol: Option<String>,
	#[serde(rename = "qty")]
	pub quantity: Option<BigDecimal>,
	pub price: Option<BigDecimal>,
	pub per_share_amount: Option<BigDecimal>,
	pub description: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum Activity {
	Trade(TradeActivity),
	NonTrade(NonTradeActivity),
}

impl Activity {
	pub fn id(&self) -> &str {
		match self {
			Activity::Trade(trade) => &trade.id,
			Activity::NonTrade(non_trade) => &non_trade.id,
		}
	}

	pub fn time(&self) -> &DateTime<Utc> {
		match self {
			Activity::Trade(trade) => &trade.transaction_time,
			Activity::NonTrade(non_trade) => &non_trade.date,
		}
	}

	#[allow(clippy::result_large_err)]
	pub fn into_trade(self) -> Result<TradeActivity, Self> {
		match self {
			Activity::Trade(trade) => Ok(trade),
			Activity::NonTrade(..) => Err(self),
		}
	}

	#[allow(clippy::result_large_err)]
	pub fn into_non_trade(self) -> Result<NonTradeActivity, Self> {
		match self {
			Activity::Trade(..) => Err(self),
			Activity::NonTrade(non_trade) => Ok(non_trade),
		}
	}
}
