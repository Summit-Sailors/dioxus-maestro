use dioxus::prelude::*;

#[derive(Debug, PartialEq, Clone, Copy)]
pub struct InteractionStateContext {
	pub is_pressed: Signal<bool>,
	pub is_hovered: Signal<bool>,
	pub is_focused: Signal<bool>,
	pub pending: ReadOnlySignal<bool>,
	pub disabled: ReadOnlySignal<bool>,
	pub self_ref: Signal<Option<Event<MountedData>>>,
}

impl InteractionStateContext {
	pub fn new(
		disabled: ReadOnlySignal<bool>,
		pending: ReadOnlySignal<bool>,
		is_pressed: Signal<bool>,
		is_hovered: Signal<bool>,
		is_focused: Signal<bool>,
		self_ref: Signal<Option<Event<MountedData>>>,
	) -> Self {
		Self { pending, disabled, is_focused, is_hovered, is_pressed, self_ref }
	}

	pub fn is_allowed(&self) -> bool {
		!*self.pending.peek() && !*self.disabled.peek()
	}

	pub fn onmouseup(&mut self) {
		self.is_pressed.set(false);
	}

	pub fn onkeydown(&mut self) {
		self.is_pressed.set(true);
	}

	pub fn onkeyup(&mut self) {
		self.is_pressed.set(false);
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

	pub fn onmousedown(&mut self) {
		if self.is_allowed() {
			self.is_pressed.set(true);
		}
	}

	pub fn onmouseenter(&mut self) {
		if self.is_allowed() {
			self.is_hovered.set(true);
		}
	}
}

pub fn use_interaction_state(pending: ReadOnlySignal<bool>, disabled: ReadOnlySignal<bool>) -> InteractionStateContext {
	let is_pressed = use_signal(|| false);
	let is_hovered = use_signal(|| false);
	let is_focused = use_signal(|| false);
	let self_ref = use_signal::<Option<Event<MountedData>>>(|| None);
	use_context_provider(|| InteractionStateContext::new(disabled, pending, is_pressed, is_hovered, is_focused, self_ref))
}
