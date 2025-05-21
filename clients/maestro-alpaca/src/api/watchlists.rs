use {
	chrono::{DateTime, Utc},
	serde::Deserialize,
	uuid::Uuid,
};

#[derive(Debug, Clone, PartialEq, Eq, Deserialize)]
pub struct WatchlistItem {
	pub id: Uuid,
	pub name: String,
	pub account_id: Uuid,
	pub created_at: DateTime<Utc>,
	pub updated_at: DateTime<Utc>,
}
