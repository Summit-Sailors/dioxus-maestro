use {dioxus::prelude::*, web_sys::window};

#[derive(Clone, PartialEq, Debug)]
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

#[derive(Clone, PartialEq, Props)]
pub struct DialogTriggerProps {
	pub children: Element,
	#[props(extends = GlobalAttributes, extends = button)]
	pub attributes: Vec<Attribute>,
}

#[component]
pub fn DialogTrigger(props: DialogTriggerProps) -> Element {
	let mut dialog_context = use_context::<DialogContext>();

	rsx! {
		button {
			r#type: "button",
			aria_haspopup: "dialog",
			aria_expanded: *dialog_context.open.clone().read(),
			"data-state": if *dialog_context.open.clone().read() { "open" } else { "closed" },
			onclick: move |_| dialog_context.toggle(true),
			..props.attributes,
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
	let dialog_context = use_context::<DialogContext>();

	if *dialog_context.open.read() {
		rsx! {
			div {
				role: "dialog",
				"data-state": if *dialog_context.open.read() { "open" } else { "closed" },
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

	rsx! {
		button {
			r#type: "button",
			onclick: move |_| dialog_context.toggle(false),
			..props.attributes,
			{props.children}
		}
	}
}
