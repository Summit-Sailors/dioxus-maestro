use dioxus::prelude::*;

#[component]
pub fn Expand(class: Option<String>) -> Element {
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
			path { d: "M 4 4 L 4 13 L 6 13 L 6 7.4375 L 14.5625 16 L 6 24.5625 L 6 19 L 4 19 L 4 28 L 13 28 L 13 26 L 7.4375 26 L 16 17.4375 L 24.5625 26 L 19 26 L 19 28 L 28 28 L 28 19 L 26 19 L 26 24.5625 L 17.4375 16 L 26 7.4375 L 26 13 L 28 13 L 28 4 L 19 4 L 19 6 L 24.5625 6 L 16 14.5625 L 7.4375 6 L 13 6 L 13 4 Z" }
		}
	}
}
