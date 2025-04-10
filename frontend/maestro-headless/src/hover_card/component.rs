use {
	crate::{
		focus_trap::FocusTrap,
		popper::{Popper, PopperAnchor, PopperArrow, PopperContent},
		presence::Presence,
		shared::{EAlign, ESide, UseControllableStateParams, use_controllable_state, use_outside_click, use_ref_provider},
	},
	dioxus::prelude::*,
	std::fmt::Debug,
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
pub struct HoverCardRootProps {
	#[props(optional, default = ReadOnlySignal::new(Signal::new(None)))]
	pub open: ReadOnlySignal<Option<bool>>,
	#[props(optional, default = false)]
	pub default_open: bool,
	#[props(optional)]
	pub on_open_change: Option<Callback<bool>>,

	#[props(default = ReadOnlySignal::new(Signal::new(700.0)))]
	open_delay: ReadOnlySignal<f32>,
	#[props(default = ReadOnlySignal::new(Signal::new(300.0)))]
	close_delay: ReadOnlySignal<f32>,

	#[props(extends = GlobalAttributes, extends = div)]
	pub attributes: Vec<Attribute>,
	children: Element,
}

#[component]
pub fn HoverCardRoot(props: HoverCardRootProps) -> Element {
	let HoverCardRootProps { open, default_open, on_open_change, open_delay, close_delay, children, attributes } = props;
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
			window.set_timeout_with_callback_and_timeout_and_arguments_0(closure.as_ref().unchecked_ref(), open_delay() as i32).expect("Cannot add timeout");
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
				window.set_timeout_with_callback_and_timeout_and_arguments_0(closure.as_ref().unchecked_ref(), close_delay() as i32).expect("Cannot add timeout");
			closure.forget();
			close_timer_ref.set(Some(timer_id));
		}
	});

	use_context_provider(|| HoverCardContext::new(open, set_open, on_open, on_close, has_selection_ref, is_pointer_down_on_content_ref));

	rsx! {
		Popper {
			position: "relative",
			"data-state": if open() { "open" } else { "closed" },
			extra_attributes: attributes,
			{children}
		}
	}
}

#[derive(Clone, PartialEq, Props)]
pub struct HoverCardTriggerProps {
	#[props(extends = GlobalAttributes, extends = a)]
	pub attributes: Vec<Attribute>,
	#[props(optional)]
	pub children: Element,
}

#[component]
pub fn HoverCardTrigger(props: HoverCardTriggerProps) -> Element {
	let HoverCardTriggerProps { attributes, children } = props;

	let context = use_context::<HoverCardContext>();

	rsx! {
		PopperAnchor {
			a {
				id: context.trigger_id.to_string(),
				aria_describedby: context.content_id.to_string(),
				aria_haspopup: "modal",
				aria_expanded: *context.open.read(),
				aria_controls: context.content_id.to_string(),
				"data-state": if *context.open.read() { "open" } else { "closed" },
				tabindex: 0,
				rel: "noreferrer noopener",
				target: "_blank",
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
				..attributes,
				{children}
			}
		}
	}
}

#[derive(Clone, PartialEq, Props)]
pub struct HoverCardContentProps {
	#[props(default = ReadOnlySignal::new(Signal::new(ESide::Bottom)))]
	side: ReadOnlySignal<ESide>,
	#[props(default = ReadOnlySignal::new(Signal::new(0.0)))]
	side_offset: ReadOnlySignal<f32>,
	#[props(default = ReadOnlySignal::new(Signal::new(EAlign::Center)))]
	align: ReadOnlySignal<EAlign>,
	#[props(default = ReadOnlySignal::new(Signal::new(0.0)))]
	align_offset: ReadOnlySignal<f32>,
	#[props(default = ReadOnlySignal::new(Signal::new(true)))]
	avoid_collisions: ReadOnlySignal<bool>,
	#[props(default = ReadOnlySignal::new(Signal::new(4.0)))]
	collision_padding: ReadOnlySignal<f32>,

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

	let handle_close = use_callback(move |()| {
		context.set_open.call(false);
	});

	let current_ref = use_ref_provider();

	use_outside_click(current_ref, handle_close, context.open);

	let mut attrs = attributes.clone();
	attrs.push(Attribute::new("--maestro-hover-card-anchor-height", "var(--maestro-popper-anchor-height)", Some("style"), false));
	attrs.push(Attribute::new("--maestro-hover-card-anchor-width", "var(--maestro-popper-anchor-width)", Some("style"), false));
	attrs.push(Attribute::new("--maestro-hover-card-content-height", "var(--maestro-popper-content-height)", Some("style"), false));
	attrs.push(Attribute::new("--maestro-hover-card-content-width", "var(--maestro-popper-content-width)", Some("style"), false));

	rsx! {
		Presence { present: *context.open.read(),
			PopperContent {
				role: "modal",
				id: context.content_id.to_string(),
				side,
				side_offset,
				align,
				align_offset,
				avoid_collisions,
				collision_padding,
				aria_labelledby: context.trigger_id.to_string(),
				aria_hidden: !*context.open.read(),
				aria_modal: true,
				"data-state": if *context.open.read() { "open" } else { "closed" },
				extra_attributes: attrs,
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
