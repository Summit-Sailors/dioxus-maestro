#![allow(non_snake_case)]
use {demo::router::Route, dioxus::prelude::*, dioxus_logger::tracing::Level};

fn App() -> Element {
	rsx! {
		Router::<Route> {}
	}
}

fn main() {
	dioxus_logger::init(Level::INFO).expect("logger failed to initialize");
	dioxus::LaunchBuilder::new().launch(App);
}
