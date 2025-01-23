#![allow(non_snake_case)]

use {dioxus::prelude::*, maestro_toast::toast_manager::ToastManager};

mod components;
mod models;
mod pages;
mod routes;

fn main() {
	//  logger
	wasm_logger::init(wasm_logger::Config::default());

	launch(App);
}

#[component]
fn App() -> Element {
	use_context_provider::<Signal<ToastManager>>(|| Signal::new(ToastManager::default()));
	rsx! {
		Router::<routes::Route> {}
	}
}
