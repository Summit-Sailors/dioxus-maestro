use serde::{Deserialize, Serialize};

pub mod alpaca_demo;
pub mod apalis_api;
pub mod apalis_demo;
pub mod serpapi_api;
pub mod serpapi_demo;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct EmailJob {
	pub to: String,
	pub subject: String,
	pub body: String,
}

impl EmailJob {
	pub fn new(to: String, subject: String, body: String) -> EmailJob {
		EmailJob { to, subject, body }
	}
}

#[cfg(feature = "server")]
impl maestro_apalis::Job for EmailJob {
	const NAME: &'static str = "email_job";
}
