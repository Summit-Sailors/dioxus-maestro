#[cfg(feature = "desktop")]
use dioxus::prelude::*;

#[derive(Debug, Clone, PartialEq)]
pub enum ClipboardError {
	FailedToRead,
	FailedToSet,
	NotAvailable,
}

#[derive(Clone)]
pub struct UseClipboard {
	#[cfg(feature = "desktop")]
	clipboard: Signal<Option<copypasta::ClipboardContext>>,
}

impl UseClipboard {
	pub async fn get(&mut self) -> Result<String, ClipboardError> {
		#[cfg(feature = "desktop")]
		{
			use copypasta::ClipboardProvider;
			self.clipboard.write().as_mut().ok_or(ClipboardError::NotAvailable)?.get_contents().map_err(|_| ClipboardError::FailedToRead)
		}

		#[cfg(not(feature = "desktop"))]
		{
			use {wasm_bindgen_futures::JsFuture, web_sys::window};
			JsFuture::from(window().ok_or(ClipboardError::NotAvailable)?.navigator().clipboard().read_text())
				.await
				.map_err(|_| ClipboardError::FailedToRead)?
				.as_string()
				.ok_or(ClipboardError::FailedToRead)
		}
	}

	pub async fn set(&mut self, contents: String) -> Result<(), ClipboardError> {
		#[cfg(feature = "desktop")]
		{
			use copypasta::ClipboardProvider;
			self.clipboard.write().as_mut().ok_or(ClipboardError::NotAvailable)?.set_contents(contents).map_err(|_| ClipboardError::FailedToSet)
		}

		#[cfg(not(feature = "desktop"))]
		{
			use {wasm_bindgen_futures::JsFuture, web_sys::window};
			JsFuture::from(window().ok_or(ClipboardError::NotAvailable)?.navigator().clipboard().write_text(&contents))
				.await
				.map_err(|_| ClipboardError::FailedToSet)?;
			Ok(())
		}
	}
}

pub fn use_clipboard() -> UseClipboard {
	#[cfg(feature = "desktop")]
	{
		use copypasta::ClipboardContext;
		let clipboard = match try_consume_context() {
			Some(rt) => rt,
			None => {
				let clipboard_signal = Signal::new_in_scope(ClipboardContext::new().ok(), ScopeId::ROOT);
				provide_root_context(clipboard_signal)
			},
		};
		UseClipboard { clipboard }
	}

	#[cfg(not(feature = "desktop"))]
	UseClipboard {}
}

