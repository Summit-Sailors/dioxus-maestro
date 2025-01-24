use {
  crate::DemoView,
  dioxus::prelude::*, maestro_ui::button::{Button, ButtonType, ButtonVariant}
};

#[component]
pub fn HomePage(current_view: Signal<DemoView>) -> Element {
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
          view: DemoView::Forms,
          current_view
        }
        FeatureCard {
          title: "Hooks",
          description: "Explore custom hooks for your components.",
          view: DemoView::Hooks,
          current_view
        }
        FeatureCard {
          title: "Charts",
          description: "Visualize data with interactive charts.",
          view: DemoView::Plotters,
          current_view
        }
        FeatureCard {
          title: "Query",
          description: "Discover powerful query management.",
          view: DemoView::Query,
          current_view
        }
        FeatureCard {
          title: "Radio",
          description: "Implement radio button components.",
          view: DemoView::Radio,
          current_view
        }
        FeatureCard {
          title: "Toast",
          description: "Show dynamic toast notifications.",
          view: DemoView::Toast,
          current_view
        }
        FeatureCard {
          title: "UI Components",
          description: "Explore reusable UI components.",
          view: DemoView::Ui,
          current_view
        }
      }
    }
  }
}

#[component]
pub fn FeatureCard(title: &'static str, description: &'static str, view: DemoView, current_view: Signal<DemoView>) -> Element {
  rsx! {
    div { class: "p-6 bg-white rounded-lg shadow-md hover:shadow-lg transition",
      h2 { class: "text-xl font-bold text-gray-900",
        title
      }
      p { class: "mt-2 text-gray-600",
        {description}
      }
      Button {
        class: "mt-4 inline-block px-4 py-2 bg-blue-500 text-white rounded hover:bg-blue-600",
        variant: ButtonVariant::Default,
        button_type: ButtonType::Button,
        on_click: move |_| current_view.set(view),
        "Explore"
      }
    }
  }
}
