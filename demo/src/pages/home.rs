use {
  crate::router::Route,
  dioxus::prelude::*,
  maestro_ui::button::{Button, ButtonVariant},
  strum::IntoEnumIterator,
};

#[component]
pub fn HomePage() -> Element {
  rsx! {
    div { class: "text-center py-16 h-full w-full flex justify-center items-center flex-col animate-fade-in",
      h1 { class: "text-2xl text-gray-800 dark:text-gray-50 font-extrabold",
        "Welcome to the Maestro Demo App"
      }
      p { class: "mt-4 text-lg text-gray-500 dark:text-gray-400",
        "Explore the capabilities of our components and utilities."
      }
      div { class: "mt-8 grid gap-8 grid-cols-1 sm:grid-cols-2 lg:grid-cols-3 w-full animate-fade-up",
        {
          Route::iter().filter(|route| route.name() != "Home").map(|route| {
            rsx! {
              FeatureCard {
                title: route.name(),
                description: route.description(),
                route: route.clone(),
              }
            }
          })
        }

        FeatureCard {
          title: "Next...",
          description: "Currently we are working on development of new amazing utilities for Dioxus and always open for any suggestions and requests :)",
        }
      }
    }
  }
}

#[component]
pub fn FeatureCard(title: &'static str, description: String, route: Option<Route>) -> Element {
  rsx! {
    div { class: "p-6 bg-white dark:bg-gray-800 rounded-lg shadow-lg transition-all transform hover:scale-105 hover:shadow-2xl animate-fade-up",
      h2 { class: "text-xl font-bold text-gray-900 dark:text-gray-50", "{title}" }
      p { class: "mt-2 text-gray-600 dark:text-gray-300", "{description}" }
      if let Some(route) = route {
        Link { to: route,
          Button {
            class: "mt-4 inline-block bg-blue-600 text-white rounded-lg hover:bg-blue-700 transition",
            variant: ButtonVariant::Default,
            r#type: "button",
            "Explore"
          }
        }
      }
    }
  }
}

