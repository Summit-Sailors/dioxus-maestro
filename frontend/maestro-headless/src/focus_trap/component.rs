use {
	dioxus::{prelude::*, web::WebEventExt},
	std::rc::Rc,
	uuid::Uuid,
	web_sys::{
		EventTarget, HtmlElement, js_sys,
		wasm_bindgen::{JsCast, JsValue, prelude::Closure},
		window,
	},
};

#[derive(Clone, PartialEq, Props)]
pub struct FocusTrapProps {
	#[props(default = None)]
	pub onclick: Option<EventHandler<Event<MouseData>>>,
	#[props(default = None)]
	pub onkeydown: Option<EventHandler<Event<KeyboardData>>>,
	#[props(default = None)]
	pub onkeyup: Option<EventHandler<Event<KeyboardData>>>,
	#[props(default = None)]
	pub onfocus: Option<EventHandler<Event<FocusData>>>,
	#[props(default = None)]
	pub onblur: Option<EventHandler<Event<FocusData>>>,
	#[props(default = None)]
	pub onmousedown: Option<EventHandler<Event<MouseData>>>,
	#[props(default = None)]
	pub onmouseup: Option<EventHandler<Event<MouseData>>>,
	#[props(default = None)]
	pub onmouseenter: Option<EventHandler<Event<MouseData>>>,
	#[props(default = None)]
	pub onmouseleave: Option<EventHandler<Event<MouseData>>>,
	#[props(optional, default = None)]
	pub onmounted: Option<EventHandler<Event<MountedData>>>,

	#[props(extends = div, extends = GlobalAttributes)]
	attributes: Vec<Attribute>,
	#[props(optional, default = Vec::new())]
	extra_attributes: Vec<Attribute>,
	children: Element,
}

#[component]
pub fn FocusTrap(props: FocusTrapProps) -> Element {
	let FocusTrapProps {
		onclick,
		onkeydown,
		onkeyup,
		onfocus,
		onblur,
		onmousedown,
		onmouseup,
		onmouseenter,
		onmouseleave,
		onmounted,
		attributes,
		extra_attributes,
		children,
	} = props;

	let mut closure_focus_in_ref = use_signal(|| None::<Closure<dyn FnMut(web_sys::FocusEvent)>>);
	let mut closure_focus_out_ref = use_signal(|| None::<Closure<dyn FnMut(web_sys::FocusEvent)>>);
	let mut closure_keyboard_ref = use_signal(|| None::<Closure<dyn FnMut(web_sys::KeyboardEvent)>>);
	let mut current_ref = use_context::<Signal<Option<Rc<MountedData>>>>();

	let mut attrs = attributes.clone();
	attrs.extend(extra_attributes);

	let get_tabbable_candidates = use_callback(move |()| {
		if let Some(node) = current_ref.read().as_ref() {
			let node = node.try_as_web_event();
			if let Some(node) = node {
				let query = node
					.query_selector_all(
						"[tabindex]:not([tabindex='-1']):not([disabled]):not([aria-disabled='true']):not([data-disabled='true']),
a[href]:not([disabled]):not([aria-disabled='true']):not([data-disabled='true']),
button:not([disabled]):not([aria-disabled='true']):not([data-disabled='true']),
input:not([disabled]):not([aria-disabled='true']):not([data-disabled='true']),
select:not([disabled]):not([aria-disabled='true']):not([data-disabled='true']),
textarea:not([disabled]):not([aria-disabled='true']):not([data-disabled='true'])",
					)
					.unwrap();
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
		if let Some(element) = current_ref().and_then(|node| node.try_as_web_event().and_then(|x| x.dyn_into::<HtmlElement>().ok())) {
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

			if closure_keyboard_ref.peek().is_none() {
				let window = window().expect("should have a window in this context");
				let document = window.document().expect("window should have a document");
				let closure = Closure::wrap(Box::new(move |event: web_sys::KeyboardEvent| {
					if let Some(node) = current_ref.read().as_ref() {
						let node = node.try_as_web_event();
						if node.is_some() {
							let tabbables = get_tabbable_candidates(());
							if !tabbables.is_empty() {
								let active = document.active_element().and_then(|e| e.dyn_into::<HtmlElement>().ok());
								if let Some(active) = active {
									let active_index = tabbables.iter().position(|el| *el == active);
									if let Some(index) = active_index {
										if !js_sys::Reflect::has(&event, &JsValue::from_str("key")).unwrap_or(false) {
											return;
										}
										match event.key().as_str() {
											"Tab" =>
												if !event.shift_key() && active == *tabbables.last().unwrap() {
													event.prevent_default();
													tabbables[0].focus().ok();
												} else if event.shift_key() && active == tabbables[0] {
													event.prevent_default();
													tabbables.last().unwrap().focus().ok();
												},
											"ArrowDown" | "ArrowRight" => {
												event.prevent_default();
												let next_index = (index + 1) % tabbables.len();
												tabbables[next_index].focus().ok();
											},
											"ArrowUp" | "ArrowLeft" => {
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

			if let Some(closure) = &*closure_focus_in_ref.read() {
				element.add_event_listener_with_callback("focusin", closure.as_ref().unchecked_ref()).expect("should register event listener");
			}

			if let Some(closure) = &*closure_keyboard_ref.read() {
				element.add_event_listener_with_callback("keydown", closure.as_ref().unchecked_ref()).expect("should register event listener");
			}
		}
	});

	rsx! {
		div {
			tabindex: "0",
			id: Uuid::new_v4().to_string(),
			onclick: move |event| {
					if let Some(handler) = onclick {
							handler.call(event);
					}
			},
			onmousedown: move |event| {
					if let Some(handler) = onmousedown {
							handler.call(event);
					}
			},
			onkeydown: move |event| {
					if let Some(handler) = onkeydown {
							handler.call(event);
					}
			},

			onkeyup: move |event| {
					if let Some(handler) = onkeyup {
							handler.call(event);
					}
			},
			onmouseup: move |event| {
					if let Some(handler) = onmouseup {
							handler.call(event);
					}
			},
			onmouseenter: move |event| {
					if let Some(handler) = onmouseenter {
							handler.call(event);
					}
			},
			onmouseleave: move |event| {
					if let Some(handler) = onmouseleave {
							handler.call(event);
					}
			},
			onfocus: move |event| {
					if let Some(handler) = onfocus {
							handler.call(event);
					}
			},
			onblur: move |event| {
					if let Some(handler) = onblur {
							handler.call(event);
					}
			},
			onmounted: move |event| {
					current_ref.set(Some(event.data()));
					let tabbables = get_tabbable_candidates(());
					if !tabbables.is_empty() {
							tabbables[0].focus().ok();
					}
					if let Some(handler) = onmounted {
							handler.call(event);
					}
			},
			..attrs,
			{children}
		}
	}
}
