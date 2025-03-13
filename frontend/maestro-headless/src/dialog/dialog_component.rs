use {
	crate::{
		button::Button,
		focus_trap::FocusTrap,
		hooks::{UseControllableStateParams, use_controllable_state, use_escape},
	},
	dioxus::prelude::*,
	std::fmt::Debug,
	web_sys::window,
};

#[derive(Clone, PartialEq, Debug, Copy)]
struct DialogContext {
	pub open: Memo<Option<bool>>,
	pub on_close: Option<Callback>,
	pub set_open: Callback<Option<bool>>,
}

impl DialogContext {
	pub fn new(open: Memo<Option<bool>>, on_close: Option<Callback>, set_open: Callback<Option<bool>>) -> Self {
		Self { open, on_close, set_open }
	}

	pub fn toggle(&mut self, value: bool) {
		self.set_open.call(Some(value));
		if let Some(onclose) = self.on_close {
			if !value {
				onclose.call(());
			}
		}
	}
}

#[derive(Props, Clone, PartialEq)]
pub struct DialogProps {
	#[props(optional, default = ReadOnlySignal::new(Signal::new(None)))]
	pub open: ReadOnlySignal<Option<bool>>,
	#[props(optional, default = false)]
	pub default_open: bool,
	#[props(optional)]
	pub on_open_change: Option<Callback<Option<bool>>>,
	#[props(optional)]
	pub on_close: Option<Callback>, // callback will trigger always when dialog close
	pub children: Element,
}

#[component]
pub fn Dialog(props: DialogProps) -> Element {
	let DialogProps { open, default_open, on_close, on_open_change, children } = props;
	let is_controlled = use_hook(move || open().is_some());
	let (open, set_open) =
		use_controllable_state(UseControllableStateParams { is_controlled, prop: open, default_prop: default_open, on_change: on_open_change });
	let dialog_context = use_context_provider::<DialogContext>(|| DialogContext::new(open, on_close, set_open));

	// TO DO: works on web
	use_effect(move || {
		let window = window().expect("should have a window in this context");
		let document = window.document().expect("window should have a document");
		let body = document.body().expect("document should have a body");

		if dialog_context.open.read().unwrap_or_default() {
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
	#[props(default = ReadOnlySignal::new(Signal::new(false)))]
	disabled: ReadOnlySignal<bool>,
}

#[component]
pub fn DialogTrigger(props: DialogTriggerProps) -> Element {
	let DialogTriggerProps { attributes, disabled, .. } = props;
	let mut dialog_context = use_context::<DialogContext>();

	let mut attributes = attributes.clone();
	attributes.push(Attribute::new("aria-haspopup", "dialog", None, false));
	attributes.push(Attribute::new("aria-expanded", *dialog_context.open.read(), None, false));
	attributes.push(Attribute::new("data-state", if dialog_context.open.read().unwrap_or_default() { "open" } else { "closed" }, None, false));
	if !attributes.iter().any(|x| x.name == "title") {
		attributes.push(Attribute::new("title", "Open popup", None, false));
	}
	if !attributes.iter().any(|x| x.name == "aria-label") {
		attributes.push(Attribute::new("aria-label", "Open popup", None, false));
	}
	if !attributes.iter().any(|x| x.name == "aria-role") {
		attributes.push(Attribute::new("aria-role", "button", None, false));
	}

	rsx! {
		Button {
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

	if dialog_context.open.read().unwrap_or_default() {
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

	if dialog_context.open.read().unwrap_or_default() {
		rsx! {
			div {
				"data-state": if dialog_context.open.peek().unwrap_or_default() { "open" } else { "closed" },
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
	let handle_close = use_callback(move |()| {
		dialog_context.toggle(false);
	});

	use_escape(handle_close, dialog_context.open);

	if dialog_context.open.read().unwrap_or_default() {
		rsx! {
			FocusTrap {
				div {
					role: "dialog",
					"aria-modal": true,
					"data-state": if dialog_context.open.read().unwrap_or_default() { "open" } else { "closed" },
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
	let mut attributes = props.attributes.clone();
	if !attributes.iter().any(|x| x.name == "title") {
		attributes.push(Attribute::new("title", "Close popup", None, false));
	}
	if !attributes.iter().any(|x| x.name == "aria-label") {
		attributes.push(Attribute::new("aria-label", "Close popup", None, false));
	}
	if !attributes.iter().any(|x| x.name == "aria-role") {
		attributes.push(Attribute::new("aria-role", "button", None, false));
	}
	rsx! {
		Button {
			r#type: "button",
			onclick: move |_| dialog_context.toggle(false),
			additional_attributes: attributes.clone(),
			{props.children}
		}
	}
}
