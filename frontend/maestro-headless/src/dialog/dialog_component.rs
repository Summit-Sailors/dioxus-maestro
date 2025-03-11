use {
	crate::{
		button::{Button, use_button},
		focus_trap::FocusTrap,
		hooks::use_outside_key_down,
	},
	dioxus::prelude::*,
	std::{fmt::Debug, rc::Rc},
	web_sys::window,
};

#[derive(Clone, PartialEq, Debug, Copy)]
struct DialogContext {
	pub open: Signal<bool>,
	pub on_close: Option<Callback>,
	pub on_open_change: Option<Callback<bool>>,
}

impl DialogContext {
	pub fn new(open: Signal<bool>, on_close: Option<Callback>, on_open_change: Option<Callback<bool>>) -> Self {
		Self { open, on_close, on_open_change }
	}

	pub fn toggle(&mut self, value: bool) {
		self.open.set(value);
		if let Some(onclose) = self.on_close {
			if !value {
				onclose.call(());
			}
		}
		if let Some(onopenchange) = self.on_open_change {
			onopenchange.call(value);
		}
	}
}

#[derive(Props, Clone, PartialEq)]
pub struct DialogProps {
	#[props(optional, default = Signal::new(false))]
	pub open: Signal<bool>, // accepts signal (better state management inside component and outside)
	#[props(optional)]
	pub on_open_change: Option<Callback<bool>>,
	#[props(optional)]
	pub on_close: Option<Callback>, // callback will trigger always when dialog close
	pub children: Element,
}

#[component]
pub fn Dialog(props: DialogProps) -> Element {
	let DialogProps { open, on_close, on_open_change, children } = props;
	let dialog_context = use_context_provider::<DialogContext>(|| DialogContext::new(open, on_close, on_open_change));

	// TO DO: works on web
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
	#[props(default = false)]
	disabled: bool,
}

#[component]
pub fn DialogTrigger(props: DialogTriggerProps) -> Element {
	let DialogTriggerProps { attributes, disabled, .. } = props;
	let mut dialog_context = use_context::<DialogContext>();
	let button_context = use_button(false, disabled);

	let mut attributes = attributes.clone();
	attributes.push(Attribute::new("aria_haspopup", "dialog", None, false));
	attributes.push(Attribute::new("aria_expanded", *dialog_context.open.read(), None, false));
	attributes.push(Attribute::new("data-state", if *dialog_context.open.read() { "open" } else { "closed" }, None, false));
	if !attributes.iter().any(|x| x.name == "title") {
		attributes.push(Attribute::new("title", "Open popup", None, false));
	}
	if !attributes.iter().any(|x| x.name == "aria_label") {
		attributes.push(Attribute::new("aria_label", "Open popup", None, false));
	}
	if !attributes.iter().any(|x| x.name == "aria_role") {
		attributes.push(Attribute::new("aria_role", "button", None, false));
	}

	rsx! {
		Button {
			context: button_context,
			r#type: "button",
			onclick: move |_| dialog_context.toggle(true),
			disabled,
			additional_attributes: attributes.clone(),
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
	let mut current_ref = use_signal(|| None::<Rc<MountedData>>);
	let handle_close = use_callback(move |()| {
		dialog_context.toggle(false);
	});

	use_outside_key_down(current_ref, handle_close);

	if *dialog_context.open.read() {
		rsx! {
			FocusTrap {
				div {
					role: "dialog",
					"aria-modal": true,
					"data-state": if *dialog_context.open.read() { "open" } else { "closed" },
					onmounted: move |event| current_ref.set(Some(event.data())),
					..props.attributes,
					{props.children}
				}
			}
		}
	} else {
		rsx! {}
	}
}

#[derive(Clone, PartialEq, Props)]
pub struct DialogHeaderProps {
	pub children: Element,
	#[props(extends = GlobalAttributes, extends=div)]
	pub attributes: Vec<Attribute>,
}

#[component]
pub fn DialogHeader(props: DialogTitleProps) -> Element {
	rsx! {
		div { ..props.attributes,{props.children} }
	}
}

#[derive(Clone, PartialEq, Props)]
pub struct DialogFooterProps {
	pub children: Element,
	#[props(extends = GlobalAttributes, extends=div)]
	pub attributes: Vec<Attribute>,
}

#[component]
pub fn DialogFooter(props: DialogTitleProps) -> Element {
	rsx! {
		div { ..props.attributes,{props.children} }
	}
}

#[derive(Clone, PartialEq, Props)]
pub struct DialogBodyProps {
	pub children: Element,
	#[props(extends = GlobalAttributes, extends=div)]
	pub attributes: Vec<Attribute>,
}

#[component]
pub fn DialogBody(props: DialogTitleProps) -> Element {
	rsx! {
		div { ..props.attributes,{props.children} }
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
	#[props(extends = GlobalAttributes, extends=div)]
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
	let mut attributes = props.attributes.clone();
	if !attributes.iter().any(|x| x.name == "title") {
		attributes.push(Attribute::new("title", "Close popup", None, false));
	}
	if !attributes.iter().any(|x| x.name == "aria_label") {
		attributes.push(Attribute::new("aria_label", "Close popup", None, false));
	}
	if !attributes.iter().any(|x| x.name == "aria_role") {
		attributes.push(Attribute::new("aria_role", "button", None, false));
	}
	rsx! {
		Button {
			r#type: "button",
			onclick: move |_| dialog_context.toggle(false),
			context: button_context,
			additional_attributes: attributes.clone(),
			{props.children}
		}
	}
}
