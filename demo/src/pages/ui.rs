use {
  dioxus::prelude::*, dioxus_free_icons::{ Icon, icons::fa_solid_icons::FaRubleSign}, maestro_ui::{
    button::{Button, ButtonSize, ButtonType, ButtonVariant}, 
    input::{Input, InputType, InputVariant}, 
    label::Label, 
    radio::Radio, select::Select, 
    spinner::FreeIconSpinner, 
    textarea::Textarea, 
    toggle::{EToggleSwitchLabelPlacement, ToggleSwitch, ToggleSwitchLabelStatesProp}
  }

};

#[component]
pub fn UIDemo() -> Element {
  let mut selected_option = use_signal(|| "Option 1".to_string());
  let mut selected_options = use_signal(Vec::<String>::new);
  let toggle_state = use_signal(|| false);
  let mut text_input = use_signal(String::new);
  let mut text_area_value = use_signal(String::new);
  let mut entered_text = use_signal(String::new);

  let selected_value = use_signal(|| "option1".to_string());

  let handle_radio_change = |value: String| {
      let mut selected_value = selected_value.clone();
      move |_| selected_value.set(value.clone())
  };

  rsx! {
    div { class: "max-w-4xl mx-auto py-8 px-4",
      h1 { class: "text-3xl font-bold mb-8", "Maestro UI Components" }
      
      // buttons section
      ComponentSection {
        title: "Buttons",
        description: "Various button styles, sizes, and types with different variants",
        div { class: "grid grid-cols-2 md:grid-cols-3 gap-6",
          Button { 
            class: "px-4 py-2 rounded-lg font-medium transition-colors hover:bg-blue-700", 
            variant: ButtonVariant::Default, 
            button_type: ButtonType::Button, 
            on_click: move |_| { log::info!("Default Button clicked"); }, "Default Button" 
          }
          Button { 
            class: "px-4 py-2 rounded-lg bg-red-500 text-white hover:bg-red-600", 
            variant: ButtonVariant::Destructive, 
            button_type: ButtonType::Button, 
            on_click: move |_| { log::info!("Destructive Button clicked"); }, 
            "Destructive Button" 
          }
          Button { 
            class: "px-4 py-2 border border-gray-400 rounded-lg hover:bg-gray-100", 
            variant: ButtonVariant::Outline, 
            button_type: ButtonType::Reset, 
            on_click: move |_| { log::info!("Outline Button clicked and form reset"); }, 
            "Outline Button" 
          }
          Button { 
            class: "px-2 py-1 rounded-md bg-gray-300 text-sm hover:bg-gray-400", 
            variant: ButtonVariant::Secondary, 
            size: ButtonSize::Sm, 
            button_type: ButtonType::Submit, 
            on_click: move |_| { log::info!("Small Submit Button clicked"); }, 
            "Small Button" 
          }
          Button { 
            class: "px-6 py-3 rounded-lg bg-transparent border border-gray-300 text-gray-600 hover:text-black", 
            variant: ButtonVariant::Ghost, 
            size: ButtonSize::Lg, 
            button_type: ButtonType::Button, 
            on_click: move |_| { log::info!("Large Ghost Button clicked"); }, 
            "Large Button" 
          }
          Button { 
            class: "text-blue-500 underline hover:text-blue-700", 
            variant: ButtonVariant::Link, 
            button_type: ButtonType::Button, 
            on_click: move |_| { log::info!("Link Button clicked"); }, 
            "Link Button" 
          }
          
          Button {
            class: "w-10 h-10 bg-gray-400 rounded-full flex items-center justify-center hover:bg-gray-300",
            variant: ButtonVariant::Icon,
            size: ButtonSize::IconLg,
            button_type: ButtonType::Button,
            on_click: move |_| { log::info!("Icon Button clicked"); },
            children: rsx! {
              Icon {
                icon: FaRubleSign,
                width: 16,
                height: 16
              }
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
              class: "border rounded-lg px-3 py-2 w-full focus:ring focus:ring-blue-300", 
              value: text_input.read().to_string(), 
              on_change: move |value| text_input.set(value), 
              placeholder: "Type something...".to_string(),
            } 
          }
          Label { 
            label_text: "Underlined Input".to_string(), 
            Input { 
              class: "border-b border-gray-400 w-full focus:ring focus:ring-blue-300", 
              variant: InputVariant::Underlined, 
              value: text_input.read().to_string(), 
              on_change: move |value| text_input.set(value), 
              placeholder: Some("Type something...".to_string()), 
            } 
          }
          Label { 
            label_text: "Password Input".to_string(), 
            Input { 
              class: "border rounded-lg px-3 py-2 w-full focus:ring focus:ring-red-300", 
              input_type: InputType::Password, value: text_input.read().to_string(), 
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
            current_value: Some(selected_option.read().to_string()),
            multi: false,
            callback: move |value| selected_option.set(value),
            multi_callback: move |_| {},
            label: Some("Single Select".into()),
            placeholder: Some("Select an option".into()),
          }
          
          Select {
            values: vec!["Item 1".to_string(), "Item 2".to_string(), "Item 3".to_string()],
            current_value: None,
            multi: true,
            callback: move |_| {},
            multi_callback: move |value| selected_options.set(value),
            label: Some("Multi Select".into()),
            placeholder: Some("Select items...".into()),
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
        div { class: "space-y-6",
          div { class: " items-center gap-2",  
          Radio { 
              label: "Option 1", 
              name: "group", 
              checked: *selected_value.read() == "option1", 
              on_change: handle_radio_change("option1".to_string()) 
            }
          }
          div { class: " items-center gap-2",
            Radio { 
              label: "Option 2", 
              name: "group", 
              checked: *selected_value.read() == "option2", 
              on_change: handle_radio_change("option2".to_string()) 
            }
          }
          div { class: "items-center gap-2",
            Radio { 
              label: "Option 3 (Disabled)", 
              name: "group", 
              disabled: true, 
              checked: *selected_value.read() == "option3", 
              on_change: handle_radio_change("option3".to_string()) 
            }
          }
            p { class: "text-sm", "Selected Option: {selected_value}" }
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
              class: "w-full p-4 border rounded-lg border-gray-300 focus:outline-none focus:ring-2 focus:ring-primary"
            }
          }

          // disabled Textarea
          Label {
            label_text: Some("Disabled Textarea".into()),
            Textarea {
              value: "Disabled content".to_string(),
              disabled: true,
              placeholder: Some("Cannot edit this text...".into()),
              class: "w-full p-4 border rounded-lg bg-gray-200 text-gray-500 cursor-not-allowed"
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
              class: "w-full p-4 border rounded-lg border-gray-300 focus:outline-none focus:ring-2 focus:ring-primary"
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
              class: "w-full p-4 border-2 border-dashed border-primary text-primary focus:outline-none focus:ring-2 focus:ring-primary",
              style: "padding: 1rem; font-style: italic;"
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

#[derive(Props, Clone, PartialEq)]
pub struct ComponentSectionProps {
  pub title: &'static str,
  pub description: &'static str,
  pub children: Element,
}

#[component]
pub fn ComponentSection(props: ComponentSectionProps) -> Element {
  rsx! {
    section { class: "mb-12",
      h2 { class: "text-2xl font-semibold mb-2", {props.title} }
      p { class: "text-gray-600 mb-6", {props.description} }
      div { class: "bg-white p-6 rounded-lg shadow-sm border", {props.children} }
    }
  }
}
