use {
	crate::{
		button::Button,
		hooks::{UseControllableStateParams, use_controllable_state, use_escape, use_outside_click},
		popper::{Popper, PopperAnchor, PopperArrow, PopperContent},
		utils::{EAlign, ESide},
	},
	dioxus::prelude::*,
	std::{fmt::Debug, rc::Rc},
	uuid::Uuid,
};

#[derive(Clone, PartialEq, Debug, Copy)]
struct PopoverContext {
	pub open: Memo<Option<bool>>,
	pub set_open: Callback<Option<bool>>,
	pub content_id: Uuid,
	pub trigger_id: Uuid,
}

impl PopoverContext {
	pub fn new(open: Memo<Option<bool>>, set_open: Callback<Option<bool>>) -> Self {
		Self { open, set_open, content_id: Uuid::new_v4(), trigger_id: Uuid::new_v4() }
	}
}

#[derive(Props, Clone, PartialEq)]
pub struct PopoverProps {
	#[props(optional, default = ReadOnlySignal::new(Signal::new(None)))]
	pub open: ReadOnlySignal<Option<bool>>,
	#[props(optional, default = false)]
	pub default_open: bool,
	#[props(optional)]
	pub on_open_change: Option<Callback<Option<bool>>>,
	#[props(optional, default = false)]
	is_arrow_hidden: bool,

	#[props(extends = div, extends = GlobalAttributes)]
	pub attributes: Vec<Attribute>,
	pub children: Element,
}

#[component]
pub fn Popover(props: PopoverProps) -> Element {
	let PopoverProps { open, default_open, on_open_change, is_arrow_hidden, children, attributes } = props;
	let is_controlled = use_hook(move || open().is_some());
	let (open, set_open) =
		use_controllable_state(UseControllableStateParams { is_controlled, prop: open, default_prop: default_open, on_change: on_open_change });
	use_context_provider::<PopoverContext>(|| PopoverContext::new(open, set_open));
	let handle_close = use_callback(move |()| {
		set_open.call(Some(false));
	});

	let mut current_ref = use_signal(|| None::<Rc<MountedData>>);
	use_outside_click(current_ref, handle_close, open);

	rsx! {
		Popper {
			position: "relative",
			is_arrow_hidden,
			onmounted: move |event: Event<MountedData>| current_ref.set(Some(event.data())),
			extra_attributes: attributes,
			{children}
		}
	}
}

#[derive(Clone, PartialEq, Props)]
pub struct PopoverTriggerProps {
	#[props(default = ReadOnlySignal::new(Signal::new(false)))]
	disabled: ReadOnlySignal<bool>,
	#[props(extends = GlobalAttributes, extends = button)]
	pub attributes: Vec<Attribute>,
	#[props(default = Vec::new())]
	pub container_attributes: Vec<Attribute>,
	pub children: Element,
}

#[component]
pub fn PopoverTrigger(props: PopoverTriggerProps) -> Element {
	let PopoverTriggerProps { attributes, disabled, container_attributes, .. } = props;
	let context = use_context::<PopoverContext>();

	rsx! {
		PopperAnchor { extra_attributes: container_attributes,
			Button {
				id: context.trigger_id.to_string(),
				r#type: "button",
				onclick: move |_| {
						let current_open = context.open.peek().unwrap_or_default();
						context.set_open.call(Some(!current_open));
				},
				disabled,
				aria_haspopup: "dialog",
				aria_expanded: *context.open.read(),
				aria_controls: context.content_id.to_string(),
				aria_disabled: disabled,
				"data-disabled": disabled,
				extra_attributes: attributes.clone(),
				"data-state": if context.open.read().unwrap_or_default() { "open" } else { "closed" },
				{props.children}
			}
		}
	}
}

#[derive(Clone, PartialEq, Props)]
pub struct PopoverContentProps {
	#[props(default = ESide::Bottom)]
	side: ESide,
	#[props(default = 0.0)]
	side_offset: f32,
	#[props(default = EAlign::Center)]
	align: EAlign,
	#[props(default = 0.0)]
	align_offset: f32,
	#[props(default = 0.0)]
	arrow_padding: f32,
	#[props(default = true)]
	avoid_collisions: bool,
	#[props(default = 4.0)]
	collision_padding: f32,
	#[props(extends = GlobalAttributes)]
	pub attributes: Vec<Attribute>,
	pub children: Element,
}

#[component]
pub fn PopoverContent(props: PopoverContentProps) -> Element {
	let context = use_context::<PopoverContext>();
	let handle_close = use_callback(move |()| {
		context.set_open.call(Some(false));
	});

	let mut current_ref = use_signal(|| None::<Rc<MountedData>>);

	use_escape(handle_close, context.open);

	if context.open.read().unwrap_or_default() {
		rsx! {
			PopperContent {
				role: "dialog",
				id: context.content_id.to_string(),
				side: props.side,
				side_offset: props.side_offset,
				align: props.align,
				align_offset: props.align_offset,
				arrow_padding: props.arrow_padding,
				avoid_collisions: props.avoid_collisions,
				collision_padding: props.collision_padding,
				aria_modal: true,
				aria_labelledby: context.trigger_id.to_string(),
				"data-state": if context.open.read().unwrap_or_default() { "open" } else { "closed" },
				onmounted: move |event: Event<MountedData>| current_ref.set(Some(event.data())),
				extra_attributes: props.attributes,
				{props.children}
			}
		}
	} else {
		rsx! {}
	}
}

#[derive(Clone, PartialEq, Props)]
pub struct PopoverCloseProps {
	pub children: Element,
	#[props(extends = GlobalAttributes, extends = button)]
	pub attributes: Vec<Attribute>,
}

#[component]
pub fn PopoverClose(props: PopoverCloseProps) -> Element {
	let context = use_context::<PopoverContext>();

	rsx! {
		Button {
			r#type: "button",
			aria_label: "Close popup",
			onclick: move |_| context.set_open.call(Some(false)),
			extra_attributes: props.attributes.clone(),
			{props.children}
		}
	}
}

#[derive(Clone, PartialEq, Props)]
pub struct PopperArrowProps {
	#[props(default = 10.0)]
	width: f32,
	#[props(default = 5.0)]
	height: f32,
	#[props(extends = svg, extends = GlobalAttributes)]
	pub attributes: Vec<Attribute>,
	#[props(default = None)]
	pub children: Option<Element>,
}

#[component]
pub fn PopoverArrow(props: PopperArrowProps) -> Element {
	rsx! {
		PopperArrow {
			width: props.width,
			height: props.height,
			extra_attributes: props.attributes.clone(),
			children: props.children,
		}
	}
}
