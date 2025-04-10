#[cfg(feature = "server")]
use {crate::clients::db::diesel_schema::sql_types, diesel::prelude::*, std::io::Write};
use {
	chrono::Utc,
	serde::{Deserialize, Serialize},
	std::fmt,
	uuid::Uuid,
};

pub mod apis;
pub mod diesel_demo;
pub mod sqlx_demo;

#[cfg(feature = "server")]
pub mod diesel_schema;

#[derive(Debug, Default, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[cfg_attr(feature = "server", derive(Queryable, Selectable, Insertable))]
#[cfg_attr(feature = "server", diesel(table_name = crate::clients::db::diesel_schema::users))]
#[cfg_attr(feature = "server", diesel(check_for_backend(diesel::pg::Pg)))]
pub struct DieselUser {
	pub id: Uuid,
	pub username: String,
	pub email: String,
	pub bio: Option<String>,
	pub age: Option<i32>,
	pub role: DieselRole,
	pub created_at: chrono::DateTime<Utc>,
	pub updated_at: chrono::DateTime<Utc>,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "server", derive(diesel::FromSqlRow, diesel::AsExpression))]
#[cfg_attr(feature = "server", diesel(sql_type = sql_types::UserRole))]
pub enum DieselRole {
	Admin,
	Moderator,
	User,
}

#[cfg(feature = "server")]
impl diesel::serialize::ToSql<sql_types::UserRole, diesel::pg::Pg> for DieselRole {
	fn to_sql<'b>(&self, out: &mut diesel::serialize::Output<'b, '_, diesel::pg::Pg>) -> diesel::serialize::Result {
		match *self {
			DieselRole::Admin => out.write_all(b"Admin")?,
			DieselRole::Moderator => out.write_all(b"Moderator")?,
			DieselRole::User => out.write_all(b"User")?,
		}
		Ok(diesel::serialize::IsNull::No)
	}
}

#[cfg(feature = "server")]
impl diesel::deserialize::FromSql<sql_types::UserRole, diesel::pg::Pg> for DieselRole {
	fn from_sql(bytes: diesel::pg::PgValue<'_>) -> diesel::deserialize::Result<Self> {
		let byte_data = bytes.as_bytes();
		match byte_data {
			b"Admin" => Ok(DieselRole::Admin),
			b"Moderator" => Ok(DieselRole::Moderator),
			b"User" => Ok(DieselRole::User),
			_ => Err("Invalid role".into()),
		}
	}
}

impl Default for DieselRole {
	fn default() -> Self {
		Self::User
	}
}

// Sqlx
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[cfg_attr(feature = "server", derive(sqlx::FromRow))]
pub struct SqlxUser {
	pub id: Uuid,
	pub username: String,
	pub email: String,
	pub bio: Option<String>,
	pub age: Option<i32>,
	pub role: SqlxRole,
	pub created_at: chrono::DateTime<Utc>,
	pub updated_at: chrono::DateTime<Utc>,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "server", derive(sqlx::Type))]
#[cfg_attr(feature = "server", sqlx(type_name = "user_role", rename_all = "PascalCase"))]
pub enum SqlxRole {
	Admin,
	Moderator,
	User,
}

impl fmt::Display for SqlxRole {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		match self {
			SqlxRole::Admin => write!(f, "Admin"),
			SqlxRole::Moderator => write!(f, "Moderator"),
			SqlxRole::User => write!(f, "User"),
		}
	}
}
