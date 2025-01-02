use {
	crate::button::Button,
	chrono::{Datelike, NaiveDate, Utc},
	dioxus::prelude::*,
	dioxus_free_icons::{
		icons::ld_icons::{LdCalendar, LdChevronLeft, LdChevronRight, LdChevronsLeft, LdChevronsRight},
		Icon,
	},
	tailwind_fuse::*,
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
	#[props(default = true)]
	is_full: bool,
	button_label: Option<String>,
}

// Calendar may be styled via custom styles: for this provided classNames "calendar" and "calendar-*": add in input.css appropriate tailwind classes
// May be big or small (open by button click)

#[component]
pub fn Calendar(props: CalendarProps) -> Element {
	let mut is_open = use_context_provider(|| Signal::new(false));
	let selected_date = use_context_provider::<Signal<Option<NaiveDate>>>(|| Signal::new(props.value));

	if props.is_full {
		rsx! {
      CalendarComponent { ..props }
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
          {
              if selected_date().is_some() {
                  selected_date().unwrap().format("%b %d, %y").to_string()
              } else {
                  props.button_label.clone().unwrap_or("Select date".to_string())
              }
          }
        }
        div {
          class: tw_join!(
              "absolute top-10 mt-6 right-0 min-w-[448px]", if is_open() { "block" } else {
              "hidden" }
          ),
          CalendarComponent { ..props }
        }
      }
    }
	}
}

#[component]
pub fn CalendarComponent(props: CalendarProps) -> Element {
	let mut current_date = use_signal(|| props.initial_date.unwrap_or(Utc::now().naive_local().date()));
	let mut selected_date = use_context::<Signal<Option<NaiveDate>>>();
	let mut hover_date = use_signal(|| None::<NaiveDate>);
	let mut is_open = use_context::<Signal<bool>>();

	let mut change_month = move |delta: i32| {
		let new_date: NaiveDate;
		if current_date().month() == 1 && delta == -1 {
			new_date = current_date().with_month(12).and_then(|date| date.with_year(date.year() - 1)).unwrap();
		} else if current_date().month() == 12 && delta == 1 {
			new_date = current_date().with_month(1).and_then(|date| date.with_year(date.year() + 1)).unwrap();
		} else {
			new_date = current_date().with_month((current_date().month() as i32 + delta) as u32).unwrap();
		}
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
			is_open.set(false);
			return;
		}
		selected_date.set(Some(date));
		is_open.set(false);
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
    div { class: "w-full max-w-md mx-auto bg-white shadow-lg rounded-lg overflow-hidden calendar",
      div { class: "px-4 py-2 bg-gray-50 flex items-center justify-between calendar-header",
        div {
          button {
            onclick: move |_| change_year(-1),
            class: "p-1 rounded-full hover:bg-gray-200 mr-1",
            Icon {
              class: "h-6 w-6 text-gray-600",
              icon: LdChevronsLeft,
            }
          }
          button {
            onclick: move |_| change_month(-1),
            class: "p-1 rounded-full hover:bg-gray-200",
            Icon {
              class: "h-6 w-6 text-gray-600",
              icon: LdChevronLeft,
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
              icon: LdChevronRight,
            }
          }
          button {
            onclick: move |_| change_year(1),
            class: "p-1 rounded-full hover:bg-gray-200 ml-1",
            Icon {
              class: "h-6 w-6 text-gray-600",
              icon: LdChevronsRight,
            }
          }
        }
      }
      div { class: "p-4 calendar-days",
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
        div { class: "grid grid-cols-7 gap-2 calendar-grid",
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
                              class: "h-10 w-10 rounded-full flex items-center justify-center text-sm text-gray-900 calendar-day",
                              class: if is_today(date) { "bg-blue-500 text-white calendar-day__today" } else { "" },
                              class: if is_selected(date) { "bg-blue-200 calendar-day__selected" } else { "" },
                              class: if is_disabled(date) { "text-gray-400 cursor-not-allowed calendar-day__disabled" } else { "" },
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
                    div { class: "px-4 py-2 bg-gray-50 border-t calendar-footer",
                      p { class: "text-sm text-gray-600", "Selected: {date.format(\"%Y-%m-%d\").to_string()}" }
                    }
                  }
              })
      }
    }
  }
}
