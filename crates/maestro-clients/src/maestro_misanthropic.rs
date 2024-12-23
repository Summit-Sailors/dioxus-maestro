use misanthropic::Client;

pub fn create_misanthropic_client() -> Client {
	Client::new(std::env::var("ANTHROPIC_API_KEY").expect("ANTHROPIC_API_KEY ENV VAR")).expect("couldnt create athropic client")
}
