use {
	crate::button::Button,
	chrono::{Datelike, NaiveDate},
	dioxus::prelude::*,
	dioxus_free_icons::{
		icons::ld_icons::{LdCalendar, LdChevronLeft, LdChevronRight, LdChevronsLeft, LdChevronsRight},
		Icon,
	},
	enums::{e_month::ECalendarMonth, e_week_day::ECalendarDay},
	strum::IntoEnumIterator,
	tailwind_fuse::*,
};

pub mod enums;

#[derive(Clone, PartialEq)]
pub struct Event {
	pub date: NaiveDate,
	pub title: String,
	pub color: Option<String>,
}

#[derive(PartialEq, bon::Builder)]
pub struct CalendarDisplayProps {
	#[builder(default = Signal::new(chrono::Local::now().date_naive().month().into()))]
	display_month: Signal<ECalendarMonth>,
	#[builder(default = Signal::new(chrono::Local::now().date_naive().year()))]
	display_year: Signal<i32>,
	#[builder(default)]
	events: Signal<Vec<Event>>,
	#[builder(default = true)]
	is_full: bool,
}

impl Clone for CalendarDisplayProps {
	fn clone(&self) -> Self {
		*self
	}
}

impl Copy for CalendarDisplayProps {}

#[derive(PartialEq, bon::Builder)]
pub struct CalendarSelectProps {
	#[builder(default = Signal::new(chrono::Local::now().date_naive().month().into()))]
	selected_month: Signal<ECalendarMonth>,
	#[builder(default = Signal::new(chrono::Local::now().date_naive().day() as u8))]
	selected_day: Signal<u8>,
	#[builder(default = Signal::new(chrono::Local::now().date_naive().year()))]
	selected_year: Signal<i32>,
	#[builder(default = Signal::new(None))]
	min_date: Signal<Option<NaiveDate>>,
	#[builder(default = Signal::new(None))]
	max_date: Signal<Option<NaiveDate>>,
}

impl Clone for CalendarSelectProps {
	fn clone(&self) -> Self {
		*self
	}
}

impl Copy for CalendarSelectProps {}

#[derive(PartialEq)]
pub struct CalendarStateProps {
	selected_date: Memo<NaiveDate>,
	hover_date: Signal<Option<NaiveDate>>,
}

impl Clone for CalendarStateProps {
	fn clone(&self) -> Self {
		*self
	}
}

impl Copy for CalendarStateProps {}

pub fn use_calendar() {}

#[component]
pub fn Calendar(display_props: CalendarDisplayProps, select_props: CalendarSelectProps) -> Element {
	let CalendarDisplayProps { mut display_month, mut display_year, is_full, .. } = display_props;
	let CalendarSelectProps { selected_day, selected_month, selected_year, .. } = select_props;

	let hover_date = use_signal(|| None::<NaiveDate>);
	let selected_date = use_memo(move || NaiveDate::from_ymd_opt(selected_year(), selected_month() as u32, selected_day() as u32).unwrap());
	let state_props = CalendarStateProps { hover_date, selected_date };

	let first_day = display_month().first_day(display_year());
	let empty_cells = first_day.days_from_sunday();
	let days_in_month = display_month().num_days(display_year());

	rsx! {
		CalendarMaybeWrapper { is_full, selected_date,
			div { class: "w-full max-w-md mx-auto bg-white shadow-lg rounded-lg overflow-hidden calendar",
				div { class: "px-4 py-2 bg-gray-50 flex items-center justify-between calendar-header",
					div {
						button {
							onclick: move |_| display_year -= 1,
							class: "p-1 rounded-full hover:bg-gray-200 mr-1",
							Icon {
								class: "h-6 w-6 text-gray-600",
								icon: LdChevronsLeft,
							}
						}
						button {
							onclick: move |_| {
									let prev_display_month = display_month() as u8;
									display_month -= 1_u8.into();
									if prev_display_month == 1 && display_month() as u8 == 12 {
											display_year -= 1;
									} else if prev_display_month == 12 && display_month() as u8 == 1 {
											display_year += 1;
									}
							},
							class: "p-1 rounded-full hover:bg-gray-200",
							Icon {
								class: "h-6 w-6 text-gray-600",
								icon: LdChevronLeft,
							}
						}
					}
					h2 { class: "text-lg font-semibold text-gray-800",
						"{display_month()} {display_year()}"
					}
					div {
						button {
							onclick: move |_| {
									let prev_display_month = display_month() as u8;
									display_month += 1_u8.into();
									if prev_display_month == 1 && display_month() as u8 == 12 {
											display_year -= 1;
									} else if prev_display_month == 12 && display_month() as u8 == 1 {
											display_year += 1;
									}
							},
							class: "p-1 rounded-full hover:bg-gray-200",
							Icon {
								class: "h-6 w-6 text-gray-600",
								icon: LdChevronRight,
							}
						}
						button {
							onclick: move |_| display_year += 1,
							class: "p-1 rounded-full hover:bg-gray-200 ml-1",
							Icon {
								class: "h-6 w-6 text-gray-600",
								icon: LdChevronsRight,
							}
						}
					}
				}
				div { class: "p-4 calendar-body",
					div { class: "grid grid-cols-7 gap-2 mb-2 calendar-days",
						for day in ECalendarDay::iter() {
							div { class: "text-center text-sm font-medium text-gray-600",
								"{day}"
							}
						}
					}
					div { class: "grid grid-cols-7 gap-2 calendar-grid",
						if empty_cells < 6 {
							for _ in 0..=empty_cells {
								div {
									class: "h-10 w-10 rounded-full flex items-center justify-center text-sm text-gray-900 calendar-day",
									class: "text-gray-400 cursor-not-allowed calendar-day__disabled",
								}
							}
						}
						for delta in (1..=days_in_month) {
							CalendarDayComponent {
								delta,
								display_props,
								select_props,
								state_props,
							}
						}
					}
				}
				div { class: "px-4 py-2 bg-gray-50 border-t calendar-footer",
					p { class: "text-sm text-gray-600",
						r#"Selected: {selected_date().format("%Y-%m-%d").to_string()}"#
					}
				}
			}
		}
	}
}

#[derive(Clone, PartialEq, Props)]
pub struct CalendarMaybeWrapperProps {
	is_full: bool,
	selected_date: Memo<NaiveDate>,
	children: Element,
}

pub fn CalendarMaybeWrapper(CalendarMaybeWrapperProps { is_full, children, selected_date }: CalendarMaybeWrapperProps) -> Element {
	let mut is_open = use_signal(|| false);
	if is_full {
		rsx! {
			{children}
		}
	} else {
		rsx! {
			div { class: "relative w-fit",
				Button {
					variant: crate::button::ButtonVariant::Outline,
					class: "calendar-button",
					on_click: move |_| is_open.toggle(),
					stop_propagation: true,
					Icon { class: "h-4 w-4", icon: LdCalendar }
					{selected_date().format("%b %d, %y").to_string()}
				}
				div {
					class: tw_join!(
							"absolute top-10 mt-6 right-0 min-w-[448px]", if is_open() { "block" } else {
							"hidden" }
					),
					{children}
				}
			}
		}
	}
}

#[component]
pub fn CalendarDayComponent(delta: u8, display_props: CalendarDisplayProps, select_props: CalendarSelectProps, state_props: CalendarStateProps) -> Element {
	let CalendarDisplayProps { display_month, display_year, events, .. } = display_props;
	let CalendarSelectProps { max_date, min_date, mut selected_day, mut selected_month, mut selected_year } = select_props;
	let CalendarStateProps { mut hover_date, .. } = state_props;
	let selected_date = use_memo(move || NaiveDate::from_ymd_opt(selected_year(), selected_month() as u32, selected_day() as u32).unwrap());

	let this_display_date = NaiveDate::from_ymd_opt(display_year(), display_month() as u32, delta.into());
	if this_display_date.is_none() {
		return rsx! {
			div {
				class: "h-10 w-10 rounded-full flex items-center justify-center text-sm text-gray-900 calendar-day",
				class: "text-gray-400 cursor-not-allowed calendar-day__disabled",
				""
			}
		};
	}

	let this_display_date = this_display_date.unwrap();
	let curr_date_events = events.iter().filter(|e| e.date == this_display_date).collect::<Vec<_>>();
	let is_today = this_display_date == chrono::Local::now().date_naive();
	let date_smaller_than_min = min_date().map(|min_date| min_date > this_display_date).unwrap_or(false);
	let date_bigger_than_max = max_date().map(|max_date| max_date < this_display_date).unwrap_or(false);
	let is_disabled = date_smaller_than_min || date_bigger_than_max;
	rsx! {
		button {
			onclick: move |_| {
					selected_day.set(delta);
					selected_month.set(display_month());
					selected_year.set(display_year());
			},
			onmouseenter: move |_| hover_date.set(Some(this_display_date)),
			onmouseleave: move |_| hover_date.set(None),
			disabled: is_disabled,
			class: "h-10 w-10 rounded-full flex items-center justify-center text-sm text-gray-900 calendar-day relative",
			class: if is_today { "bg-blue-500 text-white calendar-day__today" } else { "" },
			class: if selected_date() == this_display_date { "bg-blue-200 calendar-day__selected" } else { "" },
			class: if is_disabled { "text-gray-400 cursor-not-allowed calendar-day__disabled" } else { "" },
			class: if !is_today && selected_date() != this_display_date && !is_disabled { "hover:bg-gray-100" } else { "" },
			"{delta}"
			if !curr_date_events.is_empty() {
				span { class: "absolute -bottom-0.5 left-0 right-0 mx-auto flex gap-0.5 items-center w-fit",
					for e in curr_date_events {
						span {
							class: "flex w-1 h-1 rounded-full",
							style: format!("background-color: {}", e.color.clone().unwrap_or(String::from("#3b82f6"))),
						}
					}
				}
			}
		}
	}
}
