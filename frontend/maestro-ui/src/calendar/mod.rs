use {
	crate::button::Button,
	chrono::{Datelike, NaiveDate},
	dioxus::prelude::*,
	dioxus_free_icons::{
		Icon,
		icons::ld_icons::{LdCalendar, LdChevronLeft, LdChevronRight, LdChevronsLeft, LdChevronsRight},
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
	#[builder(default = "")]
	wrapper_class: &'static str,
	#[builder(default = "")]
	container_class: &'static str,
	#[builder(default = "")]
	header_class: &'static str,
	#[builder(default = "")]
	footer_class: &'static str,
	#[builder(default = "")]
	month_toggle_button_class: &'static str,
	#[builder(default = "")]
	body_class: &'static str,
	#[builder(default = "")]
	grid_class: &'static str,
	#[builder(default = "")]
	days_class: &'static str,
	#[builder(default = "")]
	day_class: &'static str,
	#[builder(default = "")]
	day_today_class: &'static str,
	#[builder(default = "")]
	day_disabled_class: &'static str,
	#[builder(default = "")]
	day_selected_class: &'static str,
	#[builder(default = "")]
	hover_day_class: &'static str,
	#[builder(default = "")]
	events_class: &'static str,
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

// To do: make custom classes String

#[component]
pub fn Calendar(display_props: CalendarDisplayProps, select_props: CalendarSelectProps) -> Element {
	let CalendarDisplayProps {
		mut display_month,
		mut display_year,
		is_full,
		wrapper_class,
		container_class,
		header_class,
		footer_class,
		month_toggle_button_class,
		body_class,
		grid_class,
		days_class,
		..
	} = display_props;
	let CalendarSelectProps { selected_day, selected_month, selected_year, .. } = select_props;

	let hover_date = use_signal(|| None::<NaiveDate>);
	let selected_date = use_memo(move || NaiveDate::from_ymd_opt(selected_year(), selected_month() as u32, selected_day() as u32).unwrap());
	let state_props = CalendarStateProps { hover_date, selected_date };

	let first_day = display_month().first_day(display_year());
	let empty_cells = first_day.days_from_sunday();
	let days_in_month = display_month().num_days(display_year());

	rsx! {
    CalendarMaybeWrapper { is_full, selected_date, wrapper_class,
      div {
        class: tw_merge!(
            "w-full max-w-md mx-auto bg-white shadow-lg rounded-lg overflow-hidden maestro-calendar-container",
            container_class
        ),
        div {
          class: tw_merge!(
              "px-4 py-2 bg-gray-50 flex items-center justify-between maestro-calendar-header",
              header_class
          ),
          div {
            button {
              onclick: move |_| display_year -= 1,
              class: tw_merge!(
                  "p-1 rounded-full hover:bg-gray-200 mr-1 text-gray-600",
                  month_toggle_button_class
              ),
              Icon { class: "h-6 w-6", icon: LdChevronsLeft }
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
              class: tw_merge!(
                  "p-1 rounded-full hover:bg-gray-200 mr-1 text-gray-600",
                  month_toggle_button_class
              ),
              Icon { class: "h-6 w-6", icon: LdChevronLeft }
            }
          }
          h2 { class: tw_merge!("text-lg font-semibold text-gray-800", header_class),
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
              class: tw_merge!(
                  "p-1 rounded-full hover:bg-gray-200 mr-1 text-gray-600",
                  month_toggle_button_class
              ),
              Icon { class: "h-6 w-6", icon: LdChevronRight }
            }
            button {
              onclick: move |_| display_year += 1,
              class: tw_merge!(
                  "p-1 rounded-full hover:bg-gray-200 mr-1 text-gray-600",
                  month_toggle_button_class
              ),
              Icon { class: "h-6 w-6", icon: LdChevronsRight }
            }
          }
        }
        div { class: tw_merge!("p-4 maestro-calendar-body", body_class),
          div { class: tw_merge!("grid grid-cols-7 gap-2 mb-2 maestro-calendar-days", days_class),
            for day in ECalendarDay::iter() {
              div { class: "text-center text-sm font-medium text-gray-600",
                "{day}"
              }
            }
          }
          div { class: tw_merge!("grid grid-cols-7 gap-2 maestro-calendar-grid", grid_class),
            if empty_cells < 6 {
              for _ in 0..=empty_cells {
                div {
                  class: "h-10 w-10 rounded-full flex items-center justify-center text-sm text-gray-900 maestro-calendar-day",
                  class: "text-gray-400 cursor-not-allowed maestro-calendar-day__disabled",
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
        div { class: tw_merge!("px-4 py-2 bg-gray-50 border-t calendar-footer", footer_class),
          p { r#"Selected: {selected_date().format("%Y-%m-%d").to_string()}"# }
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
	wrapper_class: &'static str,
}

pub fn CalendarMaybeWrapper(CalendarMaybeWrapperProps { is_full, children, selected_date, wrapper_class }: CalendarMaybeWrapperProps) -> Element {
	let mut is_open = use_signal(|| false);
	if is_full {
		rsx! {
      {children}
    }
	} else {
		rsx! {
      div { class: tw_merge!("relative flex justify-center maestro-calendar-wrapper", wrapper_class),
        Button {
          class: "maestro-calendar-button",
          onclick: move |_| is_open.toggle(),
          Icon { class: "h-4 w-4", icon: LdCalendar }
          {selected_date().format("%b %d, %y").to_string()}
        }
        div {
          class: tw_merge!(
              "absolute min-w-[448px] top-20 mt-6 z-50 bg-white shadow-lg rounded-lg border maestro-calendar-wrapper-container",
              if is_open() { "block" } else { "hidden" }
          ),
          {children}
        }
      }
    }
	}
}

#[component]
pub fn CalendarDayComponent(delta: u8, display_props: CalendarDisplayProps, select_props: CalendarSelectProps, state_props: CalendarStateProps) -> Element {
	let CalendarDisplayProps {
		display_month,
		display_year,
		events,
		day_class,
		day_selected_class,
		day_today_class,
		day_disabled_class,
		hover_day_class,
		events_class,
		..
	} = display_props;

	let CalendarSelectProps { max_date, min_date, mut selected_day, mut selected_month, mut selected_year } = select_props;
	let CalendarStateProps { mut hover_date, .. } = state_props;
	let selected_date = use_memo(move || NaiveDate::from_ymd_opt(selected_year(), selected_month() as u32, selected_day() as u32).unwrap());

	let this_display_date = NaiveDate::from_ymd_opt(display_year(), display_month() as u32, delta.into());
	if this_display_date.is_none() {
		return rsx! {
      div {
        class: tw_merge!(
            "h-10 w-10 rounded-full flex items-center justify-center text-sm text-gray-900 maestro-calendar-day",
            day_class
        ),
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
      key: delta,
      class: tw_merge!(
          "h-10 w-10 rounded-full flex items-center justify-center text-sm text-gray-900 maestro-calendar-day relative",
          day_class, is_today
          .then_some(tw_merge!("bg-blue-500 text-white maestro-calendar-day__today", &
          day_today_class)), (selected_date() == this_display_date)
          .then_some(tw_merge!("bg-blue-200 maestro-calendar-day__selected", &
          day_selected_class)), is_disabled
          .then_some(tw_merge!("text-gray-500 cursor-not-allowed maestro-calendar-day__disabled",
          day_disabled_class)), (! is_today && selected_date() != this_display_date && !
          is_disabled).then_some(tw_merge!("hover:bg-gray-100", hover_day_class))
      ),
      "{delta}"
      if !curr_date_events.is_empty() {
        span {
          class: tw_merge!(
              "absolute -bottom-0.5 left-0 right-0 mx-auto flex gap-0.5 items-center w-fit maestro-calendar-events",
              events_class
          ),
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
