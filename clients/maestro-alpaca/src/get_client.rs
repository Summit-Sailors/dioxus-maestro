use {
	reqwest::{
		header::{HeaderName, HeaderValue},
		Client,
	},
	std::ops::Deref,
};

const HDR_KEY_ID: &str = "apca-api-key-id";
const HDR_SECRET: &str = "apca-api-secret-key";

#[derive(Clone)]
pub struct AlpacaClient(Client);

impl Deref for AlpacaClient {
	type Target = Client;

	fn deref(&self) -> &Self::Target {
		&self.0
	}
}

pub fn get_alpaca_reqwest_client(key_id: &str, secret: &str) -> AlpacaClient {
	AlpacaClient(
		Client::builder()
			.default_headers(
				[
					(HeaderName::from_static(HDR_KEY_ID), HeaderValue::from_str(key_id).unwrap()),
					(HeaderName::from_static(HDR_SECRET), HeaderValue::from_str(secret).unwrap()),
				]
				.into_iter()
				.collect(),
			)
			.build()
			.expect("COULDNT BUILD CLIENT!!"),
	)
}
