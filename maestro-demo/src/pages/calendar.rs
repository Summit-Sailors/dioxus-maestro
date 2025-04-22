use chrono::{Datelike, Local, NaiveDate};
use dioxus::prelude::*;
use maestro_ui::{
	button::Button,
	calendar::{Calendar, CalendarDisplayProps, CalendarSelectProps, Event},
};

use crate::components::ui::{component_section::ComponentSection, features::Features};

#[component]
pub fn CalendarDemo() -> Element {
	let mut basic_display_month = use_signal(|| Local::now().date_naive().month().into());
	let basic_display_year = use_signal(|| Local::now().date_naive().year());
	let basic_selected_day = use_signal(|| (Local::now().date_naive().day() as u8));
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
			id: "maestro-calendar",
			class: "p-4 text-[color:var(--text-color)] bg-[color:var(--bg-color)] rounded-lg shadow-[var(--shadow-lg)]",
			div { class: "mb-8",
				h1 { class: "text-[color:var(--text-color)] text-center text-3xl font-bold mb-2",
					"Maestro Calendar"
				}
				p { class: "text-[color:var(--muted-text)] text-center",
					"A type-safe, and highly customizable calendar implementation that goes beyond traditional calendar components. Designed with Rust's type system and Dioxus's reactive paradigms, it offers unparalleled flexibility and developer experience."
				}
			}
			div { id: "calendar-features", class: "flex space-x-2 mb-4",
				Features {
					title: "Features".to_string(),
					features: vec![
							"Compile-Time Safety: Leverages Rust's enum system to prevent invalid calendar operations"
									.to_string(),
							"Built-in Methods: Rich set of methods like is_weekend(), next(), prev()"
									.to_string(),
							"Serialization Support: Easy conversion and storage with serde derive macros"
									.to_string(),
							"Signal-Based Reactivity: Seamless state updates with Dioxus signals"
									.to_string(),
							"Memoized Computations: Efficient date calculations".to_string(),
							"Dynamic UI Updates: Instant reflection of state changes".to_string(),
							"Full Calendar View: Comprehensive month display".to_string(),
							"Compact Modal View: Minimalist, space-efficient design".to_string(),
							"Date Range Limitations: Easily set minimum and maximum selectable dates"
									.to_string(),
					],
				}
			}

			ComponentSection {
				title: "Full Calendar",
				description: "Fully configurable calendar with display and selection controls",
				div { id: "maestro-full-calendar", class: "overflow-auto",
					Calendar {
						display_props: CalendarDisplayProps::builder()
								.display_month(basic_display_month)
								.display_year(basic_display_year)
								.is_full(true)
								.container_class(
										"bg-[color:var(--card-bg)] border border-[color:var(--border-color)] shadow-[var(--shadow-xl)]",
								)
								.header_class(
										"bg-[color:var(--input-bg)] border-b border-[color:var(--border-color)] text-[color:var(--text-color)]",
								)
								.footer_class(
										"bg-[color:var(--input-bg)] border-t border-[color:var(--border-color)] text-[color:var(--text-color)]",
								)
								.month_toggle_button_class(
										"hover:bg-[color:var(--hover-bg)] text-[color:var(--text-color)]",
								)
								.body_class("bg-[color:var(--card-bg)]")
								.day_class("text-[color:var(--text-color)] hover:bg-[color:var(--hover-bg)]")
								.day_today_class("bg-[color:var(--primary-bg)] text-[color:var(--primary-text)]")
								.day_disabled_class("text-[color:var(--muted-text)] hover:bg-transparent")
								.day_selected_class(
										"bg-[color:var(--secondary-bg)] text-[color:var(--secondary-text)]",
								)
								.hover_day_class("hover:bg-[color:var(--hover-bg)]")
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

				div {
					id: "full-calendar-control-buttons",
					class: "p-2 space-y-2 space-x-1",
					Button {
						class: "px-4 py-2 border text-[var(--primary-text)] bg-[var(--primary-bg)] border-[var(--border-color)] rounded-[var(--radius-lg)] shadow-[var(--shadow-sm)] hover:shadow-[var(--shadow-md)] transition",
						onclick: move |_| {
								basic_display_month.set(basic_display_month().prev());
						},
						"Prev Month"
					}
					Button {
						class: "px-4 py-2 border text-[var(--primary-text)] bg-[var(--primary-bg)] border-[var(--border-color)] rounded-[var(--radius-lg)] shadow-[var(--shadow-sm)] hover:shadow-[var(--shadow-md)] transition",
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

				div {
					id: "maestro-compact-calendar",
					class: "grid justify-center text-slate-200",
					Calendar {
						display_props: CalendarDisplayProps::builder()
								.container_class(
										"bg-[color:var(--card-bg)] border border-[color:var(--border-color)] shadow-[var(--shadow-xl)]",
								)
								.header_class(
										"bg-[color:var(--input-bg)] border-b border-[color:var(--border-color)] text-[color:var(--text-color)]",
								)
								.footer_class(
										"bg-[color:var(--input-bg)] border-t border-[color:var(--border-color)] text-[color:var(--text-color)]",
								)
								.month_toggle_button_class(
										"hover:bg-[color:var(--hover-bg)] text-[color:var(--text-color)]",
								)
								.body_class("bg-[color:var(--card-bg)]")
								.day_class("text-[color:var(--text-color)] hover:bg-[color:var(--hover-bg)]")
								.day_today_class("bg-[color:var(--primary-bg)] text-[color:var(--primary-text)]")
								.day_disabled_class("text-[color:var(--muted-text)] hover:bg-transparent")
								.day_selected_class(
										"bg-[color:var(--secondary-bg)] text-[color:var(--secondary-text)]",
								)
								.hover_day_class("hover:bg-[color:var(--hover-bg)]")
								.is_full(false)
								.build(),
						select_props: CalendarSelectProps::builder().build(),
					}
				}
			}

			ComponentSection {
				title: "Date Range Calendar",
				description: "Calendar with min and max date constraints",

				div {
					id: "maestro-date-range-calendar",
					class: "flex flex-col items-center",
					Calendar {
						display_props: CalendarDisplayProps::builder()
								.container_class(
										"bg-[color:var(--card-bg)] border border-[color:var(--border-color)] shadow-[var(--shadow-xl)]",
								)
								.header_class(
										"bg-[color:var(--input-bg)] border-b border-[color:var(--border-color)] text-[color:var(--text-color)]",
								)
								.footer_class(
										"bg-[color:var(--input-bg)] border-t border-[color:var(--border-color)] text-[color:var(--text-color)]",
								)
								.month_toggle_button_class(
										"hover:bg-[color:var(--hover-bg)] text-[color:var(--text-color)]",
								)
								.body_class("bg-[color:var(--card-bg)]")
								.day_class("text-[color:var(--text-color)] hover:bg-[color:var(--hover-bg)]")
								.day_today_class("bg-[color:var(--primary-bg)] text-[color:var(--primary-text)]")
								.day_disabled_class("text-[color:var(--muted-text)] hover:bg-transparent")
								.day_selected_class(
										"bg-[color:var(--secondary-bg)] text-[color:var(--secondary-text)]",
								)
								.hover_day_class("hover:bg-[color:var(--hover-bg)]")
								.build(),
						select_props: CalendarSelectProps::builder()
								.min_date(use_signal(|| Some(min_date)))
								.max_date(use_signal(|| Some(max_date)))
								.build(),
					}


					div {
						id: "date-range-calendar-range",
						class: "mt-4 text-sm sm:text:xs text-[color:var(--text-color)] text-center",
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

				div {
					id: "maestro-event-enabled-calendar",
					class: "flex flex-col items-center w-full",
					Calendar {
						display_props: CalendarDisplayProps::builder()
								.container_class(
										"bg-[color:var(--card-bg)] border border-[color:var(--border-color)] shadow-[var(--shadow-xl)]",
								)
								.header_class(
										"bg-[color:var(--input-bg)] border-b border-[color:var(--border-color)] text-[color:var(--text-color)]",
								)
								.footer_class(
										"bg-[color:var(--input-bg)] border-t border-[color:var(--border-color)] text-[color:var(--text-color)]",
								)
								.month_toggle_button_class(
										"hover:bg-[color:var(--hover-bg)] text-[color:var(--text-color)]",
								)
								.body_class("bg-[color:var(--card-bg)]")
								.day_class("text-[color:var(--text-color)] hover:bg-[color:var(--hover-bg)]")
								.day_today_class("bg-[color:var(--primary-bg)] text-[color:var(--primary-text)]")
								.day_disabled_class("text-[color:var(--muted-text)] hover:bg-transparent")
								.day_selected_class(
										"bg-[color:var(--secondary-bg)] text-[color:var(--secondary-text)]",
								)
								.hover_day_class("hover:bg-[color:var(--hover-bg)]")
								.events_class("bg-opacity-80")
								.events(events)
								.build(),
						select_props: CalendarSelectProps::builder().build(),
					}

					div {
						id: "event-enabled-calendar-events",
						class: "mt-4 space-y-2 text-[color:var(--text-color)] text-center w-full",
						h3 { class: "text-lg font-semibold text-[color:var(--text-color)]",
							"Upcoming Events:"
						}
						{
								events
										.iter()
										.map(|event| {
												rsx! {
													div { class: "flex flex-wrap items-center text-center space-x-2 text-[color:var(--text-color)] justify-center",
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
