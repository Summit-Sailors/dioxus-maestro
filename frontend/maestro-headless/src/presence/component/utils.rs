use web_sys::CssStyleDeclaration;

pub fn check_for_transitions(style: &CssStyleDeclaration) -> bool {
	let mut has_transition = false;

	if let Ok(transition) = style.get_property_value("transition") {
		if let Ok(duration) = style.get_property_value("transition-duration") {
			if !duration.is_empty()
				&& duration != "0s"
				&& duration != "0"
				&& !transition.is_empty()
				&& transition != "none"
				&& !transition.starts_with("0s")
				&& !transition.starts_with("none")
			{
				has_transition = true;
			}
		}
	}

	if let Ok(property) = style.get_property_value("transition-property") {
		if let Ok(duration) = style.get_property_value("transition-duration") {
			if !property.is_empty() && property != "none" && !duration.is_empty() && duration != "0s" && duration != "0" {
				has_transition = true;
			}
		}
	}

	has_transition
}

pub fn check_for_animations(style: &CssStyleDeclaration) -> bool {
	let mut has_animation = false;

	if let Ok(name) = style.get_property_value("animation-name") {
		if !name.is_empty() && name != "none" {
			has_animation = true;
		}
	}

	if let Ok(duration) = style.get_property_value("animation-duration") {
		if !duration.is_empty() && duration != "0s" && duration != "0" {
			has_animation = true;
		}
	}

	if let Ok(animation) = style.get_property_value("animation") {
		if !animation.is_empty() && animation != "none" && !animation.starts_with("0s") && !animation.starts_with("none") {
			has_animation = true;
		}
	}

	has_animation
}

pub fn is_element_effectively_hidden(styles: Option<&CssStyleDeclaration>) -> bool {
	if let Some(style) = styles {
		if let Ok(display) = style.get_property_value("display") {
			if display == "none" {
				return true;
			}
		}
		if let Ok(visibility) = style.get_property_value("visibility") {
			if visibility == "hidden" || visibility == "collapse" {
				return true;
			}
		}
		if let Ok(opacity) = style.get_property_value("opacity") {
			if opacity == "0" || opacity == "0.0" || (opacity.parse::<f64>().map_or(false, |v| v < 0.01)) {
				return true;
			}
		}
		if let Ok(transform) = style.get_property_value("transform") {
			if transform.contains("scale(0)") || transform.contains("scale(0,") || transform.contains("scale(0 ") {
				return true;
			}
		}
	}
	false
}
