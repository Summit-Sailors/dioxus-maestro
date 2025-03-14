use {
	crate::{
		button::Button,
		focus_trap::FocusTrap,
		hooks::{UseControllableStateParams, use_controllable_state, use_escape},
	},
	dioxus::prelude::*,
	std::fmt::Debug,
	uuid::Uuid,
	web_sys::window,
};

#[derive(Clone, PartialEq, Debug, Copy)]
struct DialogContext {
	pub open: Memo<Option<bool>>,
	pub on_close: Option<Callback>,
	pub set_open: Callback<Option<bool>>,
	pub content_id: Uuid,
}

impl DialogContext {
	pub fn new(open: Memo<Option<bool>>, on_close: Option<Callback>, set_open: Callback<Option<bool>>) -> Self {
		Self { open, on_close, set_open, content_id: Uuid::new_v4() }
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
	let context = use_context_provider::<DialogContext>(|| DialogContext::new(open, on_close, set_open));

	// TO DO: works on web
	use_effect(move || {
		let window = window().expect("should have a window in this context");
		let document = window.document().expect("window should have a document");
		let body = document.body().expect("document should have a body");

		if context.open.read().unwrap_or_default() {
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
	#[props(default = ReadOnlySignal::new(Signal::new(false)))]
	disabled: ReadOnlySignal<bool>,
	#[props(extends = GlobalAttributes, extends = button)]
	pub attributes: Vec<Attribute>,
	pub children: Element,
}

#[component]
pub fn DialogTrigger(props: DialogTriggerProps) -> Element {
	let DialogTriggerProps { attributes, disabled, .. } = props;
	let mut context = use_context::<DialogContext>();

	let mut attributes = attributes.clone();
	attributes.push(Attribute::new("aria-haspopup", "dialog", None, false));
	attributes.push(Attribute::new("aria-expanded", *context.open.read(), None, false));
	attributes.push(Attribute::new("data-state", if context.open.read().unwrap_or_default() { "open" } else { "closed" }, None, false));
	if !attributes.iter().any(|x| x.name == "title") {
		attributes.push(Attribute::new("title", "Open popup", None, false));
	}
	if !attributes.iter().any(|x| x.name == "aria-label") {
		attributes.push(Attribute::new("aria-label", "Open popup", None, false));
	}
	if !attributes.iter().any(|x| x.name == "aria-role") {
		attributes.push(Attribute::new("aria-role", "button", None, false));
	}
	if !attributes.iter().any(|x| x.name == "aria-controls") {
		attributes.push(Attribute::new("aria-controls", context.content_id.to_string(), None, false));
	}

	rsx! {
		Button {
			r#type: "button",
			onclick: move |_| context.toggle(true),
			disabled,
			additional_attributes: attributes.clone(),
			{props.children}
		}
	}
}

#[component]
pub fn DialogPortal(children: Element) -> Element {
	let context = use_context::<DialogContext>();

	if context.open.read().unwrap_or_default() {
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
	#[props(extends = GlobalAttributes)]
	pub attributes: Vec<Attribute>,
	pub children: Element,
}

#[component]
pub fn DialogOverlay(props: DialogOverlayProps) -> Element {
	let mut context = use_context::<DialogContext>();

	if context.open.read().unwrap_or_default() {
		rsx! {
			div {
				"data-state": if context.open.peek().unwrap_or_default() { "open" } else { "closed" },
				style: "pointer-events: auto;",
				onclick: move |_| context.toggle(false),
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
	#[props(extends = GlobalAttributes)]
	pub attributes: Vec<Attribute>,
	pub children: Element,
}

#[component]
pub fn DialogContent(props: DialogContentProps) -> Element {
	let mut context = use_context::<DialogContext>();
	let handle_close = use_callback(move |()| {
		context.toggle(false);
	});

	use_escape(handle_close, context.open);

	if context.open.read().unwrap_or_default() {
		rsx! {
			FocusTrap {
				div {
					role: "dialog",
					id: context.content_id.to_string(),
					"aria-modal": true,
					"data-state": if context.open.read().unwrap_or_default() { "open" } else { "closed" },
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
	#[props(extends = GlobalAttributes, extends=div)]
	pub attributes: Vec<Attribute>,
	pub children: Element,
}

#[component]
pub fn DialogHeader(props: DialogTitleProps) -> Element {
	rsx! {
		div { ..props.attributes,{props.children} }
	}
}

#[derive(Clone, PartialEq, Props)]
pub struct DialogFooterProps {
	#[props(extends = GlobalAttributes, extends=div)]
	pub attributes: Vec<Attribute>,
	pub children: Element,
}

#[component]
pub fn DialogFooter(props: DialogTitleProps) -> Element {
	rsx! {
		div { ..props.attributes,{props.children} }
	}
}

#[derive(Clone, PartialEq, Props)]
pub struct DialogBodyProps {
	#[props(extends = GlobalAttributes, extends=div)]
	pub attributes: Vec<Attribute>,
	pub children: Element,
}

#[component]
pub fn DialogBody(props: DialogTitleProps) -> Element {
	rsx! {
		div { ..props.attributes,{props.children} }
	}
}

#[derive(Clone, PartialEq, Props)]
pub struct DialogTitleProps {
	#[props(extends = GlobalAttributes)]
	pub attributes: Vec<Attribute>,
	pub children: Element,
}
#[component]
pub fn DialogTitle(props: DialogTitleProps) -> Element {
	rsx! {
		h2 { ..props.attributes,{props.children} }
	}
}

#[derive(Clone, PartialEq, Props)]
pub struct DialogDescriptionProps {
	#[props(extends = GlobalAttributes, extends=div)]
	pub attributes: Vec<Attribute>,
	pub children: Element,
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
	let mut context = use_context::<DialogContext>();
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
			onclick: move |_| context.toggle(false),
			additional_attributes: attributes.clone(),
			{props.children}
		}
	}
}
