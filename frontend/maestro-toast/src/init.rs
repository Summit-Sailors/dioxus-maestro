use dioxus::prelude::*;

use crate::toast_manager::ToastManager;

pub fn use_init_toast_ctx() -> Signal<ToastManager> {
	use_context_provider(|| Signal::new(ToastManager::default()))
}
