use {
  dioxus::prelude::*, 
  dioxus_free_icons::{
    icons::io_icons::{IoCheckmarkOutline, IoChevronDownOutline},
    Icon
  }, 
  std::rc::Rc, tailwind_fuse::tw_join
}; 

  #[derive(Props, Clone)]
  pub struct SelectProps<T>
  where
    T: std::clone::Clone + std::cmp::PartialEq + std::fmt::Display + 'static,
  {
  pub values: Vec<T>,
  pub current_value: Option<T>,
  pub multi: bool,
  pub callback: EventHandler<T>,
  pub multi_callback: EventHandler<Vec<T>>,
  pub label: Option<String>,
  pub placeholder: Option<String>,
  pub option_class: Option<String>,
  pub dropdown_class: Option<String>,
  pub container_class: Option<String>,
  pub button_class: Option<String>,
  pub label_class: Option<String>,
  pub icon_down: Option<Element>,
  pub icon_check: Option<Element>,
  pub option_renderer: Option<Rc<dyn Fn(&T) -> Element>>,
}

impl<T> PartialEq for SelectProps<T>
where
  T: Clone + PartialEq + std::fmt::Display + 'static,
{
  fn eq(&self, other: &Self) -> bool {
    self.values == other.values
      && self.current_value == other.current_value
      && self.multi == other.multi
      && self.label == other.label
      && self.placeholder == other.placeholder
      && self.option_class == other.option_class
      && self.dropdown_class == other.dropdown_class
      && self.container_class == other.container_class
      && self.button_class == other.button_class
      && self.label_class == other.label_class
      && self.icon_down == other.icon_down
      && self.icon_check == other.icon_check
  }
}


#[component]
pub fn Select<T: std::clone::Clone + std::cmp::PartialEq + std::fmt::Display + 'static>(props: SelectProps<T>) -> Element {
  let mut is_opened = use_signal(|| false);
  let mut selected_options = use_signal(|| Vec::<T>::new());

  let display_value = if props.multi {
    if selected_options().is_empty() {
      props.placeholder.clone().unwrap_or_default()
    } else {
      selected_options().iter().map(|v| format!("{v}, ")).collect::<String>().trim_end_matches(", ").to_string()
    }
  } else {
    props.current_value.clone().map(|v| v.to_string()).unwrap_or(props.placeholder.clone().unwrap_or_default())
  };

  let icon_down = props.icon_down.unwrap_or_else(|| rsx!{ Icon { width: 16, height: 16, icon: IoChevronDownOutline } });
  let icon_check = props.icon_check.clone().unwrap_or_else(|| rsx!{ Icon { width: 16, height: 16, icon: IoCheckmarkOutline } });


  rsx! {
    div {
      class: tw_join!("flex flex-col gap-2 w-full relative", props.container_class.clone().unwrap_or_default()),
      if let Some(label) = props.label {
        span { class: tw_join!("text-gray-400", props.label_class.clone().unwrap_or_default()), {label} }
      }
      div {
        class: tw_join!(
          "relative w-full bg-gray-800 text-gray-100 border border-gray-500 rounded-md hover:border-indigo-300 transition-colors ease-linear cursor-pointer",
          is_opened().then_some("ring-1 ring-indigo-500"), props.button_class.clone().unwrap_or_default()
        ),
        onclick: move |ev| {
          ev.prevent_default();
          is_opened.toggle();
        },
        div {
          class: "flex items-center justify-between py-2 px-3 w-full rounded-md focus:outline-none focus:ring-1 focus:ring-indigo-500",
          span { "{display_value}" }
          div { class: "ml-2", {icon_down} }
        }
        div {
          class: tw_join!(
            "absolute flex flex-col gap-1 bg-gray-800 text-gray-200 p-4 w-full left-0 right-0 top-[100%] mt-3 rounded-md border border-gray-700",
            if is_opened() { "flex z-40" } else { "hidden -z-40" }, props.dropdown_class.clone().unwrap_or_default()
          ),
          onclick: move |ev| {
            ev.stop_propagation();
          },
          for value in props.values {
            div {
              key: "{value}",
              class: tw_join!("flex w-full items-center justify-between py-2 hover:bg-gray-700 rounded px-3 cursor-pointer", props.option_class.clone().unwrap_or_default()),
              onclick: move |ev| {
                ev.stop_propagation();
                if props.multi {
                  let mut current = selected_options().clone();
                  if current.contains(&value) {
                    current.retain(|x| x != &value);
                  } else {
                    current.push(value.clone());
                  }
                  selected_options.set(current);
                  props.multi_callback.call(selected_options().clone());
                } else {
                  is_opened.set(false);
                  props.callback.call(value.clone());
                }
              },
              {
                if let Some(renderer) = props.option_renderer.as_ref() {
                  renderer(&value)
                } else {
                  rsx! { "{value}" }
                }
              }
              if props.multi && selected_options().contains(&value) || !props.multi && props.current_value.as_ref() == Some(&value) {
                div {
                  class: "ml-2",
                  {icon_check.clone()}
                }
              }
            }
          }
        }
      }
    }
  }
}
