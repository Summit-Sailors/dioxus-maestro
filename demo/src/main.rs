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
	dioxus::LaunchBuilder::new().launch(App);
}
