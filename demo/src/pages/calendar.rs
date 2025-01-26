use {
  chrono::{Datelike, Local, NaiveDate}, dioxus::prelude::*, maestro_ui::{
    button::{Button, ButtonVariant},
    calendar::{
      Calendar, CalendarDisplayProps, CalendarSelectProps
    },
  }
};

#[component]
pub fn CalendarDemo() -> Element {
  let mut basic_display_month = use_signal(|| Local::now().date_naive().month().into());
  let mut basic_display_year = use_signal(|| Local::now().date_naive().year());
  let mut basic_selected_day = use_signal(|| (Local::now().date_naive().day() as u8).into());
  let mut basic_selected_month = use_signal(|| Local::now().date_naive().month().into());
  let mut basic_selected_year = use_signal(|| Local::now().date_naive().year());

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
      class: "space-y-8 max-w-4xl mx-auto py-8",
      
      h1 { 
        class: "text-3xl font-bold mb-8", 
        "Maestro UI Calendar Demos" 
      }
      
      ComponentSection {
        title: "Full Calendar",
        description: "Fully configurable calendar with display and selection controls",
        
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
        
        div {
          class: "mt-4 flex space-x-2",
          Button {
            variant: ButtonVariant::Outline,
            on_click: move |_| {
              basic_display_month.set(basic_display_month().prev());
            },
            "Previous Month"
          }
          Button {
            variant: ButtonVariant::Outline,
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
        
        Calendar {
          display_props: CalendarDisplayProps::builder()
            .is_full(false)
            .build(),
          select_props: CalendarSelectProps::builder().build()
        }
      }
      
      ComponentSection {
        title: "Date Range Calendar",
        description: "Calendar with min and max date constraints",
        
        div {
          Calendar {
            display_props: CalendarDisplayProps::builder().build(),
            select_props: CalendarSelectProps::builder()
              .min_date(use_signal(|| Some(min_date)))
              .max_date(use_signal(|| Some(max_date)))
              .build()
          }
          
          div { 
            class: "mt-4 text-sm text-gray-600",
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
          Calendar {
            display_props: CalendarDisplayProps::builder()
              .events(events)
              .build(),
            select_props: CalendarSelectProps::builder().build()
          }
          
          div { 
            class: "mt-4 space-y-2",
            h3 { 
              class: "text-lg font-semibold", 
              "Upcoming Events:" 
            }
            {events.read().iter().map(|event| {
              rsx! {
                div { 
                  class: "flex items-center space-x-2",
                  div { 
                    class: "w-3 h-3 rounded-full", 
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

#[derive(Props, Clone, PartialEq)]
pub struct ComponentSectionProps {
  title: &'static str,
  description: &'static str,
  children: Element,
}

#[component]
fn ComponentSection(props: ComponentSectionProps) -> Element {
  rsx! {
    section { class: "mb-12",
      h2 { class: "text-2xl font-semibold mb-2", {props.title} }
      p { class: "text-gray-600 mb-6", {props.description} }
      div { class: "bg-white p-6 rounded-lg shadow-sm border", {props.children} }
    }
  }
}

// event struct for demonstration
#[derive(Clone, PartialEq)]
struct Event {
  date: NaiveDate,
  title: String,
  color: Option<String>,
}
