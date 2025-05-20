use {
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

#[bon::builder]
pub async fn account_get_request(urls: AlpacaUrls, #[builder(default = Client::new())] client: Client) -> Result<Account, reqwest::Error> {
	client.get(EAlpacaRoute::Api(EApiRoute::AccountActivities).url_path(urls)).send().await?.json().await
}

#[derive(Clone, Copy, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub enum Status {
	#[serde(rename = "ONBOARDING")]
	Onboarding,
	#[serde(rename = "SUBMISSION_FAILED")]
	SubmissionFailed,
	#[serde(rename = "SUBMITTED")]
	Submitted,
	#[serde(rename = "ACCOUNT_UPDATED")]
	Updating,
	#[serde(rename = "APPROVAL_PENDING")]
	ApprovalPending,
	#[serde(rename = "ACTIVE")]
	Active,
	#[serde(rename = "REJECTED")]
	Rejected,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct Account {
	pub id: Uuid,
	pub status: Status,
	pub currency: String,
	pub cash: BigDecimal,
	#[serde(rename = "pattern_day_trader")]
	pub day_trader: bool,
	#[serde(rename = "trade_suspended_by_user")]
	pub trading_suspended: bool,
	pub trading_blocked: bool,
	pub transfers_blocked: bool,
	pub account_blocked: bool,
	pub created_at: DateTime<Utc>,
	pub shorting_enabled: bool,
	#[serde(rename = "long_market_value")]
	pub market_value_long: BigDecimal,
	#[serde(rename = "short_market_value")]
	pub market_value_short: BigDecimal,
	pub equity: BigDecimal,
	pub last_equity: BigDecimal,
	pub multiplier: BigDecimal,
	pub buying_power: BigDecimal,
	pub initial_margin: BigDecimal,
	pub maintenance_margin: BigDecimal,
	pub daytrade_count: u64,
}
