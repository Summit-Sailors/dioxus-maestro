use {
  dioxus::prelude::*,
  maestro_radio::use_radio::{use_init_radio_station, use_radio, RadioChannel, RadioStation},
  std::fmt::Display
};

// our app state
#[derive(Clone, Debug)]
struct CounterState {
  pub count: i32,
  pub last_update: String,
}

// our channels for different types of updates
#[derive(PartialEq, Eq, Clone)]
pub enum CounterChannel {
  Increment,
  Decrement,
  Reset,
  All,
}

// Implement RadioChannel trait for our channel enum
impl RadioChannel<CounterState> for CounterChannel {
  fn derive_channel(self, _radio: &CounterState) -> Vec<Self> {
    match self {
      CounterChannel::All => vec![
        CounterChannel::Increment,
        CounterChannel::Decrement,
        CounterChannel::Reset,
      ],
      _ => vec![self],
    }
  }
}

impl Display for CounterChannel {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    match self {
      CounterChannel::Increment => write!(f, "Increment"),
      CounterChannel::Decrement => write!(f, "Decrement"),
      CounterChannel::Reset => write!(f, "Reset"),
      CounterChannel::All => write!(f, "All"),
    }
  }
}

// Main Radio Demo Component
#[component]
pub fn RadioDemo() -> Element {
  let _station: RadioStation<CounterState, CounterChannel> = use_init_radio_station(|| CounterState {
    count: 0,
    last_update: "No updates yet".to_string(),
  });

  rsx! {
    div { 
      class: "max-w-4xl mx-auto p-6 space-y-8",
      
      // header section
      div { 
        class: "text-center mb-8",
        h1 { 
          class: "text-3xl font-bold text-gray-900 mb-4",
          "Radio State Management Demo"
        }
        p { 
          class: "text-gray-600",
          "Demonstrating channel-based state management with multiple subscribers"
        }
      }

      // main content grid
      div { 
          class: "grid grid-cols-1 md:grid-cols-2 gap-6",
          
          // counter controls section
          div { 
            class: "bg-white p-6 rounded-lg shadow-sm border border-gray-200",
            h2 { 
              class: "text-xl font-semibold mb-4",
              "Counter Controls"
            }
            CounterControls {}
          }

          // display section
          div { 
            class: "bg-white p-6 rounded-lg shadow-sm border border-gray-200",
            h2 { 
              class: "text-xl font-semibold mb-4",
              "Counter Display"
            }
            CounterDisplay {}
          }

          // channel Monitor
          div { 
            class: "md:col-span-2 bg-white p-6 rounded-lg shadow-sm border border-gray-200",
            h2 { 
              class: "text-xl font-semibold mb-4",
              "Channel Monitor"
            }
            ChannelMonitor {}
          }
      }
    }
  }
}

// component for counter controls
#[component]
fn CounterControls() -> Element {
  let mut radio = use_radio(CounterChannel::All);

  let handle_increment = move |_| {
    let mut guard = radio.write_channel(CounterChannel::Increment);
    let state = &mut *guard;
    state.count += 1;
    state.last_update = "Incremented".to_string();
  };

  let handle_decrement = move |_| {
    let mut guard = radio.write_channel(CounterChannel::Decrement);
    let state = &mut *guard;
    state.count -= 1;
    state.last_update = "Decremented".to_string();
  };

  let handle_reset = move |_| {
    let mut guard = radio.write_channel(CounterChannel::Reset);
    let state = &mut *guard;
    state.count = 0;
    state.last_update = "Reset".to_string();
  };

  rsx! {
    div { 
      class: "space-x-4",
      button {
        class: "px-4 py-2 bg-blue-500 text-white rounded hover:bg-blue-600",
        onclick: handle_increment,
        "Increment"
      }
      button {
        class: "px-4 py-2 bg-blue-500 text-white rounded hover:bg-blue-600",
        onclick: handle_decrement,
        "Decrement"
      }
      button {
        class: "px-4 py-2 bg-red-500 text-white rounded hover:bg-red-600",
        onclick: handle_reset,
        "Reset"
      }
    }
  }
}

// component for displaying counter value
#[component]
fn CounterDisplay() -> Element {
  let radio = use_radio(CounterChannel::All);
  let state = radio.read();

  rsx! {
    div { 
      class: "space-y-4",
      div {
        class: "text-4xl font-bold text-center",
        "{state.count}"
      }
      div {
        class: "text-sm text-gray-500 text-center",
        "Last update: {state.last_update}"
      }
    }
  }
}

// component for monitoring channel activities
#[component]
fn ChannelMonitor() -> Element {
  let increment_radio = use_radio(CounterChannel::Increment);
  let decrement_radio = use_radio(CounterChannel::Decrement);
  let reset_radio = use_radio(CounterChannel::Reset);

  let increment_count = increment_radio.read().count;
  let decrement_count = decrement_radio.read().count;
  let reset_count = reset_radio.read().count;

  rsx! {
    div { 
      class: "grid grid-cols-3 gap-4",
      
      // individual channel monitors
      ChannelCard {
        channel: CounterChannel::Increment,
        value: increment_count
      }
      ChannelCard {
        channel: CounterChannel::Decrement,
        value: decrement_count
      }
      ChannelCard {
        channel: CounterChannel::Reset,
        value: reset_count
      }
    }
  }
}


#[component]
fn ChannelCard(channel: CounterChannel, value: i32) -> Element {
  rsx! {
    div { 
      class: "p-4 bg-gray-50 rounded-lg border border-gray-200",
      div {
        class: "font-medium text-gray-700 mb-2",
        "{channel} Channel"
      }
      div {
        class: "text-2xl font-bold text-gray-900",
        "Value: {value}"
      }
    }
  }
}
