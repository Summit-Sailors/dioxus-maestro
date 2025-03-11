use {
	dioxus::{prelude::*, web::WebEventExt},
	std::rc::Rc,
	web_sys::{
		AddEventListenerOptions, EventTarget, HtmlElement,
		wasm_bindgen::{JsCast, prelude::Closure},
		window,
	},
};

pub fn use_outside_click(node_ref: Signal<Option<Rc<MountedData>>>, on_outside_click: Callback<()>) {
	let mut closure_ref = use_signal(|| None::<Closure<dyn FnMut(web_sys::MouseEvent)>>);

	use_effect(move || {
		let window = window().expect("should have a window in this context");
		let document = window.document().expect("window should have a document");

		if closure_ref.peek().is_none() {
			let closure = Closure::wrap(Box::new(move |event: web_sys::MouseEvent| {
				if let Some(node) = node_ref.read().as_ref() {
					let node = node.try_as_web_event();
					if let Some(node) = node {
						if let Some(target) = event.target().and_then(|t: EventTarget| t.dyn_into::<HtmlElement>().ok()) {
							if !node.contains(Some(&target)) {
								on_outside_click.call(());
							}
						}
					}
				}
			}) as Box<dyn FnMut(web_sys::MouseEvent)>);
			closure_ref.set(Some(closure));
		}

		if let Some(closure) = &*closure_ref.read() {
			let options = AddEventListenerOptions::new();
			options.set_capture(true);

			document
				.add_event_listener_with_callback_and_add_event_listener_options("mousedown", closure.as_ref().unchecked_ref(), options.as_ref())
				.expect("should register event listener");
		}
	});

	use_drop(move || {
		if let Some(closure) = closure_ref.peek().as_ref() {
			let window = window().expect("should have a window in this context");
			let document = window.document().expect("window should have a document");

			document.remove_event_listener_with_callback("mousedown", closure.as_ref().unchecked_ref()).expect("should remove event listener");
			drop(closure);
		}
	});
}
