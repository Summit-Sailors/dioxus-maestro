#![allow(non_snake_case)]
use {
	demo::router::Route,
	dioxus::{logger::tracing::Level, prelude::*},
};

fn App() -> Element {
	rsx! {
		document::Stylesheet { href: asset!("./assets/main.css") }
		Router::<Route> {}
	}
}

fn main() {
	dioxus::logger::init(Level::INFO).expect("logger failed to initialize");
	dioxus::LaunchBuilder::new()
		.with_context(server_only!(maestro_diesel::async_client::client::acreate_diesel_pool(env!("DATABASE_URL"))))
		.with_context(server_only!({ dioxus::fullstack::ServeConfig::builder().enable_out_of_order_streaming() }))
		.with_context(server_only!(
			maestro_apalis::create::create_apalis_storage_sync::<demo::clients::utilities::EmailJob>()
				.maybe_db_url(Some(std::env!("APALIS_DATABASE_URL").to_string().leak()))
		))
		.with_context(server_only!(maestro_alpaca::get_client::get_alpaca_reqwest_client(std::env!("ALPACA_KEY_ID"), std::env!("ALPACA_SECRET_KEY"))))
		.with_context(server_only!(maestro_anthropic::AnthropicClient::new(std::env!("ANTHROPIC_API_KEY"))))
		.launch(App);
}
