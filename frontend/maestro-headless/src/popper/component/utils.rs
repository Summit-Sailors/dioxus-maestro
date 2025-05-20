use {
	crate::shared::ESide,
	dioxus::{prelude::*, web::WebEventExt},
	serde::{Deserialize, Serialize},
	std::rc::Rc,
	web_sys::{HtmlElement, wasm_bindgen::JsCast, window},
};

#[derive(Clone, Debug, PartialEq)]
pub struct Rect {
	pub x: f32,
	pub y: f32,
	pub width: f32,
	pub height: f32,
}

#[derive(Clone, Debug, PartialEq)]
pub struct Placement {
	pub side: ESide,
	pub alignment: Option<Alignment>,
}

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum Alignment {
	Start,
	End,
}

impl Placement {
	pub fn side(&self) -> ESide {
		self.side
	}

	pub fn alignment(&self) -> Option<Alignment> {
		self.alignment
	}
}

#[derive(Clone, Debug, Default, PartialEq)]
pub struct FloatingStyles {
	pub position: String,
	pub top: String,
	pub left: String,
	pub transform: Option<String>,
}

impl FloatingStyles {
	pub fn style_position(&self) -> String {
		self.position.clone()
	}

	pub fn style_top(&self) -> String {
		self.top.clone()
	}

	pub fn style_left(&self) -> String {
		self.left.clone()
	}

	pub fn style_transform(&self) -> Option<String> {
		self.transform.clone()
	}
}

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct ArrowData {
	pub x: Option<f32>,
	pub y: Option<f32>,
	pub center_offset: f32,
}

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct TransformOriginData {
	pub x: String,
	pub y: String,
}

pub fn get_element_rect(ref_element: Signal<Option<Rc<MountedData>>>) -> Option<Rect> {
	if let Some(data) = ref_element.peek().as_ref() {
		if let Some(element) = data.try_as_web_event() {
			let rect = element.get_bounding_client_rect();
			let window = window().expect("should have a window in this context");
			let scroll_x = window.page_x_offset().unwrap_or(0.0) as f32;
			let scroll_y = window.page_y_offset().unwrap_or(0.0) as f32;
			return Some(Rect { x: rect.left() as f32 + scroll_x, y: rect.top() as f32 + scroll_y, width: rect.width() as f32, height: rect.height() as f32 });
		};
	}
	None
}

pub fn calculate_position(
	fixed_parent: Option<web_sys::Element>,
	reference_rect: &Rect,
	floating_rect: &Rect,
	placement: &Placement,
	arrow_width: f32,
	arrow_height: f32,
	offset: f32,
	align_offset: f32,
) -> (FloatingStyles, ArrowData, TransformOriginData) {
	let side = placement.side();
	let alignment = placement.alignment();
	let window = window().expect("should have a window in this context");
	let is_parent_positioned = if let Some(positioned_parent) = &fixed_parent {
		let parent_rect = positioned_parent.get_bounding_client_rect();
		parent_rect.left() as f32 != 0.0 || parent_rect.top() as f32 != 0.0
	} else {
		false
	};
	let scroll_x = if is_parent_positioned { 0.0 } else { window.page_x_offset().unwrap_or(0.0) as f32 };
	let scroll_y = if is_parent_positioned { 0.0 } else { window.page_y_offset().unwrap_or(0.0) as f32 };
	let reference_rect_x = if is_parent_positioned { 0.0 } else { reference_rect.x };
	let reference_rect_y = if is_parent_positioned { 0.0 } else { reference_rect.y };

	let (left, top) = match side {
		ESide::Top => (
			reference_rect_x + reference_rect.width / 2.0 - floating_rect.width / 2.0 - scroll_x,
			reference_rect_y - floating_rect.height - offset - arrow_height - scroll_y,
		),
		ESide::Right => (
			if is_parent_positioned {
				-floating_rect.width - offset - arrow_height
			} else {
				reference_rect_x + reference_rect.width + offset + arrow_height - scroll_x
			},
			reference_rect_y + (reference_rect.height / 2.0) - (floating_rect.height / 2.0) - scroll_y,
		),
		ESide::Bottom => (
			reference_rect_x + (reference_rect.width / 2.0) - (floating_rect.width / 2.0) - scroll_x,
			reference_rect_y + reference_rect.height + offset + arrow_height - scroll_y,
		),
		ESide::Left => (
			reference_rect.x - floating_rect.width - offset - arrow_height - scroll_x,
			reference_rect.y + (reference_rect.height / 2.0) - (floating_rect.height / 2.0) - scroll_y,
		),
	};

	let (left, top) = match (side, alignment) {
		(ESide::Top | ESide::Bottom, Some(Alignment::Start)) => (left - reference_rect.width / 2.0 + floating_rect.width / 2.0 + align_offset, top),
		(ESide::Top | ESide::Bottom, Some(Alignment::End)) => (left + reference_rect.width / 2.0 - floating_rect.width / 2.0 - align_offset, top),
		(ESide::Right | ESide::Left, Some(Alignment::Start)) => (left, top - reference_rect.height / 2.0 + floating_rect.height / 2.0 + align_offset),
		(ESide::Right | ESide::Left, Some(Alignment::End)) => (left, top + reference_rect.height / 2.0 - floating_rect.height / 2.0 - align_offset),
		_ => (left, top),
	};

	let (arrow_x, arrow_y) = match (side, alignment) {
		(ESide::Top | ESide::Bottom, None) => {
			let arrow_x = floating_rect.width / 2.0;
			let constrained_x = arrow_x.max(arrow_width).min(floating_rect.width - arrow_width);
			(Some(constrained_x), None)
		},
		(ESide::Right | ESide::Left, None) => {
			let arrow_y = floating_rect.height / 2.0;
			let constrained_y = arrow_y.max(arrow_width).min(floating_rect.height - arrow_width);
			(None, Some(constrained_y))
		},
		(ESide::Top | ESide::Bottom, Some(Alignment::Start)) => {
			let arrow_x = 0.0 + reference_rect.width / 2.0;
			(Some(arrow_x), None)
		},
		(ESide::Top | ESide::Bottom, Some(Alignment::End)) => {
			let arrow_x = floating_rect.width - reference_rect.width / 2.0;
			(Some(arrow_x), None)
		},
		(ESide::Right | ESide::Left, Some(Alignment::Start)) => {
			let arrow_y = 0.0 + reference_rect.height / 2.0;
			(None, Some(arrow_y))
		},
		(ESide::Right | ESide::Left, Some(Alignment::End)) => {
			let arrow_y = floating_rect.height - reference_rect.height / 2.0;
			(None, Some(arrow_y))
		},
	};

	let position = if is_parent_positioned { "absolute".to_string() } else { "fixed".to_string() };

	let styles = FloatingStyles { position, top: "0px".to_string(), left: "0px".to_string(), transform: Some(format!("translate({}px, {}px)", left, top)) };

	let arrow_data = ArrowData { x: arrow_x, y: arrow_y, center_offset: 0.0 };

	let (origin_x, origin_y) = match side {
		ESide::Top => (format!("{}px", arrow_x.unwrap_or(floating_rect.width / 2.0)), format!("{}px", floating_rect.height)),
		ESide::Right => ("0px".to_string(), format!("{}px", arrow_y.unwrap_or(floating_rect.height / 2.0))),
		ESide::Bottom => (format!("{}px", arrow_x.unwrap_or(floating_rect.width / 2.0)), "0px".to_string()),
		ESide::Left => (format!("{}px", floating_rect.width), format!("{}px", arrow_y.unwrap_or(floating_rect.height / 2.0))),
	};

	let transform_origin = TransformOriginData { x: origin_x, y: origin_y };

	(styles, arrow_data, transform_origin)
}

pub fn find_scrollable_parents(element: &Rc<MountedData>) -> Vec<web_sys::Element> {
	let mut elements: Vec<web_sys::Element> = Vec::new();
	let overflow_values = ["auto", "scroll", "overlay", "hidden", "clip"];
	if let Some(element) = element.try_as_web_event().and_then(|x| x.dyn_into::<HtmlElement>().ok()) {
		let mut current_element = element.clone();

		while let Some(parent) = current_element.parent_element() {
			if let Some(computed_style) = web_sys::window().and_then(|w| w.get_computed_style(&parent).ok()).unwrap_or(None) {
				if let (Ok(overflow), Ok(overflow_y)) = (computed_style.get_property_value("overflow"), computed_style.get_property_value("overflow-y")) {
					if overflow_values.contains(&overflow.as_str()) || overflow_values.contains(&overflow_y.as_str()) {
						elements.push(parent.clone());
					}
				}
			}
			current_element = parent.dyn_into::<HtmlElement>().ok().expect("Error");
		}
	}
	elements
}

pub fn find_positioned_parent(element: &Rc<MountedData>) -> Option<web_sys::Element> {
	let position_values = ["fixed", "absolute", "sticky"];
	if let Some(element) = element.try_as_web_event().and_then(|x| x.dyn_into::<HtmlElement>().ok()) {
		let mut current_element = element.clone();

		while let Some(parent) = current_element.parent_element() {
			if let Some(computed_style) = web_sys::window().and_then(|w| w.get_computed_style(&parent).ok()).unwrap_or(None) {
				if let Ok(position) = computed_style.get_property_value("position") {
					if position_values.contains(&position.as_str()) {
						return Some(parent.clone());
					}
				}
			}
			current_element = parent.dyn_into::<HtmlElement>().ok().expect("Error");
		}
	}
	None
}
