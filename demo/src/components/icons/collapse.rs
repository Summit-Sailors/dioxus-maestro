use dioxus::prelude::*;

#[component]
pub fn Collapse(class: Option<String>) -> Element {
	rsx! {
		svg {
			class: class.unwrap_or_default(),
			stroke: "currentColor",
			fill: "currentColor",
			stroke_width: "0",
			view_box: "0 0 32 32",
			height: "200px",
			width: "200px",
			xmlns: "http://www.w3.org/2000/svg",
			path { d: "M 4.71875 3.28125 L 3.28125 4.71875 L 10.5625 12 L 5 12 L 5 14 L 14 14 L 14 5 L 12 5 L 12 10.5625 Z M 27.28125 3.28125 L 20 10.5625 L 20 5 L 18 5 L 18 14 L 27 14 L 27 12 L 21.4375 12 L 28.71875 4.71875 Z M 5 18 L 5 20 L 10.5625 20 L 3.28125 27.28125 L 4.71875 28.71875 L 12 21.4375 L 12 27 L 14 27 L 14 18 Z M 18 18 L 18 27 L 20 27 L 20 21.4375 L 27.28125 28.71875 L 28.71875 27.28125 L 21.4375 20 L 27 20 L 27 18 Z" }
		}
	}
}
