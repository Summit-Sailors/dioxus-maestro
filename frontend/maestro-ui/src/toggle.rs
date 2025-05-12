use dioxus::prelude::*;
use serde::{Deserialize, Serialize};
use strum_macros::{Display, EnumIter, EnumString};
use tailwind_fuse::tw_merge;

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, EnumIter, Display, EnumString)]
pub enum EToggleSwitchLabelPlacement {
	Left,
	#[default]
	Right,
	Top,
	Center,
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
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

#[derive(Debug, Default, Clone, PartialEq, Eq, Props)]
pub struct ToggleSwitchProps {
	state: Signal<bool>,
	label_states: Option<ToggleSwitchLabelStatesProp>,
	label_placement: Option<EToggleSwitchLabelPlacement>,
	#[props(default = String::new())]
	class: String,
	#[props(default = String::new())]
	label_class: String,
	#[props(default = String::new())]
	toggled_class: String,
	#[props(default = String::new())]
	default_class: String,
	#[props(default = String::new())]
	toggled_translate_class: String,
	#[props(default = String::new())]
	default_translate_class: String,
}

#[component]
pub fn ToggleSwitch(props: ToggleSwitchProps) -> Element {
	rsx! {
		div { class: "inline-flex items-center justify-center gap-3",
			match props.label_states {
					Some(label_states) => {
							let label = label_states.get_label(*props.state.read());
							let label_placement = props.label_placement.unwrap_or_default();
							rsx! {
								if EToggleSwitchLabelPlacement::Left == label_placement {
									span {
										class: tw_merge!("text-lg maestro-toggle__label", & props.label_class),
										aria_label: label,
										{label}
									}
								}
								RawToggleSwitch {
									state: props.state,
									class: props.class.clone(),
									toggled_class: props.toggled_class.clone(),
									default_class: props.default_class.clone(),
									toggled_translate_class: props.toggled_translate_class.clone(),
									default_translate_class: props.default_translate_class.clone(),
								}
								if EToggleSwitchLabelPlacement::Right == label_placement {
									span {
										class: tw_merge!("text-lg maestro-toggle__label", & props.label_class),
										aria_label: label,
										{label}
									}
								}
							}
					}
					None => rsx! {
						RawToggleSwitch {
							state: props.state,
							class: props.class.clone(),
							toggled_class: props.toggled_class.clone(),
							default_class: props.default_class.clone(),
							toggled_translate_class: props.toggled_translate_class.clone(),
							default_translate_class: props.default_translate_class.clone(),
						}
					},
			}
		}
	}
}

#[component]
pub fn RawToggleSwitch(
	mut state: Signal<bool>,
	class: String,
	toggled_class: String,
	default_class: String,
	toggled_translate_class: String,
	default_translate_class: String,
) -> Element {
	rsx! {
		button {
			class: tw_merge!(
					"maestro-toggle__switch relative inline-flex h-6 w-11 items-center rounded-full transition-colors",
					class, if state() { format!("maestro-toggleon {toggled_class}") } else {
					format!("maestro-toggleoff {default_class}") }
			),
			r#type: "button",
			role: "switch",
			aria_checked: state(),
			onclick: move |_| {
					state.toggle();
			},
			div {
				class: tw_merge!(
						"maestro-toggle__handle absolute h-5 w-5 transform rounded-full transition-transform",
						if state() { format!("translate-x-5 {toggled_translate_class}") } else {
						format!("translate-x-1 {default_translate_class}") }
				),
			}
		}
	}
}
