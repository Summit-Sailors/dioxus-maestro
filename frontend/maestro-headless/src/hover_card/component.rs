use {
	crate::{
		button::Button,
		popper::{Popper, PopperAnchor, PopperArrow, PopperContent},
		presence::Presence,
		shared::{EAlign, ESide, UseControllableStateParams, use_controllable_state},
	},
	dioxus::prelude::*,
	std::{fmt::Debug, rc::Rc},
	uuid::Uuid,
	web_sys::{
		wasm_bindgen::{JsCast, prelude::Closure},
		window,
	},
};

#[derive(Clone, Debug, PartialEq, Copy)]
struct HoverCardContext {
	pub open: Memo<bool>,
	pub set_open: Callback<bool>,
	pub content_id: Uuid,
	pub trigger_id: Uuid,
	pub on_open: EventHandler<()>,
	pub on_close: EventHandler<()>,
	pub has_selection_ref: Signal<bool>,
	pub is_pointer_down_on_content_ref: Signal<bool>,
}

impl HoverCardContext {
	pub fn new(
		open: Memo<bool>,
		set_open: Callback<bool>,
		on_open: EventHandler<()>,
		on_close: EventHandler<()>,
		has_selection_ref: Signal<bool>,
		is_pointer_down_on_content_ref: Signal<bool>,
	) -> Self {
		Self { open, set_open, on_close, on_open, has_selection_ref, is_pointer_down_on_content_ref, trigger_id: Uuid::new_v4(), content_id: Uuid::new_v4() }
	}
}

#[derive(Props, PartialEq, Clone)]
pub struct HoverCardProps {
	#[props(optional, default = ReadOnlySignal::new(Signal::new(None)))]
	pub open: ReadOnlySignal<Option<bool>>,
	#[props(optional, default = false)]
	pub default_open: bool,
	#[props(optional)]
	pub on_open_change: Option<Callback<bool>>,
	#[props(optional, default = false)]
	pub is_arrow_hidden: bool,
	#[props(default = 700.0)]
	open_delay: f32,
	#[props(default = 300.0)]
	close_delay: f32,

	#[props(extends = GlobalAttributes, extends = div)]
	pub attributes: Vec<Attribute>,
	children: Element,
}

#[component]
pub fn HoverCard(props: HoverCardProps) -> Element {
	let HoverCardProps { open, default_open, on_open_change, is_arrow_hidden, open_delay, close_delay, children, attributes } = props;
	let is_controlled = use_hook(move || open().is_some());
	let (open, set_open) =
		use_controllable_state(UseControllableStateParams { is_controlled, prop: open, default_prop: default_open, on_change: on_open_change });

	let mut open_timer_ref = use_signal(|| None::<i32>);
	let mut close_timer_ref = use_signal(|| None::<i32>);
	let has_selection_ref = use_signal(|| false);
	let is_pointer_down_on_content_ref = use_signal(|| false);

	let on_open = use_callback(move |_| {
		let window = window().expect("Must be window");
		if let Some(timer_id) = close_timer_ref.clone().peek().as_ref() {
			window.clear_timeout_with_handle(*timer_id);
		}
		let closure = Closure::wrap(Box::new(move || {
			set_open(true);
		}) as Box<dyn FnMut()>);

		let timer_id =
			window.set_timeout_with_callback_and_timeout_and_arguments_0(closure.as_ref().unchecked_ref(), open_delay as i32).expect("Cannot add timeout");
		closure.forget();
		open_timer_ref.set(Some(timer_id));
	});

	let on_close = use_callback(move |_| {
		let window = window().expect("Must be window");
		if let Some(timer_id) = *open_timer_ref.clone().peek() {
			window.clear_timeout_with_handle(timer_id);
		}
		if !*has_selection_ref.peek() && !*is_pointer_down_on_content_ref.peek() {
			let closure = Closure::wrap(Box::new(move || {
				set_open(false);
			}) as Box<dyn FnMut()>);
			let timer_id =
				window.set_timeout_with_callback_and_timeout_and_arguments_0(closure.as_ref().unchecked_ref(), close_delay as i32).expect("Cannot add timeout");
			closure.forget();
			close_timer_ref.set(Some(timer_id));
		}
	});

	use_context_provider(|| HoverCardContext::new(open, set_open, on_open, on_close, has_selection_ref, is_pointer_down_on_content_ref));

	rsx! {
		Popper {
			position: "relative",
			is_arrow_hidden,
			"data-state": if open() { "open" } else { "closed" },
			extra_attributes: attributes,
			{children}
		}
	}
}

#[derive(Clone, PartialEq, Props)]
pub struct HoverCardTriggerProps {
	#[props(optional)]
	onclick: Option<EventHandler<MouseEvent>>,

	#[props(extends = GlobalAttributes, extends = div)]
	pub attributes: Vec<Attribute>,
	#[props(default = Vec::new())]
	pub container_attributes: Vec<Attribute>,
	pub children: Element,
}

#[component]
pub fn HoverCardTrigger(props: HoverCardTriggerProps) -> Element {
	let HoverCardTriggerProps { onclick, attributes, container_attributes, children } = props;

	let context = use_context::<HoverCardContext>();

	rsx! {
		PopperAnchor { extra_attributes: container_attributes.clone(),
			Button {
				r#type: "button",
				id: context.trigger_id.to_string(),
				aria_describedby: context.content_id.to_string(),
				aria_haspopup: "dialog",
				aria_expanded: *context.open.read(),
				aria_controls: context.content_id.to_string(),
				extra_attributes: attributes.clone(),
				"data-state": if *context.open.read() { "open" } else { "closed" },
				tabindex: 0,
				onmouseenter: move |_| {
						context.on_open.call(());
				},
				onmouseleave: move |_| {
						context.on_close.call(());
				},
				onfocus: move |_| {
						context.on_open.call(());
				},
				onblur: move |_| {
						context.on_close.call(());
				},
				onclick: move |event| {
						if let Some(handler) = onclick {
								handler.call(event);
						}
				},
				{children}
			}
		}
	}
}

#[derive(Clone, PartialEq, Props)]
pub struct HoverCardContentProps {
	#[props(default = ESide::Bottom)]
	side: ESide,
	#[props(default = 0.0)]
	side_offset: f32,
	#[props(default = EAlign::Center)]
	align: EAlign,
	#[props(default = 0.0)]
	align_offset: f32,
	#[props(default = true)]
	avoid_collisions: bool,
	#[props(default = 4.0)]
	collision_padding: f32,

	#[props(optional)]
	onmouseenter: Option<EventHandler<MouseEvent>>,
	#[props(optional)]
	onmouseleave: Option<EventHandler<MouseEvent>>,
	#[props(optional)]
	#[props(extends = GlobalAttributes)]
	pub attributes: Vec<Attribute>,
	pub children: Element,
}

#[component]
pub fn HoverCardContent(props: HoverCardContentProps) -> Element {
	let HoverCardContentProps { side, side_offset, align, align_offset, avoid_collisions, collision_padding, onmouseenter, onmouseleave, attributes, children } =
		props;

	let context = use_context::<HoverCardContext>();
	let mut current_ref = use_signal(|| None::<Rc<MountedData>>);

	rsx! {
		Presence { node_ref: current_ref, present: *context.open.read(),
			PopperContent {
				role: "dialog",
				id: context.content_id.to_string(),
				side,
				side_offset,
				align,
				align_offset,
				avoid_collisions,
				collision_padding,
				aria_labelledby: context.trigger_id.to_string(),
				aria_hidden: !*context.open.read(),
				"data-state": if *context.open.read() { "open" } else { "closed" },
				extra_attributes: attributes,
				onmounted: move |event: Event<MountedData>| current_ref.set(Some(event.data())),
				onmouseenter: move |event| {
						context.on_open.call(());
						if let Some(callback) = onmouseenter.as_ref() {
								callback.call(event);
						}
				},
				onmouseleave: move |event| {
						context.on_close.call(());
						if let Some(callback) = onmouseleave.as_ref() {
								callback.call(event);
						}
				},
				{children}
			}
		}
	}
}

#[derive(Clone, PartialEq, Props)]
pub struct HoverCardArrowProps {
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
pub fn HoverCardArrow(props: HoverCardArrowProps) -> Element {
	let HoverCardArrowProps { width, height, attributes, children } = props;

	rsx! {
		PopperArrow {
			width,
			height,
			extra_attributes: attributes.clone(),
			children,
		}
	}
}
