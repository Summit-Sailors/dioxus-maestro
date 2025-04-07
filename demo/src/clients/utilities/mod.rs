use serde::{Deserialize, Serialize};

pub mod alpaca_demo;
pub mod anthropic_demo;
pub mod apalis_demo;
pub mod apis;
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
