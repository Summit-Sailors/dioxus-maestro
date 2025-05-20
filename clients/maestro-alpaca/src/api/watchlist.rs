use {
	super::assets::Asset,
	chrono::{DateTime, Utc},
	serde::{Deserialize, Serialize},
	uuid::Uuid,
};

#[derive(Debug, Deserialize, Eq, PartialEq)]
pub struct Watchlist {
	pub id: Uuid,
	pub name: String,
	pub account_id: Uuid,
	pub created_at: DateTime<Utc>,
	pub updated_at: DateTime<Utc>,
	pub assets: Vec<Asset>,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
pub struct UpdateReq {
	pub name: String,
	pub symbols: Vec<String>,
}
