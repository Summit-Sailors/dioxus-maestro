use {
	dioxus::{prelude::*, web::WebEventExt},
	std::rc::Rc,
	web_sys::{
		AddEventListenerOptions, EventTarget, HtmlElement,
		wasm_bindgen::{JsCast, prelude::Closure},
		window,
	},
};

#[derive(Props, PartialEq, Clone)]
pub struct FocusTrapProps {
	#[props(extends = div, extends = GlobalAttributes)]
	attributes: Vec<Attribute>,
	children: Element,
}

#[component]
pub fn FocusTrap(props: FocusTrapProps) -> Element {
	let mut current_ref = use_signal(|| None::<Rc<MountedData>>);
	let mut closure_focus_in_ref = use_signal(|| None::<Closure<dyn FnMut(web_sys::FocusEvent)>>);
	let mut closure_focus_out_ref = use_signal(|| None::<Closure<dyn FnMut(web_sys::FocusEvent)>>);
	let mut closure_keyboard_ref = use_signal(|| None::<Closure<dyn FnMut(web_sys::KeyboardEvent)>>);

	let get_tabbable_candidates = use_callback(move |()| {
		if let Some(node) = current_ref.read().as_ref() {
			let node = node.try_as_web_event();
			if let Some(node) = node {
				let query = node.query_selector_all("[tabindex]:not([tabindex='-1']):not([disabled]):not([aria-disabled='true']):not([data-disabled='true']), a[href]:not([disabled]):not([aria-disabled='true']):not([data-disabled='true']), button:not([disabled]):not([aria-disabled='true']):not([data-disabled='true']), input:not([disabled]):not([aria-disabled='true']):not([data-disabled='true']), select:not([disabled]):not([aria-disabled='true']):not([data-disabled='true']), textarea:not([disabled]):not([aria-disabled='true']):not([data-disabled='true'])").unwrap();
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
		} else {
			Vec::new()
		}
	});

	use_effect(move || {
		let window = window().expect("should have a window in this context");
		let document = window.document().expect("window should have a document");

		if closure_focus_in_ref.peek().is_none() {
			let closure = Closure::wrap(Box::new(move |event: web_sys::FocusEvent| {
				if let Some(node) = current_ref.read().as_ref() {
					let node = node.try_as_web_event();
					if let Some(node) = node {
						if let Some(target) = event.target().and_then(|t: EventTarget| t.dyn_into::<HtmlElement>().ok()) {
							if !node.contains(Some(&target)) {
								let tabbables = get_tabbable_candidates(());
								if !tabbables.is_empty() {
									tabbables[0].focus().ok();
								}
							}
						}
					}
				}
			}) as Box<dyn FnMut(web_sys::FocusEvent)>);
			closure_focus_in_ref.set(Some(closure));
		}

		if let Some(closure) = &*closure_focus_in_ref.read() {
			let options = AddEventListenerOptions::new();
			document
				.add_event_listener_with_callback_and_add_event_listener_options("focusin", closure.as_ref().unchecked_ref(), options.as_ref())
				.expect("should register event listener");
		}
	});

	use_effect(move || {
		let window = window().expect("should have a window in this context");
		let document = window.document().expect("window should have a document");

		if closure_focus_out_ref.peek().is_none() {
			let closure = Closure::wrap(Box::new(move |event: web_sys::FocusEvent| {
				if let Some(node) = current_ref.read().as_ref() {
					let node = node.try_as_web_event();
					if let Some(node) = node {
						if let Some(target) = event.related_target().and_then(|t: EventTarget| t.dyn_into::<HtmlElement>().ok()) {
							if !node.contains(Some(&target)) {
								let tabbables = get_tabbable_candidates(());
								if !tabbables.is_empty() {
									tabbables[0].focus().ok();
								}
							}
						}
					}
				}
			}) as Box<dyn FnMut(web_sys::FocusEvent)>);
			closure_focus_out_ref.set(Some(closure));
		}

		if let Some(closure) = &*closure_focus_out_ref.read() {
			let options = AddEventListenerOptions::new();
			document
				.add_event_listener_with_callback_and_add_event_listener_options("focusout", closure.as_ref().unchecked_ref(), options.as_ref())
				.expect("should register event listener");
		}
	});

	use_effect(move || {
		let window = window().expect("should have a window in this context");
		let document = window.document().expect("window should have a document");

		if closure_keyboard_ref.peek().is_none() {
			let cloned_document = document.clone();
			let closure = Closure::wrap(Box::new(move |event: web_sys::KeyboardEvent| {
				if let Some(node) = current_ref.read().as_ref() {
					let node = node.try_as_web_event();
					if node.is_some() {
						let tabbables = get_tabbable_candidates(());
						if !tabbables.is_empty() {
							let active = cloned_document.active_element().and_then(|e| e.dyn_into::<HtmlElement>().ok());
							if let Some(active) = active {
								let active_index = tabbables.iter().position(|el| *el == active);
								if let Some(index) = active_index {
									match event.key().as_str() {
										"Tab" =>
											if !event.shift_key() && active == *tabbables.last().unwrap() {
												event.prevent_default();
												tabbables[0].focus().ok();
											} else if event.shift_key() && active == tabbables[0] {
												event.prevent_default();
												tabbables.last().unwrap().focus().ok();
											},
										"ArrowDown" => {
											event.prevent_default();
											let next_index = (index + 1) % tabbables.len();
											tabbables[next_index].focus().ok();
										},
										"ArrowUp" => {
											event.prevent_default();
											let prev_index = if index == 0 { tabbables.len() - 1 } else { index - 1 };
											tabbables[prev_index].focus().ok();
										},
										_ => {},
									}
								}
							}
						}
					}
				}
			}) as Box<dyn FnMut(web_sys::KeyboardEvent)>);
			closure_keyboard_ref.set(Some(closure));
		}
		if let Some(closure) = &*closure_keyboard_ref.read() {
			let options = AddEventListenerOptions::new();
			document
				.add_event_listener_with_callback_and_add_event_listener_options("keydown", closure.as_ref().unchecked_ref(), options.as_ref())
				.expect("should register event listener");
		}
	});

	use_drop(move || {
		let window = window().expect("should have a window in this context");
		let document = window.document().expect("window should have a document");

		if let Some(closure) = closure_focus_in_ref.peek().as_ref() {
			document.remove_event_listener_with_callback("focusin", closure.as_ref().unchecked_ref()).expect("should remove event listener");
			drop(closure);
		}
		if let Some(closure) = closure_focus_out_ref.peek().as_ref() {
			document.remove_event_listener_with_callback("focusout", closure.as_ref().unchecked_ref()).expect("should remove event listener");
			drop(closure);
		}
		if let Some(closure) = closure_keyboard_ref.peek().as_ref() {
			document.remove_event_listener_with_callback("keydown", closure.as_ref().unchecked_ref()).expect("should remove event listener");
			drop(closure);
		}
	});

	rsx! {
		div {
			tabindex: "0",
			onmounted: move |event| {
					current_ref.set(Some(event.data()));
					let tabbables = get_tabbable_candidates(());
					if !tabbables.is_empty() {
							tabbables[0].focus().ok();
					}
			},
			..props.attributes,
			{props.children}
		}
	}
}
