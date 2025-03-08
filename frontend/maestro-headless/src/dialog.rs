use {
	crate::{
		button::{use_button::use_button, Button},
		hooks::use_attributes,
	},
	dioxus::prelude::*,
	std::fmt::Debug,
	web_sys::window,
};

#[derive(Clone, PartialEq, Debug, Copy)]
struct DialogContext {
	pub open: Signal<bool>,
	pub onclose: Option<Callback>,
	pub on_open_change: Option<Callback<bool>>,
}

impl DialogContext {
	pub fn new(open: Signal<bool>, onclose: Option<Callback>, on_open_change: Option<Callback<bool>>) -> Self {
		Self { open, onclose, on_open_change }
	}

	pub fn toggle(&mut self, value: bool) {
		if let Some(onclose) = self.onclose {
			if !value {
				onclose.call(());
			}
		}
		if let Some(on_open_change) = self.on_open_change {
			on_open_change.call(value);
		}
		self.open.set(value);
	}
}

#[derive(Props, Clone, PartialEq)]
pub struct DialogProps {
	#[props(optional, default = Signal::new(false))]
	pub open: Signal<bool>, // signal allows user to control dialod state (for example, in some callbacks ets)
	#[props(optional)]
	pub onclose: Option<Callback>, // callback will trigger always when dialog close
	#[props(optional)]
	pub on_open_change: Option<Callback<bool>>, // callback will trigger always when dialog close
	pub children: Element,
}

#[component]
pub fn Dialog(props: DialogProps) -> Element {
	let DialogProps { open, onclose, on_open_change, children } = props;
	let dialog_context = use_context_provider::<DialogContext>(|| DialogContext::new(open, onclose, on_open_change));

	use_effect(move || {
		let window = window().expect("should have a window in this context");
		let document = window.document().expect("window should have a document");
		let body = document.body().expect("document should have a body");

		if *dialog_context.open.read() {
			body.set_class_name("overflow-hidden");
		} else {
			body.set_class_name("overflow-[unset]");
		}
	});

	rsx! {
		{children}
	}
}

// TO DO!!! Impossible to pass attributes to child, used a hack (custom hook)

#[derive(Clone, PartialEq, Props)]
pub struct DialogTriggerProps {
	pub children: Element,
	#[props(extends = GlobalAttributes, extends = button)]
	pub attributes: Vec<Attribute>,
	#[props(default = false)]
	disabled: bool,
}

#[component]
pub fn DialogTrigger(props: DialogTriggerProps) -> Element {
	let DialogTriggerProps { attributes, disabled, .. } = props;
	let mut dialog_context = use_context::<DialogContext>();
	let button_context = use_button(false, disabled);
	use_attributes(button_context.self_ref, attributes.clone());

	rsx! {
		Button {
			context: button_context,
			r#type: "button",
			aria_haspopup: "dialog",
			aria_expanded: *dialog_context.open.read(),
			"data-state": if *dialog_context.open.read() { "open" } else { "closed" },
			onclick: move |_| dialog_context.toggle(true),
			disabled: props.disabled,
			{props.children}
		}
	}
}

#[component]
pub fn DialogPortal(children: Element) -> Element {
	let dialog_context = use_context::<DialogContext>();

	if *dialog_context.open.read() {
		rsx! {
			Fragment { {children} }
		}
	} else {
		rsx! {
			Fragment {}
		}
	}
}

#[derive(Clone, PartialEq, Props)]
pub struct DialogOverlayProps {
	pub children: Element,
	#[props(extends = GlobalAttributes)]
	pub attributes: Vec<Attribute>,
}

#[component]
pub fn DialogOverlay(props: DialogOverlayProps) -> Element {
	let mut dialog_context = use_context::<DialogContext>();

	if *dialog_context.open.read() {
		rsx! {
			div {
				"data-state": if *dialog_context.open.peek() { "open" } else { "closed" },
				style: "pointer-events: auto;",
				onclick: move |_| dialog_context.toggle(false),
				..props.attributes,
				{props.children}
			}
		}
	} else {
		rsx! {}
	}
}

#[derive(Clone, PartialEq, Props)]
pub struct DialogContentProps {
	pub children: Element,
	#[props(extends = GlobalAttributes)]
	pub attributes: Vec<Attribute>,
}

#[component]
pub fn DialogContent(props: DialogContentProps) -> Element {
	let mut dialog_context = use_context::<DialogContext>();
	let mut content_element = use_signal::<Option<std::rc::Rc<dioxus::events::MountedData>>>(|| None);

	use_effect(move || {
		spawn(async move {
			if *dialog_context.open.peek() {
				if let Some(element) = content_element() {
					let _ = element.set_focus(true).await;
				}
			}
		});
	});

	if *dialog_context.open.read() {
		rsx! {
			div {
				role: "dialog",
				"data-state": if *dialog_context.open.read() { "open" } else { "closed" },
				tabindex: "0",
				onkeydown: move |evt: KeyboardEvent| {
						if evt.key() == Key::Escape {
								dialog_context.toggle(false);
						}
				},
				onmounted: move |el| {
						content_element.set(Some(el.data()));
				},
				..props.attributes,
				{props.children}
			}
		}
	} else {
		rsx! {}
	}
}

#[derive(Clone, PartialEq, Props)]
pub struct DialogTitleProps {
	pub children: Element,
	#[props(extends = GlobalAttributes)]
	pub attributes: Vec<Attribute>,
}

#[component]
pub fn DialogTitle(props: DialogTitleProps) -> Element {
	rsx! {
		h2 { ..props.attributes,{props.children} }
	}
}

#[derive(Clone, PartialEq, Props)]
pub struct DialogDescriptionProps {
	pub children: Element,
	#[props(extends = GlobalAttributes)]
	pub attributes: Vec<Attribute>,
}

#[component]
pub fn DialogDescription(props: DialogDescriptionProps) -> Element {
	rsx! {
		div { ..props.attributes,{props.children} }
	}
}

#[derive(Clone, PartialEq, Props)]
pub struct DialogCloseProps {
	pub children: Element,
	#[props(extends = GlobalAttributes, extends = button)]
	pub attributes: Vec<Attribute>,
}

#[component]
pub fn DialogClose(props: DialogCloseProps) -> Element {
	let mut dialog_context = use_context::<DialogContext>();
	let button_context = use_button(false, false);
	use_attributes(button_context.self_ref, props.attributes.clone());
	rsx! {
		Button {
			r#type: "button",
			onclick: move |_| dialog_context.toggle(false),
			context: button_context,
			{props.children}
		}
	}
}
