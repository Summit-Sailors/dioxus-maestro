use dioxus::prelude::*;

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct InteractionStateContext {
	pub is_hovered: Signal<bool>,
	pub is_focused: Signal<bool>,
	pub self_ref: Signal<Option<Event<MountedData>>>,
}

impl InteractionStateContext {
	pub fn new(is_hovered: Signal<bool>, is_focused: Signal<bool>, self_ref: Signal<Option<Event<MountedData>>>) -> Self {
		Self { is_focused, is_hovered, self_ref }
	}

	pub fn onmouseleave(&mut self) {
		self.is_hovered.set(false);
	}

	pub fn onfocus(&mut self) {
		self.is_focused.set(true);
	}

	pub fn onblur(&mut self) {
		self.is_focused.set(false);
	}

	pub fn onmouseenter(&mut self) {
		self.is_hovered.set(true);
	}
}

pub fn use_interaction_state() -> InteractionStateContext {
	let is_hovered = use_signal(|| false);
	let is_focused = use_signal(|| false);
	let self_ref = use_signal::<Option<Event<MountedData>>>(|| None);
	use_context_provider(|| InteractionStateContext::new(is_hovered, is_focused, self_ref))
}
