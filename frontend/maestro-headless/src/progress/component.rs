use {
	dioxus::prelude::*,
	serde::{Deserialize, Serialize},
};

#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize, strum_macros::Display)]
enum ProgressState {
	#[strum(to_string = "init")]
	Init,
	#[strum(to_string = "loading")]
	Loading,
	#[strum(to_string = "completed")]
	Completed,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct ProgressContext {
	value: Memo<f32>,
	max: f32,
}

impl ProgressContext {
	pub fn get_label(&self) -> String {
		format!("{}%", (*self.value.peek() / self.max * 100.0).round())
	}

	pub fn get_state(&self) -> ProgressState {
		if *self.value.peek() == 0.0 {
			ProgressState::Init
		} else if *self.value.peek() == self.max {
			ProgressState::Completed
		} else {
			ProgressState::Loading
		}
	}
}

const DEFAULT_MAX: f32 = 100.0;

#[derive(Clone, PartialEq, Props)]
pub struct ProgressRootProps {
	#[props(default = ReadOnlySignal::new(Signal::new(0.0)))]
	value: ReadOnlySignal<f32>,
	#[props(optional, default = DEFAULT_MAX)]
	max: f32,
	#[props(extends = div, extends = GlobalAttributes)]
	attributes: Vec<Attribute>,
	#[props(default = Vec::new())]
	pub extra_attributes: Vec<Attribute>,
	children: Element,
}

#[component]
pub fn ProgressRoot(props: ProgressRootProps) -> Element {
	let ProgressRootProps { value, max, attributes, extra_attributes, children } = props;

	let value = use_memo(move || if value() <= max && value() > 0.0 { value() } else { 0.0 });
	let context = use_context_provider::<ProgressContext>(|| ProgressContext { value, max });

	rsx! {
		div {
			position: "relative",
			role: "progressbar",
			aria_valuemax: max,
			aria_valuemin: 0,
			aria_valuenow: value(),
			aria_valuetext: context.get_label(),
			"data-state": context.get_state().to_string(),
			"data-value": value(),
			"data-max": max,
			..attributes.clone(),
			..extra_attributes.clone(),
			{children.clone()}
		}
	}
}

#[derive(Clone, PartialEq, Props)]
pub struct ProgressIndicatorProps {
	#[props(extends = div, extends = GlobalAttributes)]
	attributes: Vec<Attribute>,
	#[props(default = Vec::new())]
	pub extra_attributes: Vec<Attribute>,
	#[props(optional, default = None)]
	children: Element,
}

#[component]
pub fn ProgressIndicator(props: ProgressIndicatorProps) -> Element {
	let context = use_context::<ProgressContext>();
	let transform = use_memo(move || 100.0 - *context.value.read());

	rsx! {
		div {
			"data-state": context.get_state().to_string(),
			"data-value": *context.value.read(),
			"data-max": context.max,
			transform: format!("translateX(-{}%)", transform()),
			..props.attributes.clone(),
			..props.extra_attributes.clone(),
			{props.children.clone()}
		}
	}
}
