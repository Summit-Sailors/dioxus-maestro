use {
  crate::router::Route,
  dioxus::prelude::*, 
  maestro_ui::button::{Button, ButtonType, ButtonVariant}
};

#[component]
pub fn HomePage() -> Element {
  rsx! {
    div { class: "text-center py-16",
      h1 { class: "text-4xl font-bold text-gray-900",
        "Welcome to the Maestro Demo App"
      }
      p { class: "mt-4 text-gray-600",
        "Explore the capabilities of our components and utilities."
      }
      div { class: "mt-8 grid gap-4 grid-cols-1 sm:grid-cols-2 lg:grid-cols-4",
        FeatureCard {
          title: "Forms",
          description: "Learn about form handling with validations.",
          route: Route::FormsDemo {}
        }
        FeatureCard {
          title: "Hooks",
          description: "Explore custom hooks for your components.",
          route: Route::HooksDemo {}
        }
        FeatureCard {
          title: "Charts",
          description: "Visualize data with interactive charts.",
          route: Route::PlottersDemo {}
        }
        FeatureCard {
          title: "Query",
          description: "Discover powerful query management.",
          route: Route::QueryDemo {}
        }
        FeatureCard {
          title: "Radio",
          description: "Implement radio button components.",
          route: Route::RadioDemo {}
        }
        FeatureCard {
          title: "Toast",
          description: "Show dynamic toast notifications.",
          route: Route::ToastDemo {}
        }
        FeatureCard {
          title: "UI Components",
          description: "Explore reusable UI components.",
          route: Route::UIDemo {}
        }
      }
    }
  }
}

#[component]
pub fn FeatureCard(title: &'static str, description: &'static str, route: Route) -> Element {
  rsx! {
    div { class: "p-6 bg-white rounded-lg shadow-md hover:shadow-lg transition",
      h2 { class: "text-xl font-bold text-gray-900",
        {title}
      }
      p { class: "mt-2 text-gray-600",
        {description}
      }
      Link { 
        to: route,
        Button {
          class: "mt-4 inline-block px-4 py-2 bg-blue-500 text-white rounded hover:bg-blue-600",
          variant: ButtonVariant::Default,
          button_type: ButtonType::Button,
          "Explore"
        }
      }
    }
  }
}
