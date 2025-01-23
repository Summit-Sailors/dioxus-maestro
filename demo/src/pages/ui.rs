use {
  dioxus::prelude::*,
  chrono::{Datelike, Local, NaiveDate},
  maestro_ui::{
    button::{Button, ButtonSize, ButtonType, ButtonVariant},
    calendar::{Calendar, CalendarDisplayProps, CalendarSelectProps},
    input::{Input, InputType, InputVariant},
    label::Label,
    multi_select::MultiSelect,
    radio::Radio,
    range::Range,
    select::Select,
    spinner::FreeIconSpinner,
    textarea::Textarea,
    toggle::{EToggleSwitchLabelPlacement, ToggleSwitch, ToggleSwitchLabelStatesProp},
  },

};

#[component]
pub fn UIDemo() -> Element {
  let mut selected_option = use_signal(|| "Option 1".to_string());
  let mut selected_options = use_signal(Vec::<String>::new);
  let toggle_state = use_signal(|| false);
  let mut range_value = use_signal(|| 50);
  let mut text_input = use_signal(String::new);
  let mut text_area_value = use_signal(String::new);
  let mut entered_text = use_signal(String::new);

  rsx! {
    div { class: "max-w-4xl mx-auto py-8 px-4",
      h1 { class: "text-3xl font-bold mb-8", "Maestro UI Components" }
      
      // buttons section
      ComponentSection {
        title: "Buttons",
        description: "Various button styles, sizes, and types with different variants",

        div { class: "grid grid-cols-2 md:grid-cols-3 gap-4",
          // default button
          Button {
            variant: ButtonVariant::Default,
            button_type: ButtonType::Button,
            on_click: move |_| { log::info!("Default Button clicked"); },
            "Default Button"
          }

          // destructive button
          Button {
            variant: ButtonVariant::Destructive,
            button_type: ButtonType::Button,
            on_click: move |_| { log::info!("Destructive Button clicked"); },
            "Destructive Button"
          }

          // outline button
          Button {
            variant: ButtonVariant::Outline,
            button_type: ButtonType::Reset,
            on_click: move |_| { log::info!("Outline Button clicked and form reset"); },
            "Outline Button"
          }

          // small secondary button
          Button {
            variant: ButtonVariant::Secondary,
            size: ButtonSize::Sm,
            button_type: ButtonType::Submit,
            on_click: move |_| { log::info!("Small Submit Button clicked"); },
            "Small Button"
          }

          // large ghost button
          Button {
            variant: ButtonVariant::Ghost,
            size: ButtonSize::Lg,
            button_type: ButtonType::Button,
            on_click: move |_| { log::info!("Large Ghost Button clicked"); },
            "Large Button"
          }

          // link button
          Button {
            variant: ButtonVariant::Link,
            button_type: ButtonType::Button,
            on_click: move |_| { log::info!("Link Button clicked"); },
            "Link Button"
          }

          // icon button example
          Button {
            variant: ButtonVariant::Icon,
            size: ButtonSize::IconLg,
            button_type: ButtonType::Button,
            on_click: move |_| { log::info!("Icon Button clicked"); },
            children: rsx! {
              i { class: "fas fa-plus" } // fontawesome or similar icon
            }
          }
        }
      }

      // input fields section
      ComponentSection {
        title: "Input Fields",
        description: "Text inputs with different variants and states",
        
        div { class: "space-y-4",
          Label {
            label_text: "Default Input".to_string(),
            Input {
              value: text_input.read().to_string(),
              on_change: move |value| text_input.set(value),
              placeholder: "Type something...".to_string(),
            }
          }
          Label {
            label_text: "Underlined Input".to_string(),
            Input {
              variant: InputVariant::Underlined,
              value: text_input.read().to_string(),
              on_change: move |value| text_input.set(value),
              placeholder: Some("Type something...".to_string()),
            }
          }
          Label {
            label_text: "Password Input".to_string(),
            Input {
              input_type: InputType::Password,
              value: text_input.read().to_string(),
              on_change: move |value| text_input.set(value),
              placeholder: "Enter password...".to_string(),
            }
          }
        }  
      }

      // select and multiselect section
      ComponentSection {
        title: "Selection Components",
        description: "Single and multiple selection components",
    
        div { class: "space-y-6",
          Select {
            values: vec!["Option 1".to_string(), "Option 2".to_string(), "Option 3".to_string()],
            current_value: selected_option.read().to_string(),
            callback: move |value| selected_option.set(value),
            label: Some("Single Select".into())
          }
          MultiSelect {
            values: vec!["Item 1".to_string(), "Item 2".to_string(), "Item 3".to_string()],
            current_value: selected_options.read().to_vec(),
            callback: move |value| {
                let mut current = selected_options.read().to_vec();
                if current.contains(&value) {
                  current.retain(|x| x != &value);
                } else {
                  current.push(value);
                }
                selected_options.set(current);
            },
            label: Some("Multi Select".into()),
            placeholder: Some("Select items...".into())
          }
        }

      }

      // toggle and radio section
      ComponentSection {
        title: "Toggle and Radio",
        description: "Toggle switches and radio buttons",
        
        div { class: "space-y-6",
          div { class: "space-y-4",
            ToggleSwitch {
              state: toggle_state,
              label_states: Some(ToggleSwitchLabelStatesProp {
                on: "Enabled",
                off: "Disabled"
              }),
              label_placement: Some(EToggleSwitchLabelPlacement::Right)
            }
          }
          div { class: "space-y-2",
            Radio {
              name: "radio-group".to_string(),
              label: "Option 1".to_string(),
              checked: true,
              on_change: move |_| {}
            }
            Radio {
              name: "radio-group".to_string(),
              label: "Option 2".to_string(),
              checked: false,
              on_change: move |_| {}
            }
          }
        }
        
      }

      // range and calendar section
      ComponentSection {
        title: "Range and Calendar",
        description: "Range slider and calendar components",
        
        div { class: "space-y-8",
          Range {
            current_value: range_value(),
            callback: move |value| range_value.set(value),
            label: Some("Range Slider".into()),
            min_value: 0,
            max_value: 100,
            step: 1
          }
          Calendar {
            display_props: CalendarDisplayProps::builder().build(),
            select_props: CalendarSelectProps::builder().build()
          }
        }
        
      }

      // calendar
      ComponentSection {
        title: "Calendar Components",
        description: "Various calendar implementations showcasing different features and use cases",
        
        div { class: "space-y-8",
          // basic calendar
          div { class: "space-y-4",
            h3 { class: "text-xl font-medium", "Basic Calendar" }
            p { class: "text-gray-600", 
                "Default calendar implementation with standard features"
            }
            Calendar {
              display_props: CalendarDisplayProps::builder().build(),
              select_props: CalendarSelectProps::builder().build()
            }
          }

          // date range calendar
          div { class: "space-y-4",
            h3 { class: "text-xl font-medium", "Date Range Calendar" }
            p { class: "text-gray-600", 
              "Calendar with minimum and maximum date constraints"
            }
            {date_range_calendar()}
          }

          // compact calendar
          div { class: "space-y-4",
            h3 { class: "text-xl font-medium", "Compact Calendar" }
            p { class: "text-gray-600", 
              "Space-efficient calendar variant with button trigger"
            }
            {compact_calendar()}
          }
        }
        
      }

      // textarea and spinner section
      ComponentSection {
        title: "Textarea and Loading",
        description: "Textarea component with multiple configurations and loading spinner",
        
        div { class: "space-y-6",
          // default Textarea
          Label {
            label_text: Some("Default Textarea".into()),
            Textarea {
              value: text_area_value.read().to_string(),
              on_change: move |value| text_area_value.set(value),
              placeholder: Some("Enter text here...".into()),
            }
          }

          // disabled Textarea
          Label {
            label_text: Some("Disabled Textarea".into()),
            Textarea {
              value: "Disabled content".to_string(),
              disabled: true,
              placeholder: Some("Cannot edit this text...".into()),
            }
          }

          // textarea with on_enter functionality
          Label {
            label_text: Some("Textarea with Enter Handler".into()),
            Textarea {
              value: text_area_value.read().to_string(),
              on_change: move |value| text_area_value.set(value),
              on_enter: move |value| entered_text.set(value),
              placeholder: Some("Type and press Shift+Enter...".into()),
            }
          }
          p { class: "text-sm text-gray-600 italic", 
            "Last entered text: {entered_text.read()}"
          }

          // textarea with custom styles
          Label {
            label_text: Some("Custom Styled Textarea".into()),
            Textarea {
              value: text_area_value.read().to_string(),
              on_change: move |value| text_area_value.set(value),
              placeholder: Some("Styled input...".into()),
              class: Some("border-2 border-dashed border-primary text-primary".into()),
              style: Some("padding: 1rem; font-style: italic;".into()),
            }
          }

          // loading Spinner
          div { class: "flex items-center gap-4",
            FreeIconSpinner { size: 24 }
            span { class: "text-sm text-gray-600", "Loading..." }
          }
        }
      }
    }
  }
}

// function for date range calendar
fn date_range_calendar() -> Element {
  let today = Local::now().date_naive();
  let min_date = NaiveDate::from_ymd_opt(today.year(), today.month(), 1).unwrap();
  let max_date = min_date.checked_add_months(chrono::Months::new(2)).unwrap();

  let display_props = CalendarDisplayProps::builder()
    .is_full(true)
    .build();

  let select_props = CalendarSelectProps::builder()
    .min_date(Signal::new(Some(min_date)))
    .max_date(Signal::new(Some(max_date)))
    .build();

  rsx! {
    div { class: "border rounded-lg p-4 bg-white",
      Calendar {
        display_props: display_props,
        select_props: select_props
      }
    }
  }
}

// function for compact calendar
fn compact_calendar() -> Element {
  let display_props = CalendarDisplayProps::builder()
    .is_full(false)
    .build();

  let select_props = CalendarSelectProps::builder().build();

  rsx! {
    div { class: "border rounded-lg p-4 bg-white",
      Calendar {
        display_props: display_props,
        select_props: select_props
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
