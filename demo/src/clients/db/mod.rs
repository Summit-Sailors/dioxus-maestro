use serde::{Deserialize, Serialize};

pub mod diesel;
// pub mod sqlx;

pub mod api;

#[cfg(feature = "server")]
pub mod diesel_schema;

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct ClientsUser {
	username: String,
	email: String,
	bio: String,
	age: i32,
	role: ClientsRole,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq, Hash)]
pub enum ClientsRole {
	Admin,
	User,
	Moderator,
}
