#![allow(non_snake_case)]
use {dioxus::prelude::*, dioxus_logger::tracing::Level, headless_demo::router::Route};

fn App() -> Element {
	rsx! {
		head {
			document::Link { rel: "icon", href: asset!("/assets/favicon.ico") }
			document::Link { rel: "stylesheet", href: asset!("/assets/tailwind.css") }
			document::Link { rel: "preconnect", href: "https://fonts.googleapis.com" }
			document::Link {
				rel: "stylesheet",
				href: "https://fonts.googleapis.com/css2?family=Poppins:ital,wght@0,400;0,500;0,600;0,700;1,400;1,500;1,600;1,700&display=swap",
			}
		}
		Router::<Route> {}
	}
}

fn main() {
	dioxus_logger::init(Level::INFO).expect("logger failed to initialize");
	dioxus::LaunchBuilder::new().launch(App);
}
