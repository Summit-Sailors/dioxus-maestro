use {
	crate::{id::ToastID, toast_info::ToastInfo},
	dioxus_logger::tracing::debug,
	std::collections::BTreeMap,
};

#[derive(Clone, Debug)]
pub struct ToastManagerItem {
	pub info: ToastInfo,
	pub hide_after: i64,
}

#[derive(Debug)]
pub struct ToastManager {
	pub list: BTreeMap<usize, ToastManagerItem>,
	maximum_toast: u8,
	id_manager: ToastID,
}

impl Default for ToastManager {
	fn default() -> Self {
		Self { list: Default::default(), maximum_toast: 6, id_manager: ToastID::default() }
	}
}

impl ToastManager {
	pub fn new(maximum_toast: u8) -> Self {
		Self { list: BTreeMap::new(), maximum_toast, id_manager: ToastID::new() }
	}

	pub fn popup(&mut self, info: ToastInfo) -> usize {
		let toast_id = self.id_manager.add();
		if self.list.len() >= self.maximum_toast.into() {
			if let Some(result) = self.list.first_key_value() {
				let id = *result.0;
				debug!("Deleted Toast ID: {:?}", id);
				self.list.remove(&id);
			}
		}
		let hide_after = chrono::Local::now().timestamp() + info.hide_after as i64;
		self.list.insert(toast_id, ToastManagerItem { info, hide_after });
		toast_id
	}

	pub fn remove(&mut self, id: usize) {
		self.list.remove(&id);
	}

	pub fn clear(&mut self) {
		self.list.clear();
	}
}
