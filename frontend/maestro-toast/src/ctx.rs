use {crate::toast_manager::ToastManager, dioxus::prelude::*};

pub fn use_toast() -> Signal<ToastManager> {
	use_context::<Signal<ToastManager>>()
}
