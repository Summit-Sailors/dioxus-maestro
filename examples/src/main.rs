#![allow(non_snake_case)]

use dioxus::prelude::*;
use dioxus_router::prelude::*;

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
fn App(cx: Scope) -> Element {
  cx.render(rsx! {
    Router::<routes::Route> {}
  })
}
