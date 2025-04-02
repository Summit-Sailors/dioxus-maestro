use {
	async_std::task::sleep,
	dioxus::{prelude::*, web::WebEventExt},
	std::{rc::Rc, time::Duration},
	web_sys::{HtmlElement, wasm_bindgen::JsCast},
};

pub fn use_dimensions(current_ref: Signal<Option<Rc<MountedData>>>, is_open: bool) -> (Signal<f32>, Signal<f32>) {
	let mut width = use_signal(|| 0.0_f32);
	let mut height = use_signal(|| 0.0_f32);
	let mut is_mount_animation_prevented = use_signal(|| is_open);
	let mut original_styles = use_signal(|| None::<(String, String)>);

	use_effect(move || {
		spawn(async move {
			sleep(Duration::from_millis(10)).await;
			is_mount_animation_prevented.set(false);
		});
	});

	use_effect(move || {
		if let Some(data) = current_ref() {
			if let Some(element) = data.try_as_web_event().and_then(|x| x.dyn_into::<HtmlElement>().ok()) {
				if original_styles.peek().is_none() {
					let style = element.style();
					original_styles
						.set(Some((style.get_property_value("transition-duration").unwrap_or_default(), style.get_property_value("animation-name").unwrap_or_default())));
				}

				let style = element.style();
				style.set_property("transition-duration", "0s").ok();
				style.set_property("animation-name", "none").ok();

				let rect = element.get_bounding_client_rect();
				width.set(rect.width() as f32);
				height.set(rect.height() as f32);

				if !is_mount_animation_prevented() {
					if let Some((transition_duration, animation_name)) = original_styles() {
						style.set_property("transition-duration", &transition_duration).ok();
						style.set_property("animation-name", &animation_name).ok();
					}
				}
			}
		}
	});

	(width, height)
}
