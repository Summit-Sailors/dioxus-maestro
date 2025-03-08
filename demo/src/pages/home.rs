use {crate::router::Route, dioxus::prelude::*, maestro_ui::button::Button, strum::IntoEnumIterator};

#[component]
pub fn HomePage() -> Element {
	rsx! {
		div { id: "home-page", class: "overflow-y-auto",
			div { class: "container flex flex-col animate-fade-in py-7 sm:py-10 lg:py-16",
				h1 { class: "2xl:text-5xl lg:text-4xl text-2xl sm:text-3xl font-semibold text-center",
					"Welcome to the Maestro Demo App"
				}
				p { class: "mt-3 text-base lg:text-xl 2xl:text-2xl text-center",
					"Explore the capabilities of our components and utilities."
				}
				div {
					id: "home-page-feature-card",
					class: "mt-8 sm:mt-10 lg:mt-16 grid gap-6 sm:gap-8 grid-cols-1 lg:grid-cols-2 2xl:grid-cols-3 animate-fade-up",
					{
							Route::iter()
									.filter(|route| route.name() != "Home")
									.map(|route| {
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
		div { class: "py-6 px-5 bg-slate-800/70 rounded transition-all flex flex-col gap-4 border border-slate-700 hover:border-slate-600 items-center",
			h2 { class: "lg:text-2xl text-xl font-medium", "{title}" }
			p { class: "text-slate-300 xl:text-lg text-base mb-6 text-center", "{description}" }
			if let Some(route) = route {
				Link {
					to: route,
					class: "mt-auto ring-0 ring-offset-0 focus-visible:outline-none",
					Button {
						class: "w-fit text-lg bg-indigo-600 text-slate-100 hover:bg-indigo-700 ring-indigo-700 ring-offset-slate-800/70 focus:outline-none",
						// variant: ButtonVariant::Default,
						// size: ButtonSize::Lg,
						r#type: "button",
						"Explore"
					}
				}
			}
		}
	}
}
