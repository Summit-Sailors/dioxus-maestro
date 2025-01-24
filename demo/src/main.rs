#![allow(non_snake_case)]

use {
  dioxus::prelude::*, dioxus_logger::tracing::Level, maestro_toast::{init::use_init_toast_ctx, toast_frame_component::ToastFrame}, maestro_ui::
    button::{
      Button,
      ButtonType, 
      ButtonVariant
    },
};

mod components;
mod models;
mod pages;

fn main() {
	//  logger
  dioxus_logger::init(Level::INFO).expect("logger failed to init");
	launch(App);
}

#[derive(PartialEq, Clone, Copy)]
enum DemoView {
  Home,
  Forms,
  Hooks,
  Plotters,
  Query,
  Radio,
  Toast,
  Ui,
}

#[component]
fn App() -> Element {
  let toast = use_init_toast_ctx();
  let current_view = use_signal(|| DemoView::Home);

  rsx! {
    ToastFrame { manager: toast }
    div { class: "min-h-screen bg-gray-50",
      nav { class: "bg-white shadow-sm",
        div { class: "max-w-7xl mx-auto px-4 sm:px-6 lg:px-8",
          div { class: "flex justify-between h-16",
            div { class: "flex",
              div { class: "flex-shrink-0 flex items-center",
                img { class: "h-8 w-auto", src: "/logo.svg", alt: "Maestro Demo" }
              }
              div { class: "hidden sm:ml-6 sm:flex sm:space-x-8",
                NavigationLink { view: DemoView::Forms, current_view, "Forms" }
                NavigationLink { view: DemoView::Hooks, current_view, "Hooks" }
                NavigationLink { view: DemoView::Plotters, current_view, "Charts" }
                NavigationLink { view: DemoView::Query, current_view, "Query" }
                NavigationLink { view: DemoView::Radio, current_view, "Radio" }
                NavigationLink { view: DemoView::Toast, current_view, "Toast" }
                NavigationLink { view: DemoView::Ui, current_view, "UI" }
              }
            }
          }
        }
      }
      main { class: "py-10",
        div { class: "max-w-7xl mx-auto px-4 sm:px-6 lg:px-8",
          {
            let view = current_view.read();
            match *view {
              DemoView::Home => rsx! { pages::home::HomePage { current_view } },
              DemoView::Forms => rsx! { pages::forms::FormsDemo {} },
              DemoView::Hooks => rsx! { pages::hooks::HooksDemo {} },
              DemoView::Plotters => rsx! { pages::plotters::PlottersDemo {} },
              DemoView::Query => rsx! { pages::query::QueryDemo {} },
              DemoView::Radio => rsx! { pages::radio::RadioDemo {} },
              DemoView::Toast => rsx! { pages::toast::ToastDemo {} },
              DemoView::Ui => rsx! { pages::ui::UIDemo {} },
            }
          }
        }
      }
    }
  }
}

#[component]
fn NavigationLink(view: DemoView, current_view: Signal<DemoView>, children: Element) -> Element {
  let is_active = *current_view.read() == view;
  rsx! {
    Button {
      class: format!(
        "inline-flex items-center px-1 pt-1 border-b-2 text-sm font-medium {}",
        if is_active {
          "border-blue-500 text-gray-900"
        } else {
          "border-transparent text-gray-500 hover:border-gray-300 hover:text-gray-700"
        }
      ),
      variant: ButtonVariant::Default,
      button_type: ButtonType::Button,
      on_click: move |_| current_view.set(view),
      children
    }
  }
}
