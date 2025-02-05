
#![allow(non_snake_case)]

use {dioxus::prelude::*, dioxus_logger::tracing::Level, demo::router::Route};

fn App() -> Element {
  rsx! {
    Router::<Route> {}
  }
}

fn main() {
  dioxus_logger::init(Level::INFO).expect("logger failed to init");
  dioxus::LaunchBuilder::new().launch(App);
}
