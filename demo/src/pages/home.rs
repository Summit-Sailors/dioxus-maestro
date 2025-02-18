use {
  crate::router::Route,
  dioxus::prelude::*, 
  maestro_ui::button::{Button, ButtonType, ButtonVariant}, 
  strum::IntoEnumIterator
};

#[component]
pub fn HomePage() -> Element {
  rsx! {
    div { class: "text-center py-16",
      h1 { class: "text-3xl text-white font-extrabold",
        "Welcome to the Maestro Demo App"
      }
      p { class: "mt-4 text-lg text-gray-500",
        "Explore the capabilities of our components and utilities."
      }
      div { class: "mt-8 grid gap-8 grid-cols-1 sm:grid-cols-2 lg:grid-cols-3",
        {Route::iter().map(|route| {
          rsx! {
            FeatureCard {
              title: route.name(),
              description: format!("Learn more about {}", route.name()),
              route: route.clone()
            }
          }
        })}
      }
    }
  }
}

#[component]
pub fn FeatureCard(title: &'static str, description: String, route: Route) -> Element {
  rsx! {
    div { class: "p-6 bg-white rounded-lg shadow-lg hover:shadow-2xl transition",
      h2 { class: "text-xl font-bold text-gray-900",
        "{title}"
      }
      p { class: "mt-2 text-gray-600",
        "{description}"
      }
      Link {
        to: route,
        Button {
          class: "mt-4 inline-block px-4 py-2 bg-blue-600 text-white rounded-lg hover:bg-blue-700 transition",
          variant: ButtonVariant::Default,
          button_type: ButtonType::Button,
          "Explore"
        }
      }
    }
  }
}
