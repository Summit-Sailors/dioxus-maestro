use diesel_async::{pooled_connection::deadpool, AsyncPgConnection};

pub type AsyncDieselConn = deadpool::Object<AsyncPgConnection>;
pub type AsyncDieselPool = deadpool::Pool<AsyncPgConnection>;

pub mod client;

#[cfg(feature = "dioxus")]
pub mod from_server;
