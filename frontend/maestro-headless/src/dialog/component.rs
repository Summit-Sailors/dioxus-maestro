use {
	crate::{
		button::Button,
		focus_trap::FocusTrap,
		presence::Presence,
		shared::{UseControllableStateParams, use_controllable_state, use_escape},
	},
	dioxus::prelude::*,
	std::{fmt::Debug, rc::Rc},
	uuid::Uuid,
	web_sys::window,
};

#[derive(Clone, PartialEq, Debug, Copy)]
struct DialogContext {
	pub open: Memo<bool>,
	pub on_close: Option<Callback>,
	pub set_open: Callback<bool>,
	pub content_id: Uuid,
	pub trigger_id: Uuid,
}

impl DialogContext {
	pub fn new(open: Memo<bool>, on_close: Option<Callback>, set_open: Callback<bool>) -> Self {
		Self { open, on_close, set_open, content_id: Uuid::new_v4(), trigger_id: Uuid::new_v4() }
	}

	pub fn toggle(&mut self, value: bool) {
		self.set_open.call(value);
		if let Some(onclose) = self.on_close {
			if !value {
				onclose.call(());
			}
		}
	}
}

#[derive(Props, Clone, PartialEq)]
pub struct DialogRootProps {
	#[props(optional, default = ReadOnlySignal::new(Signal::new(None)))]
	pub open: ReadOnlySignal<Option<bool>>,
	#[props(optional, default = false)]
	pub default_open: bool,
	#[props(optional)]
	pub on_open_change: Option<Callback<bool>>,
	#[props(optional)]
	pub on_close: Option<Callback>, // callback will trigger always when dialog close
	pub children: Element,
}

#[component]
pub fn DialogRoot(props: DialogRootProps) -> Element {
	let DialogRootProps { open, default_open, on_close, on_open_change, children } = props;

	let is_controlled = use_hook(move || open().is_some());
	let (open, set_open) =
		use_controllable_state(UseControllableStateParams { is_controlled, prop: open, default_prop: default_open, on_change: on_open_change });
	use_context_provider::<DialogContext>(|| DialogContext::new(open, on_close, set_open));

	use_effect(move || {
		let window = window().expect("should have a window in this context");
		let document = window.document().expect("window should have a document");
		let body = document.body().expect("document should have a body");

		if open() {
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
	#[props(optional)]
	pub children: Element,
}

#[component]
pub fn DialogTrigger(props: DialogTriggerProps) -> Element {
	let DialogTriggerProps { attributes, disabled, children } = props;
	let mut context = use_context::<DialogContext>();

	rsx! {
		Button {
			id: context.trigger_id.to_string(),
			r#type: "button",
			onclick: move |_| context.toggle(true),
			disabled,
			aria_haspopup: "dialog",
			aria_expanded: *context.open.read(),
			aria_controls: context.content_id.to_string(),
			"data-state": if *context.open.read() { "open" } else { "closed" },
			extra_attributes: attributes.clone(),
			{children}
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
	let mut node_ref = use_signal(|| None::<Rc<MountedData>>);

	rsx! {
		Presence { node_ref, present: *context.open.read(),
			div {
				"data-state": if *context.open.peek() { "open" } else { "closed" },
				pointer_events: "auto",
				onmounted: move |event| node_ref.set(Some(event.data())),
				onclick: move |_| context.toggle(false),
				..props.attributes,
				{props.children}
			}
		}
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
	let mut node_ref = use_signal(|| None::<Rc<MountedData>>);

	let handle_close = use_callback(move |()| {
		context.toggle(false);
	});

	use_escape(handle_close, context.open);

	rsx! {
		Presence { node_ref, present: *context.open.read(),
			FocusTrap {
				div {
					role: "dialog",
					id: context.content_id.to_string(),
					aria_modal: true,
					aria_labelledby: context.trigger_id.to_string(),
					"data-state": if *context.open.read() { "open" } else { "closed" },
					onmounted: move |event| node_ref.set(Some(event.data())),
					..props.attributes,
					{props.children}
				}
			}
		}
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

	rsx! {
		Button {
			r#type: "button",
			aria_label: "Close popup",
			onclick: move |_| context.toggle(false),
			extra_attributes: props.attributes.clone(),
			{props.children}
		}
	}
}
