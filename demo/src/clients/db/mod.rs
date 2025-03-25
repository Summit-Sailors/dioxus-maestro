#[cfg(feature = "server")]
use {crate::clients::db::diesel_schema::sql_types, diesel::prelude::*, std::io::Write};
use {
	chrono::Utc,
	serde::{Deserialize, Serialize},
	uuid::Uuid,
};

pub mod diesel_demo;
// pub mod sqlx;

pub mod diesel_api;

#[cfg(feature = "server")]
pub mod diesel_schema;

#[derive(Debug, Default, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[cfg_attr(feature = "server", derive(Queryable, Selectable, Insertable))]
#[cfg_attr(feature = "server", diesel(table_name = crate::clients::db::diesel_schema::users))]
#[cfg_attr(feature = "server", diesel(check_for_backend(diesel::pg::Pg)))]
pub struct ClientsUser {
	pub id: Uuid,
	pub username: String,
	pub email: String,
	pub bio: Option<String>,
	pub age: Option<i32>,
	pub role: ClientsRole,
	pub created_at: chrono::DateTime<Utc>,
	pub updated_at: chrono::DateTime<Utc>,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "server", derive(diesel::FromSqlRow, diesel::AsExpression))]
#[cfg_attr(feature = "server", diesel(sql_type = sql_types::UserRole))]
pub enum ClientsRole {
	Admin,
	Moderator,
	User,
}

#[cfg(feature = "server")]
impl diesel::serialize::ToSql<sql_types::UserRole, diesel::pg::Pg> for ClientsRole {
	fn to_sql<'b>(&self, out: &mut diesel::serialize::Output<'b, '_, diesel::pg::Pg>) -> diesel::serialize::Result {
		match *self {
			ClientsRole::Admin => out.write_all(b"Admin")?,
			ClientsRole::Moderator => out.write_all(b"Moderator")?,
			ClientsRole::User => out.write_all(b"User")?,
		}
		Ok(diesel::serialize::IsNull::No)
	}
}

#[cfg(feature = "server")]
impl diesel::deserialize::FromSql<sql_types::UserRole, diesel::pg::Pg> for ClientsRole {
	fn from_sql(bytes: diesel::pg::PgValue<'_>) -> diesel::deserialize::Result<Self> {
		let byte_data = bytes.as_bytes();
		match byte_data {
			b"Admin" => Ok(ClientsRole::Admin),
			b"Moderator" => Ok(ClientsRole::Moderator),
			b"User" => Ok(ClientsRole::User),
			_ => Err("Invalid role".into()),
		}
	}
}

impl Default for ClientsRole {
	fn default() -> Self {
		Self::User
	}
}
