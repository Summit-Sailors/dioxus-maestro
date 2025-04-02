use {
	crate::shared::EOrientation,
	dioxus::{prelude::*, web::WebEventExt},
	std::rc::Rc,
	web_sys::{HtmlElement, wasm_bindgen::JsCast},
};

pub fn use_arrow_key_navigation(
	current_ref: Signal<Option<Rc<MountedData>>>,
	selector: Option<String>,
	orientation: EOrientation,
) -> Callback<Event<KeyboardData>> {
	let handle_key_down = use_callback(move |event: Event<KeyboardData>| {
		let get_tabbable_candidates = move |node: &Rc<MountedData>, selector: Option<String>| {
			let node = node.try_as_web_event();
			if let Some(node) = node {
				let selector = selector.unwrap_or("[tabindex]:not([tabindex='-1']):not([disabled]):not([aria-disabled='true']):not([data-disabled='true']), a[href]:not([disabled]):not([aria-disabled='true']):not([data-disabled='true']), button:not([disabled]):not([aria-disabled='true']):not([data-disabled='true']), input:not([disabled]):not([aria-disabled='true']):not([data-disabled='true']), select:not([disabled]):not([aria-disabled='true']):not([data-disabled='true']), textarea:not([disabled]):not([aria-disabled='true']):not([data-disabled='true'])".into());
				let query = node.query_selector_all(selector.as_str()).unwrap();
				let mut tabbables = Vec::new();
				for i in 0..query.length() {
					if let Some(element) = query.get(i).and_then(|e| e.dyn_into::<HtmlElement>().ok()) {
						tabbables.push(element);
					}
				}
				tabbables
			} else {
				Vec::new()
			}
		};

		if let Some(node) = current_ref.read().as_ref() {
			let item_elements = get_tabbable_candidates(node, selector.clone());

			let active_element = web_sys::window()
				.and_then(|window| window.document())
				.and_then(|document| document.active_element())
				.and_then(|element| element.dyn_into::<HtmlElement>().ok());

			if let Some(active_element) = active_element {
				if let Some(active_index) = item_elements.iter().position(|el| *el == active_element) {
					let elements_count = item_elements.len();
					let next_index = match orientation {
						EOrientation::Vertical => match event.key() {
							Key::ArrowDown =>
								if active_index + 1 >= elements_count {
									0
								} else {
									active_index + 1
								},
							Key::ArrowUp =>
								if active_index == 0 {
									elements_count.saturating_sub(1)
								} else {
									active_index - 1
								},
							_ => return,
						},
						EOrientation::Horizontal => match event.key() {
							Key::ArrowRight =>
								if active_index + 1 >= elements_count {
									0
								} else {
									active_index + 1
								},
							Key::ArrowLeft =>
								if active_index == 0 {
									elements_count.saturating_sub(1)
								} else {
									active_index - 1
								},
							_ => return,
						},
					};

					if let Some(next_element) = item_elements.get(next_index) {
						next_element.focus().ok();
						event.prevent_default();
					}
				}
			}
		}
	});

	handle_key_down
}
