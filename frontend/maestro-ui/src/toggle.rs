use dioxus::prelude::*;
use serde::{Deserialize, Serialize};
use strum_macros::{Display, EnumIter, EnumString};

#[derive(Clone, Copy, Debug, Default, Deserialize, Display, EnumIter, EnumString, Eq, PartialEq, Serialize)]
pub enum EToggleSwitchLabelPlacement {
	Left,
	#[default]
	Right,
	Top,
	Center,
}

#[derive(Clone, Copy, Debug, Default, Deserialize, Eq, PartialEq, Serialize)]
pub struct ToggleSwitchLabelStatesProp {
	pub on: &'static str,
	pub off: &'static str,
}

impl ToggleSwitchLabelStatesProp {
	pub fn get_label(&self, state: bool) -> &'static str {
		match state {
			true => self.on,
			false => self.off,
		}
	}
}

#[component]
pub fn ToggleSwitch(
	mut state: Signal<bool>, label_states: Option<ToggleSwitchLabelStatesProp>, label_placement: Option<EToggleSwitchLabelPlacement>,
) -> Element {
	rsx! {
		div { class: "inline-flex items-center justify-center gap-3",
			match label_states {

				Some(label_states) => {
					let label = label_states.get_label(state());
					let label_placement=label_placement.unwrap_or_default();
					rsx!{
					if EToggleSwitchLabelPlacement::Left==label_placement {
						span { class: "text-lg text-gray-500", aria_label: label, {label} }
					}
					RawToggleSwitch{state}
					if EToggleSwitchLabelPlacement::Right==label_placement {
						span { class: "text-lg text-gray-500", aria_label: label, {label} }
					}
				}},
				None => rsx!{RawToggleSwitch{state}},
			}
		}
	}
}

#[component]
pub fn RawToggleSwitch(mut state: Signal<bool>) -> Element {
	rsx! {
		button {
			class: "relative inline-flex h-6 w-11 items-center rounded-full transition-colors",
			class: if state() { "bg-blue-600" } else { "bg-gray-200" },
			r#type: "button",
			role: "switch",
			aria_checked: state,
			onclick: move |_| {
					state.set(!state());
			},
			div {
				class: "absolute h-5 w-5 transform rounded-full bg-white transition-transform",
				class: if state() { "translate-x-6" } else { "translate-x-0.5" }
			}
		}
	}
}
