use misanthropic::Client;

pub type MisanthropicClient = misanthropic::Client;

pub fn create_misanthropic_client(api_key: &str) -> MisanthropicClient {
	Client::new(api_key.to_string()).expect("couldnt create athropic client")
}
