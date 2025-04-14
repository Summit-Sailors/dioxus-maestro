use dioxus::prelude::*;

use crate::toast_manager::ToastManager;

pub fn use_toast() -> Signal<ToastManager> {
	use_context::<Signal<ToastManager>>()
}
