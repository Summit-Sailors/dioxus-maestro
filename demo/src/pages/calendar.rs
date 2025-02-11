use {
  crate::components::ui::component_section::ComponentSection, 
  chrono::{Datelike, Local, NaiveDate}, dioxus::prelude::*, 
  maestro_ui::{
    button::{Button, ButtonVariant},
    calendar::{
      Calendar, CalendarDisplayProps, CalendarSelectProps, Event
    },
  }
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

  let events = use_signal(|| vec![
    Event {
      date: NaiveDate::from_ymd_opt(today.year(), today.month(), 15).unwrap(),
      title: "Team Meeting".to_string(),
      color: Some("#3B82F6".to_string()),
    },
    Event {
      date: NaiveDate::from_ymd_opt(today.year(), today.month(), 22).unwrap(),
      title: "Product Launch".to_string(),
      color: Some("#10B981".to_string()),
    }
  ]);

    rsx! {
      div {
        h1 { 
          class: "text-2xl sm:text-3xl md:text-4xl font-bold text-gray-800 mb-8 text-center", 
          "Maestro UI Calendar Demos" 
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
                .build()
            }
          }
      
          div {
            class: "p-2 space-y-2 space-x-1",
            Button {
              variant: ButtonVariant::Outline,
              class: "px-4 py-2 border text-gray-500 rounded-lg shadow-sm hover:shadow-md transition hover:bg-gray-200",
              on_click: move |_| {
                basic_display_month.set(basic_display_month().prev());
              },
              "Prev Month"
            }
            Button {
              variant: ButtonVariant::Outline,
              class: "px-4 py-2 border text-gray-500 rounded-lg shadow-sm hover:shadow-md transition hover:bg-gray-200",
              on_click: move |_| {
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
            class: "flex justify-center",
            Calendar {
              display_props: CalendarDisplayProps::builder()
                .is_full(false)
                .build(),
              select_props: CalendarSelectProps::builder().build()
            }
          }
        }
        
        ComponentSection {
          title: "Date Range Calendar",
          description: "Calendar with min and max date constraints",
          
          div {
            class: "flex flex-col items-center",
            Calendar {
              display_props: CalendarDisplayProps::builder().build(),
              select_props: CalendarSelectProps::builder()
                .min_date(use_signal(|| Some(min_date)))
                .max_date(use_signal(|| Some(max_date)))
                .build()
            }
            
            div { 
              class: "mt-4 text-sm sm:text:xs text-gray-600 text-center",
              {format!("Allowed date range: {} to {}", 
                min_date.format("%Y-%m-%d"), 
                max_date.format("%Y-%m-%d"))}
            }
          }
        }
        
        ComponentSection {
          title: "Event-Enabled Calendar",
          description: "Calendar with custom events demonstration",
          
          div {
            class: "flex flex-col items-center w-full",
            Calendar {
              display_props: CalendarDisplayProps::builder()
                .events(events)
                .build(),
              select_props: CalendarSelectProps::builder().build()
            }
      
            div { 
              class: "mt-4 space-y-2 text-center w-full",
              h3 { 
                class: "text-lg font-semibold text-gray-700", 
                "Upcoming Events:" 
              }
              {events.read().iter().map(|event| {
                rsx! {
                  div { 
                    class: "flex flex-wrap items-center text-center space-x-2 text-gray-800 justify-center",
                    div { 
                      class: "w-2 h-2 rounded-full", 
                      style: format!("background-color: {}", event.color.clone().unwrap_or_default())
                    }
                    span { 
                      {format!("{} - {}", 
                        event.date.format("%b %d"), 
                        event.title)}
                    }
                  }
                }
              })}
            }
          }
        }
      }
  }
}  
