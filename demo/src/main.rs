#![allow(non_snake_case)]

use {
  dioxus::prelude::*, 
  maestro_toast::toast_manager::ToastManager,
  dioxus_logger::tracing::Level
};

mod components;
mod models;
mod pages;
mod routes;

fn main() {
	//  logger
  dioxus_logger::init(Level::INFO).expect("logger failed to init");
	launch(App);
}

#[component]
fn App() -> Element {
	use_context_provider::<Signal<ToastManager>>(|| Signal::new(ToastManager::default()));
	rsx! {
		Router::<routes::Route> {}
	}
}
