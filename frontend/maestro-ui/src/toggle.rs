use {
	dioxus::prelude::*,
	serde::{Deserialize, Serialize},
	strum_macros::{Display, EnumIter, EnumString},
	tailwind_fuse::tw_merge,
};

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
	class: Option<String>,
	label_class: Option<String>,
	toggled_class: Option<String>,
	default_class: Option<String>,
	toggled_translate_class: Option<String>,
	default_translate_class: Option<String>,
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
										class: tw_merge!("text-lg text-gray-500", props.label_class.clone().unwrap_or_default()),
										aria_label: label,
										{label}
									}
								}
								RawToggleSwitch {
									state: props.state,
									class: props.class.clone().unwrap_or_default(),
									toggled_class: props.toggled_class.clone().unwrap_or_default(),
									default_class: props.default_class.clone().unwrap_or_default(),
									toggled_translate_class: props.toggled_translate_class.clone().unwrap_or_default(),
									default_translate_class: props.default_translate_class.clone().unwrap_or_default(),
								}
								if EToggleSwitchLabelPlacement::Right == label_placement {
									span {
										class: tw_merge!("text-lg text-gray-500", props.label_class.clone().unwrap_or_default()),
										aria_label: label,
										{label}
									}
								}
							}
					}
					None => rsx! {
						RawToggleSwitch {
							state: props.state,
							class: props.class.clone().unwrap_or_default(),
							toggled_class: props.toggled_class.clone().unwrap_or_default(),
							default_class: props.default_class.clone().unwrap_or_default(),
							toggled_translate_class: props.toggled_translate_class.clone().unwrap_or_default(),
							default_translate_class: props.default_translate_class.clone().unwrap_or_default(),
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
					"relative inline-flex h-6 w-11 items-center rounded-full transition-colors",
					class, if state() { format!("bg-blue-600 {}", toggled_class) } else {
					format!("bg-gray-500 {}", default_class) }
			),
			r#type: "button",
			role: "switch",
			aria_checked: state,
			onclick: move |_| {
					state.toggle();
			},
			div {
				class: tw_merge!(
						"absolute h-5 w-5 transform rounded-full bg-white transition-transform", if
						state() { format!("translate-x-5 {}", toggled_translate_class) } else {
						format!("translate-x-1 {}", default_translate_class) }
				),
			}
		}
	}
}
