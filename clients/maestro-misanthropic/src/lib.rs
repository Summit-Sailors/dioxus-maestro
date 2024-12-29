use misanthropic::Client;

pub type MisanthropicClient = misanthropic::Client;

pub fn create_misanthropic_client() -> MisanthropicClient {
	Client::new(std::env::var("ANTHROPIC_API_KEY").expect("ANTHROPIC_API_KEY ENV VAR")).expect("couldnt create athropic client")
}
