use wasm_bindgen_futures::JsFuture;
use web_sys::window;

#[derive(Clone, Debug, PartialEq)]
pub enum ClipboardError {
	FailedToRead,
	FailedToSet,
	NotAvailable,
}

#[derive(Clone, Copy)]
pub struct UseClipboard;

impl UseClipboard {
	pub async fn get(&self) -> Result<String, ClipboardError> {
		JsFuture::from(window().ok_or(ClipboardError::NotAvailable)?.navigator().clipboard().read_text())
			.await
			.map_err(|_| ClipboardError::FailedToRead)?
			.as_string()
			.ok_or(ClipboardError::FailedToRead)
	}

	pub async fn set(&self, contents: String) -> Result<(), ClipboardError> {
		JsFuture::from(window().ok_or(ClipboardError::NotAvailable)?.navigator().clipboard().write_text(&contents))
			.await
			.map_err(|_| ClipboardError::FailedToSet)?;
		Ok(())
	}
}

pub fn use_clipboard() -> UseClipboard {
	UseClipboard
}
