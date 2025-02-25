use {
	crate::components::ui::{component_section::ComponentSection, features::Features},
	chrono::{Datelike, Local, NaiveDate},
	dioxus::prelude::*,
	maestro_ui::{
		button::Button,
		calendar::{Calendar, CalendarDisplayProps, CalendarSelectProps, Event},
	},
};

#[component]
pub fn CalendarDemo() -> Element {
	let mut basic_display_month = use_signal(|| Local::now().date_naive().month().into());
	let basic_display_year = use_signal(|| Local::now().date_naive().year());
	let basic_selected_day = use_signal(|| (Local::now().date_naive().day() as u8).into());
	let basic_selected_month = use_signal(|| Local::now().date_naive().month().into());
	let basic_selected_year = use_signal(|| Local::now().date_naive().year());

	let today = Local::now().date_naive();
	let min_date = NaiveDate::from_ymd_opt(today.year(), today.month(), 1).unwrap();
	let max_date = min_date.checked_add_months(chrono::Months::new(3)).unwrap();

	let events = use_signal(|| {
		vec![
			Event { date: NaiveDate::from_ymd_opt(today.year(), today.month(), 15).unwrap(), title: "Team Meeting".to_string(), color: Some("#3B82F6".to_string()) },
			Event {
				date: NaiveDate::from_ymd_opt(today.year(), today.month(), 22).unwrap(),
				title: "Product Launch".to_string(),
				color: Some("#10B981".to_string()),
			},
		]
	});

	rsx! {
		div {
      class: "p-6 text-gray-100 bg-gray-900 rounded-lg shadow-lg",
			div { 
        class: "mb-8",
				h1 { class: "text-gray-100 text-center text-3xl font-bold mb-2", "Maestro Calendar" }
				p { class: "text-gray-300 text-center",
					"A type-safe, and highly customizable calendar implementation that goes beyond traditional calendar components. Designed with Rust's type system and Dioxus's reactive paradigms, it offers unparalleled flexibility and developer experience."
				}
			}

      div {
        class: "flex space-x-2",  
        Features {
          title: "Calendar".to_string(),
          features: vec![
            "Compile-Time Safety: Leverages Rust's enum system to prevent invalid calendar operations".to_string(),
            "Built-in Methods: Rich set of methods like is_weekend(), next(), prev()".to_string(),
            "Serialization Support: Easy conversion and storage with serde derive macros".to_string(),
            "Signal-Based Reactivity: Seamless state updates with Dioxus signals".to_string(),
            "Memoized Computations: Efficient date calculations".to_string(),
            "Dynamic UI Updates: Instant reflection of state changes".to_string(),
            "Full Calendar View: Comprehensive month display".to_string(),
            "Compact Modal View: Minimalist, space-efficient design".to_string(),
            "Date Range Limitations: Easily set minimum and maximum selectable dates".to_string(),
          ]
        }
      }

			ComponentSection {
				title: "Full Calendar",
				description: "Fully configurable calendar with display and selection controls",

				div { 
          class: "overflow-auto",
					Calendar {
						display_props: CalendarDisplayProps::builder()
              .display_month(basic_display_month)
              .display_year(basic_display_year)
              .is_full(true)
              .build(),
						select_props: CalendarSelectProps::builder()
              .selected_day(basic_selected_day)
              .selected_month(basic_selected_month)
              .selected_year(basic_selected_year)
              .min_date(use_signal(|| Some(min_date)))
              .max_date(use_signal(|| Some(max_date)))
              .build(),
					}
				}

				div { class: "p-2 space-y-2 space-x-1",
					Button {
						class: "px-4 py-2 border text-gray-400 border-gray-800 rounded-lg shadow-sm hover:shadow-md transition",
						onclick: move |_| {
								basic_display_month.set(basic_display_month().prev());
						},
						"Prev Month"
					}
					Button {
						class: "px-4 py-2 border text-gray-400 border-gray-800 rounded-lg shadow-sm hover:shadow-md transition",
						onclick: move |_| {
								basic_display_month.set(basic_display_month().next());
						},
						"Next Month"
					}
				}
			}

			ComponentSection {
				title: "Compact Calendar",
				description: "Space-efficient calendar with button trigger",

				div { class: "grid justify-center text-gray-200",
					Calendar {
						display_props: CalendarDisplayProps::builder()
								.is_full(false)
								.build(),
						select_props: CalendarSelectProps::builder().build(),
					}
				}
			}

			ComponentSection {
				title: "Date Range Calendar",
				description: "Calendar with min and max date constraints",

				div { class: "flex flex-col items-center",
					Calendar {
						display_props: CalendarDisplayProps::builder().build(),
						select_props: CalendarSelectProps::builder()
								.min_date(use_signal(|| Some(min_date)))
								.max_date(use_signal(|| Some(max_date)))
								.build(),
					}

					div { class: "mt-4 text-sm sm:text:xs text-gray-200 text-center",
						{
              format!(
                  "Allowed date range: {} to {}",
                  min_date.format("%Y-%m-%d"),
                  max_date.format("%Y-%m-%d"),
              )
						}
					}
				}
			}

			ComponentSection {
				title: "Event-Enabled Calendar",
				description: "Calendar with custom events demonstration",

				div { class: "flex flex-col items-center w-full",
					Calendar {
						display_props: CalendarDisplayProps::builder().events(events).build(),
						select_props: CalendarSelectProps::builder().build(),
					}

					div { class: "mt-4 space-y-2 text-gray-200 text-center w-full",
						h3 { class: "text-lg font-semibold text-gray-200", "Upcoming Events:" }
						{
              events.read().iter().map(|event| {
                rsx! {
                  div { class: "flex flex-wrap items-center text-center space-x-2 text-gray-100 justify-center",
                    div {
                      class: "w-2 h-2 rounded-full",
                      style: format!("background-color: {}", event.color.clone().unwrap_or_default()),
                    }
                    span { {format!("{} - {}", event.date.format("%b %d"), event.title)} }
                  }
                }
              })
						}
					}
				}
			}
		}
	}
}
