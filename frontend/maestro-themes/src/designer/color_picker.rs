// Color selection component

use {crate::designer::state::ColorPalette, dioxus::prelude::*};

#[derive(Props, PartialEq, Clone)]
pub struct ColorPickerProps {
	/// Current color Palette
	color: ColorPalette,
	/// Callback wneh color changes
	on_change: EventHandler<ColorPalette>,
}

#[component]
pub fn ColorPicker(props: ColorPickerProps) -> Element {
	let colors = props.colors.clone();

	let update_color = move |field: &'static str, value: String| {
		let mut new_colors = props.colors.clone();
		match field {}
		props.on_change.call(new_colors)
	};

	rsx! {}
}

#[derive(Props, PartialEq, Clone)]
struct ColorInputProps {
	label: &'static str,
	value: String,
	on_change: EventHandler<String>,
}

#[component]
fn ColorIput(props: ColorInputProps) -> Element {
	let handle_change = move |evt| props.on_change.call(evt.value());

	rsx! {}
}
