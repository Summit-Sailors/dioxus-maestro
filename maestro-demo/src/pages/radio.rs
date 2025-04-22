use std::fmt::Display;

use dioxus::prelude::*;
use maestro_radio::use_radio::{RadioChannel, RadioStation, use_init_radio_station, use_radio};

use crate::components::ui::features::Features;

#[derive(Debug, Clone)]
struct CounterState {
	pub count: i32,
	pub last_update: String,
	// channel-specific counts to track per-channel state
	pub increment_count: i32,
	pub decrement_count: i32,
	pub reset_count: i32,
}

#[derive(Clone, PartialEq, Eq)]
pub enum CounterChannel {
	Increment,
	Decrement,
	Reset,
	All,
}

impl RadioChannel<CounterState> for CounterChannel {
	fn derive_channel(self, _radio: &CounterState) -> Vec<Self> {
		match self {
			CounterChannel::All => vec![CounterChannel::Increment, CounterChannel::Decrement, CounterChannel::Reset],
			_ => vec![self],
		}
	}
}

impl Display for CounterChannel {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		match self {
			CounterChannel::Increment => write!(f, "+"),
			CounterChannel::Decrement => write!(f, "-"),
			CounterChannel::Reset => write!(f, "Reset"),
			CounterChannel::All => write!(f, "All"),
		}
	}
}

#[component]
pub fn RadioDemo() -> Element {
	let station: RadioStation<CounterState, CounterChannel> =
		use_init_radio_station(|| CounterState { count: 0, last_update: "No updates yet".to_string(), increment_count: 0, decrement_count: 0, reset_count: 0 });

	provide_context(station);

	rsx! {
		div {
			id: "maestro-radio",
			class: "grid justify-center grid-cols-1 p-4 space-y-6 bg-[color:var(--bg-color)] text-[color:var(--text-color)] rounded-lg",

			div { id: "maestro-radio-header", class: "mb-4",
				h1 { class: "text-[color:var(--text-color)] text-center text-3xl font-bold mb-2",
					"Maestro Radio"
				}
				p { class: "text-[color:var(--muted-text)] text-center",
					"Maestro Radio is a state management utility for Dioxus that takes reactive state to the next level with its innovative channel-based approach. Unlike traditional state management solutions,
				Maestro Radio provides a flexible, granular, and type-safe mechanism for managing complex application states."
				}
			}

			div { id: "maestro-radio-features", class: "flex space-x-2",
				Features {
					title: "Features".to_string(),
					features: vec![
							"Granular Control: Define custom channels for different types of state updates"
									.to_string(),
							"Targeted Notifications: Precisely control which components get updated"
									.to_string(),
							"Flexible Derivation: Easily create derived channels with custom logic"
									.to_string(),
							"Automatic scope-based subscriptions".to_string(),
							"Lightweight and zero-overhead design".to_string(),
							"Seamless integration with Dioxus component lifecycle".to_string(),
					],
				}
			}

			div {
				id: "maestro-radio-main",
				class: "grid grid-cols-1 md:grid-cols-3 gap-3",

				div {
					id: "maestro-radio-controls",
					class: "bg-[color:var(--card-bg)] text-[color:var(--card-text)] p-4 rounded-xl shadow-md border border-[color:var(--border-color)]",
					h2 { class: "text-xl text-center font-semibold mb-4", "Counter Controls" }
					CounterControls {}
				}

				div {
					id: "maestro-radio-display",
					class: "bg-[color:var(--card-bg)] text-[color:var(--card-text)] p-4 rounded-xl shadow-md border border-[color:var(--border-color)]",
					h2 { class: "text-xl text-center font-semibold mb-4", "Counter Display" }
					CounterDisplay {}
				}

				div {
					id: "maestro-radio-monitor",
					class: "lg:col-span-1 bg-[color:var(--card-bg)] text-[color:var(--card-text)] p-4 rounded-xl shadow-md border border-[color:var(--border-color)]",
					h2 { class: "text-xl text-center font-semibold mb-4", "Channel Monitor" }
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
		div { class: "grid grid-cols-1 space-y-2 rounded-lg",
			button {
				class: "flex-1 px-4 py-2 bg-[color:var(--primary-bg)] text-[color:var(--primary-text)] rounded-lg shadow-md hover:bg-blue-600 transition",
				onclick: handle_increment,
				"+"
			}
			button {
				class: "flex-1 px-4 py-2 bg-[color:var(--secondary-bg)] text-[color:var(--secondary-text)] rounded-lg shadow-md hover:bg-yellow-600 transition",
				onclick: handle_decrement,
				"-"
			}
			button {
				class: "flex-1 px-4 py-2 bg-[color:var(--destructive)] text-[color:var(--destructive-foreground)] rounded-lg shadow-md hover:bg-red-600 transition",
				onclick: handle_reset,
				"Reset"
			}
		}
	}
}

#[component]
fn CounterDisplay() -> Element {
	let increment_radio = use_radio(CounterChannel::Increment);
	let _decrement_radio = use_radio(CounterChannel::Decrement);
	let _reset_radio = use_radio(CounterChannel::Reset);

	let state = increment_radio.read();

	rsx! {
		div { class: "space-y-4 text-center bg-[color:var(--card-bg)] text-[color:var(--card-text)] border border-[color:var(--border-color)] rounded-lg p-2",
			div { class: "text-5xl font-bold", "{state.count}" }
			div { class: "text-sm text-[color:var(--muted-text)]", "Last update: {state.last_update}" }
		}
	}
}

#[component]
fn ChannelMonitor() -> Element {
	rsx! {
		div { class: "grid grid-cols-1 sm:grid-cols-1 md:grid-cols-1 text-center gap-4",
			ChannelCard {
				channel: CounterChannel::Increment,
				value: use_radio(CounterChannel::Increment).read().increment_count,
			}
			ChannelCard {
				channel: CounterChannel::Decrement,
				value: use_radio(CounterChannel::Decrement).read().decrement_count,
			}
			ChannelCard {
				channel: CounterChannel::Reset,
				value: use_radio(CounterChannel::Reset).read().reset_count,
			}
		}
	}
}

#[component]
fn ChannelCard(channel: CounterChannel, value: i32) -> Element {
	rsx! {
		div { class: "p-4 bg-[color:var(--card-bg)] text-[color:var(--card-text)] rounded-lg border border-[color:var(--border-color)] shadow-md",
			div { class: "font-bold mb-2 text-lg", "{channel} Channel" }
			div { class: "text-3xl font-bold", "{value}" }
		}
	}
}
