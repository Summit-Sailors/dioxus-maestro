// Here styles are generated based on components.
use anyhow::{Error, Result};
pub fn get_component_styles(components_section_id: &str) -> Result<String, Error> {
	match components_section_id {
		"buttons" => {
			let button_styles = include_str!("../../../../css/buttons.css");
			Ok(String::from(button_styles))
		},
		"input-and-labels" => {
			let input_styles = include_str!("../../../../css/input_and_label.css");
			Ok(String::from(input_styles))
		},
		"select-and-multiselect" => {
			let select_css = include_str!("../../../../css/select.css");
			let multiselect_css = include_str!("../../../../css/multiselect.css");

			let combined_css = format!("{}\n{}", select_css, multiselect_css);
			Ok(combined_css)
		},
		"toggle-and-radio" => {
			let radio_css = include_str!("../../../../css/radio.css");
			let toggle_css = include_str!("../../../../css/toggle.css");

			let combined_css = format!("{}\n{}", radio_css, toggle_css);
			Ok(combined_css)
		},
		"textarea-spinner-range" => {
			let textarea_css = include_str!("../../../../css/textarea.css");
			let range_css = include_str!("../../../../css/range.css");

			let combined_css = format!("{}\n{}", textarea_css, range_css);
			Ok(combined_css)
		},
		_ => Err(Error::msg(format!("Unsupported components section ID: {}", components_section_id))),
	}
}
