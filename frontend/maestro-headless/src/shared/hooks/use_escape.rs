use {
	dioxus::prelude::*,
	web_sys::{
		AddEventListenerOptions, js_sys,
		wasm_bindgen::{JsCast, JsValue, prelude::Closure},
		window,
	},
};

pub fn use_escape(on_outside_key: Callback<()>, flag: Memo<bool>) {
	let mut closure_ref = use_signal(|| None::<Closure<dyn FnMut(web_sys::KeyboardEvent)>>);

	use_effect(move || {
		let window = window().expect("should have a window in this context");
		let document = window.document().expect("window should have a document");

		if closure_ref.peek().is_none() && flag() {
			let closure = Closure::wrap(Box::new(move |event: web_sys::KeyboardEvent| {
				if !js_sys::Reflect::has(&event, &JsValue::from_str("key")).unwrap_or(false) {
					return;
				}
				if event.key().as_str() == "Escape" {
					event.prevent_default();
					on_outside_key.call(());
				}
			}) as Box<dyn FnMut(web_sys::KeyboardEvent)>);
			closure_ref.set(Some(closure));
		}

		if flag() {
			if let Some(closure) = &*closure_ref.read() {
				let options = AddEventListenerOptions::new();

				document
					.add_event_listener_with_callback_and_add_event_listener_options("keydown", closure.as_ref().unchecked_ref(), options.as_ref())
					.expect("should register event listener");
			}
		}
	});

	use_effect(move || {
		if !flag() {
			if let Some(closure) = closure_ref.peek().as_ref() {
				let window = window().expect("should have a window in this context");
				let document = window.document().expect("window should have a document");
				document.remove_event_listener_with_callback("keydown", closure.as_ref().unchecked_ref()).expect("should remove event listener");
			}
			closure_ref.set(None);
		}
	});

	{
		use_drop(move || {
			if let Some(closure) = closure_ref.peek().as_ref() {
				let window = window().expect("should have a window in this context");
				let document = window.document().expect("window should have a document");

				document.remove_event_listener_with_callback("keydown", closure.as_ref().unchecked_ref()).expect("should remove event listener");
			}
		});
	}
}
