use {
	super::assets::Asset,
	chrono::{DateTime, Utc},
	serde::{Deserialize, Serialize},
	uuid::Uuid,
};

#[derive(Debug, PartialEq, Eq, Deserialize)]
pub struct Watchlist {
	pub id: Uuid,
	pub name: String,
	pub account_id: Uuid,
	pub created_at: DateTime<Utc>,
	pub updated_at: DateTime<Utc>,
	pub assets: Vec<Asset>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct UpdateReq {
	pub name: String,
	pub symbols: Vec<String>,
}
