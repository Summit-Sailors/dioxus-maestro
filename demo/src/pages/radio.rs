use {
  dioxus::prelude::*,
  maestro_radio::use_radio::{use_init_radio_station, use_radio, RadioChannel, RadioStation},
  std::fmt::Display,
};

#[derive(Clone, Debug)]
struct CounterState {
  pub count: i32,
  pub last_update: String,
  // channel-specific counts to track per-channel state
  pub increment_count: i32,
  pub decrement_count: i32,
  pub reset_count: i32,
}

#[derive(PartialEq, Eq, Clone)]
pub enum CounterChannel {
  Increment,
  Decrement,
  Reset,
  All,
}

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

#[component]
pub fn RadioDemo() -> Element {
  let station: RadioStation<CounterState, CounterChannel> = use_init_radio_station(|| CounterState {
    count: 0,
    last_update: "No updates yet".to_string(),
    increment_count: 0,
    decrement_count: 0,
    reset_count: 0,
  });

  provide_context(station);

  rsx! {
    div { 
      class: "max-w-4xl mx-auto p-6 space-y-8",
      
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

      div { 
          class: "grid grid-cols-1 md:grid-cols-2 gap-6",
          
          div { 
            class: "bg-white p-6 rounded-lg shadow-sm border border-gray-200",
            h2 { 
              class: "text-xl font-semibold mb-4",
              "Counter Controls"
            }
            CounterControls {}
          }

          div { 
            class: "bg-white p-6 rounded-lg shadow-sm border border-gray-200",
            h2 { 
              class: "text-xl font-semibold mb-4",
              "Counter Display"
            }
            CounterDisplay {}
          }

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

#[component]
fn CounterControls() -> Element {
  let mut increment_radio = use_radio(CounterChannel::Increment);
  let mut decrement_radio = use_radio(CounterChannel::Decrement);
  let mut reset_radio = use_radio(CounterChannel::Reset);

  let handle_increment = move |_| {
    increment_radio.write_with(|mut state| {
      state.count += 1;
      state.increment_count = state.count;
      state.last_update = "Incremented".to_string();
    });
  };

  let handle_decrement = move |_| {
    decrement_radio.write_with(|mut state| {
      state.count -= 1;
      state.decrement_count = state.count;
      state.last_update = "Decremented".to_string();
    });
  };

  let handle_reset = move |_| {
    reset_radio.write_with(|mut state| {
      state.count = 0;
      state.reset_count = state.count;
      state.last_update = "Reset".to_string();
    });
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

#[component]
fn CounterDisplay() -> Element {
  // separate radios for each action
  let increment_radio = use_radio(CounterChannel::Increment);
  let _decrement_radio = use_radio(CounterChannel::Decrement);
  let _reset_radio = use_radio(CounterChannel::Reset);
  
  // reading the latest state from any of the radios (they all share the same state)
  let state = increment_radio.read();
  
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

#[component]
fn ChannelMonitor() -> Element {
  rsx! {
    div { 
      class: "grid grid-cols-3 gap-4",
      ChannelCard {
        channel: CounterChannel::Increment,
        value: use_radio(CounterChannel::Increment).read().increment_count
      }
      ChannelCard {
        channel: CounterChannel::Decrement,
        value: use_radio(CounterChannel::Decrement).read().decrement_count
      }
      ChannelCard {
        channel: CounterChannel::Reset,
        value: use_radio(CounterChannel::Reset).read().reset_count
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
