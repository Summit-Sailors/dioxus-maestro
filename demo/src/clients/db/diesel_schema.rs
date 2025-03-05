// @generated automatically by Diesel CLI.

pub mod sql_types {
	#[derive(diesel::query_builder::QueryId, diesel::sql_types::SqlType)]
	#[diesel(postgres_type(name = "user_role"))]
	pub struct UserRole;
}

diesel::table! {
		use diesel::sql_types::*;
		use super::sql_types::UserRole;

		users (id) {
				id -> Uuid,
				username -> Text,
				email -> Text,
				bio -> Nullable<Text>,
				age -> Nullable<Int4>,
				role -> UserRole,
				created_at -> Timestamptz,
				updated_at -> Timestamptz,
		}
}
