#![allow(non_snake_case)]

mod cached_result;
mod result;
mod use_mutation;
mod use_query;
mod use_query_client;

pub mod prelude {
	pub use futures_util;

	pub use crate::{cached_result::*, result::*, use_mutation::*, use_query::*, use_query_client::*};
}
