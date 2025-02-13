
#![allow(non_snake_case)]

use {dioxus::prelude::*, dioxus_logger::tracing::Level, demo::router::Route};

fn App() -> Element {
  rsx! {
    document::Link {
			rel: "stylesheet",
			href: "https://raw.githubusercontent.com/oovm/prism-wasm/dev/projects/prism-wasm/src/prism.min.css",
		}
    Router::<Route> {}
  }
}

fn main() {
  dioxus_logger::init(Level::INFO).expect("logger failed to initialize");
  dioxus::LaunchBuilder::new().launch(App);
}
