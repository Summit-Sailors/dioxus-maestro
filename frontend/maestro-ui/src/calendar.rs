use {
	chrono::{Datelike, NaiveDate, Utc},
	dioxus::prelude::*,
	dioxus_free_icons::{
		icons::fa_solid_icons::{FaChevronLeft, FaChevronRight},
		Icon,
	},
};

const DAYS: [&str; 7] = ["Sun", "Mon", "Tue", "Wed", "Thu", "Fri", "Sat"];
const MONTHS: [&str; 12] = ["January", "February", "March", "April", "May", "June", "July", "August", "September", "October", "November", "December"];

#[derive(Clone, PartialEq)]
struct Event {
	date: NaiveDate,
	title: String,
	color: Option<String>,
}

#[derive(Clone, Props, PartialEq)]
pub struct CalendarProps {
	initial_date: Option<NaiveDate>,
	value: Option<NaiveDate>,
	onchange: Option<EventHandler<NaiveDate>>,
	onmonth_change: Option<EventHandler<NaiveDate>>,
	onyear_change: Option<EventHandler<i32>>,
	onhover_date: Option<EventHandler<Option<NaiveDate>>>,
	min_date: Option<NaiveDate>,
	max_date: Option<NaiveDate>,
	mode: Option<String>,
	events: Option<Vec<Event>>,
	week_starts_on: Option<u32>,
	show_week_numbers: Option<bool>,
	locale: Option<String>,
}

#[component]
pub fn Calendar(props: CalendarProps) -> Element {
	let mut current_date = use_signal(|| props.initial_date.unwrap_or(Utc::now().naive_local().date()));
	let mut selected_date = use_signal(|| props.value);
	let mut hover_date = use_signal(|| None::<NaiveDate>);

	let mut change_month = move |delta: i32| {
		let new_date = current_date().with_month(current_date().month() as u32 + delta as u32).unwrap();
		current_date.set(new_date);
		if let Some(handler) = &props.onmonth_change {
			handler.call(new_date);
		}
	};

	let mut change_year = move |delta: i32| {
		let new_date = current_date().with_year(current_date().year() + delta).unwrap();
		current_date.set(new_date);
		if let Some(handler) = &props.onyear_change {
			handler.call(current_date().year());
		}
	};

	let is_today = move |date: NaiveDate| {
		let today = Utc::now().naive_local().date();
		date == today
	};

	let is_selected = move |date: NaiveDate| {
		if let Some(selected) = selected_date() {
			return date == selected;
		}
		false
	};

	let is_disabled = move |date: NaiveDate| {
		if let Some(min_date) = props.min_date {
			if date < min_date {
				return true;
			}
		}
		if let Some(max_date) = props.max_date {
			if date > max_date {
				return true;
			}
		}
		false
	};

	let mut select_date = move |date: NaiveDate| {
		if is_disabled(date) {
			return;
		}
		selected_date.set(Some(date));
		if let Some(handler) = &props.onchange {
			handler.call(date);
		}
	};

	let mut handle_hover = move |date: Option<NaiveDate>| {
		hover_date.set(date);
		if let Some(handler) = &props.onhover_date {
			handler.call(date);
		}
	};

	rsx! {
		div {
			class: "w-full max-w-md mx-auto bg-white shadow-lg rounded-lg overflow-hidden",
			class: "",
			div { class: "px-4 py-2 bg-gray-50 flex items-center justify-between",
				div {
					button {
						onclick: move |_| change_year(-1),
						class: "p-1 rounded-full hover:bg-gray-200 mr-1",
						Icon {
							class: "h-4 w-4 text-gray-600",
							icon: FaChevronLeft,
						}
						Icon {
							class: "h-4 w-4 text-gray-600 -ml-2",
							icon: FaChevronLeft,
						}
					}
					button {
						onclick: move |_| change_month(-1),
						class: "p-1 rounded-full hover:bg-gray-200",
						Icon {
							class: "h-6 w-6 text-gray-600",
							icon: FaChevronLeft,
						}
					}
				}
				h2 { class: "text-lg font-semibold text-gray-800",
					"{MONTHS[current_date().month() as usize - 1]} {current_date().year()}"
				}
				div {
					button {
						onclick: move |_| change_month(1),
						class: "p-1 rounded-full hover:bg-gray-200",
						Icon {
							class: "h-6 w-6 text-gray-600",
							icon: FaChevronRight,
						}
					}
					button {
						onclick: move |_| change_year(1),
						class: "p-1 rounded-full hover:bg-gray-200 ml-1",
						Icon {
							class: "h-4 w-4 text-gray-600",
							icon: FaChevronRight,
						}
						Icon {
							class: "h-4 w-4 text-gray-600 -ml-2",
							icon: FaChevronRight,
						}
					}
				}
			}
			div { class: "p-4",
				div { class: "grid grid-cols-7 gap-2 mb-2",
					{
							DAYS.iter()
									.map(|day| {
											rsx! {
												div { class: "text-center text-sm font-medium text-gray-600", "{day}" }
											}
									})
					}
				}
				div { class: "grid grid-cols-7 gap-2",
					{
							(1..=31)
									.map(|day| {
											let date = NaiveDate::from_ymd_opt(
													current_date().year(),
													current_date().month(),
													day,
											);
											if let Some(date) = date {
													rsx! {
														button {
															onclick: move |_| select_date(date),
															onmouseenter: move |_| handle_hover(Some(date)),
															onmouseleave: move |_| handle_hover(None),
															disabled: is_disabled(date),
															class: "h-10 w-10 rounded-full flex items-center justify-center text-sm",
															class: if is_today(date) { "bg-blue-500 text-white" } else { "" },
															class: if is_selected(date) { "bg-blue-200" } else { "" },
															class: if is_disabled(date) { "text-gray-400 cursor-not-allowed" } else { "" },
															class: if !is_today(date) && !is_selected(date) && !is_disabled(date) { "hover:bg-gray-100" } else { "" },
															"{day}"
														}
													}
											} else {
													rsx! {
														div { class: "h-10 w-10" }
													}
											}
									})
					}
				}
			}
			{
					selected_date()
							.map(|date| {
									rsx! {
										div { class: "px-4 py-2 bg-gray-50 border-t",
											p { class: "text-sm text-gray-600", "Selected: {date.format(\"%Y-%m-%d\").to_string()}" }
										}
									}
							})
			}
		}
	}
}
