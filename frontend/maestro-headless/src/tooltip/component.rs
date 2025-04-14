use {
	crate::{
		button::Button,
		popper::{Popper, PopperAnchor, PopperArrow, PopperContent},
		presence::Presence,
		shared::{EAlign, ESide, UseControllableStateParams, use_controllable_state, use_ref_provider},
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
struct TooltipProviderContext {
	pub is_open_delayed_ref: Signal<bool>,
	pub delay_duration: f32,
	pub on_open: EventHandler<()>,
	pub on_close: EventHandler<()>,
	pub on_pointer_in_transit_change: EventHandler<bool>,
	pub is_pointer_in_transit_ref: Signal<bool>,
}

impl TooltipProviderContext {
	pub fn new(
		is_open_delayed_ref: Signal<bool>,
		delay_duration: f32,
		on_open: EventHandler<()>,
		on_close: EventHandler<()>,
		on_pointer_in_transit_change: EventHandler<bool>,
		is_pointer_in_transit_ref: Signal<bool>,
	) -> Self {
		Self { is_open_delayed_ref, delay_duration, on_open, on_close, on_pointer_in_transit_change, is_pointer_in_transit_ref }
	}
}

#[derive(Props, PartialEq, Clone)]
pub struct TooltipRootProps {
	#[props(default = 700.0)]
	pub delay_duration_ms: f32,
	#[props(default = 300.0)]
	pub skip_delay_duration_ms: f32,

	#[props(extends = GlobalAttributes, extends = div)]
	pub attributes: Vec<Attribute>,
	children: Element,
}

#[component]
pub fn TooltipRoot(props: TooltipRootProps) -> Element {
	let TooltipRootProps { delay_duration_ms, skip_delay_duration_ms, attributes, children } = props;

	let mut is_open_delayed_ref = use_signal(|| true);
	let mut is_pointer_in_transit_ref = use_signal(|| false);
	let mut skip_delay_timer_ref = use_signal(|| None::<i32>);

	let on_open = use_callback(move |_| {
		let window = window().expect("Must be window");
		if let Some(timer_id) = skip_delay_timer_ref.clone().peek().as_ref() {
			window.clear_timeout_with_handle(*timer_id);
		}
		is_open_delayed_ref.set(false);
	});

	let on_close = use_callback(move |_| {
		let window = window().expect("Must be window");
		if let Some(timer_id) = *skip_delay_timer_ref.clone().peek() {
			window.clear_timeout_with_handle(timer_id);
		}
		let closure = Closure::wrap(Box::new(move || {
			is_open_delayed_ref.set(true);
		}) as Box<dyn FnMut()>);

		let timer_id = window
			.set_timeout_with_callback_and_timeout_and_arguments_0(closure.as_ref().unchecked_ref(), skip_delay_duration_ms as i32)
			.expect("Cannot add timeout");

		closure.forget();

		skip_delay_timer_ref.set(Some(timer_id));
	});

	let on_pointer_in_transit_change = use_callback(move |in_transit: bool| {
		is_pointer_in_transit_ref.set(in_transit);
	});

	use_context_provider(|| {
		TooltipProviderContext::new(is_open_delayed_ref, delay_duration_ms, on_open, on_close, on_pointer_in_transit_change, is_pointer_in_transit_ref)
	});

	rsx! {
		div { ..attributes,{children} }
	}
}

#[derive(Clone, Debug, PartialEq, Copy)]
struct TooltipContext {
	pub open: Memo<bool>,
	pub set_open: Callback<bool>,
	pub content_id: Uuid,
	pub trigger_id: Uuid,
	pub state_attribute: Memo<String>,
	pub trigger: Signal<Option<Rc<MountedData>>>,
	pub on_trigger_enter: EventHandler<()>,
	pub on_trigger_leave: EventHandler<()>,
	pub on_open: EventHandler<()>,
	pub on_close: EventHandler<()>,
}

impl TooltipContext {
	pub fn new(
		open: Memo<bool>,
		set_open: Callback<bool>,
		state_attribute: Memo<String>,
		trigger: Signal<Option<Rc<MountedData>>>,
		on_trigger_enter: EventHandler<()>,
		on_trigger_leave: EventHandler<()>,
		on_open: EventHandler<()>,
		on_close: EventHandler<()>,
	) -> Self {
		Self {
			open,
			set_open,
			state_attribute,
			trigger,
			on_trigger_enter,
			on_trigger_leave,
			on_open,
			on_close,
			trigger_id: Uuid::new_v4(),
			content_id: Uuid::new_v4(),
		}
	}
}

#[derive(Props, PartialEq, Clone)]
pub struct TooltipProps {
	#[props(optional, default = ReadOnlySignal::new(Signal::new(None)))]
	pub open: ReadOnlySignal<Option<bool>>,
	#[props(optional, default = false)]
	pub default_open: bool,
	#[props(optional)]
	pub on_open_change: Option<Callback<bool>>,
	#[props(optional)]
	delay_duration: Option<f32>,

	#[props(extends = GlobalAttributes, extends = div)]
	pub attributes: Vec<Attribute>,
	children: Element,
}

#[component]
pub fn Tooltip(props: TooltipProps) -> Element {
	let TooltipProps { open, default_open, on_open_change, delay_duration, children, attributes } = props;

	let is_controlled = use_hook(move || open().is_some());
	let (open, set_open) =
		use_controllable_state(UseControllableStateParams { is_controlled, prop: open, default_prop: default_open, on_change: on_open_change });

	let provider_context = use_context::<TooltipProviderContext>();
	let trigger = use_signal(|| None::<Rc<MountedData>>);
	let mut open_timer_ref = use_signal(|| None::<i32>);
	let mut was_open_delayed_ref = use_signal(|| false);
	let delay_duration = delay_duration.unwrap_or(provider_context.delay_duration);

	let state_attribute = use_memo(move || match open() {
		true =>
			if was_open_delayed_ref() {
				"delayed-open".to_string()
			} else {
				"instant-open".to_string()
			},
		false => "closed".to_string(),
	});

	let on_open = use_callback(move |_| {
		let window = window().expect("Must be window");
		if let Some(timer_id) = open_timer_ref.clone().peek().as_ref() {
			window.clear_timeout_with_handle(*timer_id);
		}
		was_open_delayed_ref.set(false);
		set_open(true)
	});

	let on_close = use_callback(move |_| {
		let window = window().expect("Must be window");
		if let Some(timer_id) = *open_timer_ref.clone().peek() {
			window.clear_timeout_with_handle(timer_id);
		}
		set_open(false);
	});

	let on_delayed_open = use_callback(move |_| {
		let window = window().expect("Must be window");
		if let Some(timer_id) = *open_timer_ref.clone().peek() {
			window.clear_timeout_with_handle(timer_id);
		}

		let closure = Closure::wrap(Box::new(move || {
			was_open_delayed_ref.set(true);
			set_open(true);
		}) as Box<dyn FnMut()>);

		let timer_id =
			window.set_timeout_with_callback_and_timeout_and_arguments_0(closure.as_ref().unchecked_ref(), delay_duration as i32).expect("Cannot add timeout");
		closure.forget();
		open_timer_ref.set(Some(timer_id));
	});

	let on_trigger_enter = use_callback(move |_| {
		if *provider_context.is_open_delayed_ref.peek() {
			on_delayed_open.call(());
		} else {
			on_open.call(());
		}
	});

	let on_trigger_leave = use_callback(move |_| {
		let window = window().expect("Must be window");
		if let Some(timer_id) = *open_timer_ref.clone().peek() {
			window.clear_timeout_with_handle(timer_id);
		}
		on_close.call(())
	});

	use_context_provider(|| TooltipContext::new(open, set_open, state_attribute, trigger, on_trigger_enter, on_trigger_leave, on_open, on_close));

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
pub struct TooltipTriggerProps {
	#[props(optional)]
	onclick: Option<EventHandler<MouseEvent>>,

	#[props(extends = GlobalAttributes, extends = button)]
	pub attributes: Vec<Attribute>,
	#[props(default = Vec::new())]
	pub children: Element,
}

#[component]
pub fn TooltipTrigger(props: TooltipTriggerProps) -> Element {
	let TooltipTriggerProps { attributes, onclick, children } = props;

	let context = use_context::<TooltipContext>();
	let provider_context = use_context::<TooltipProviderContext>();
	let mut is_pointer_down_ref = use_signal(|| false);
	let mut has_pointer_move_opened_ref = use_signal(|| false);

	rsx! {
		PopperAnchor { role: "tooltip",
			Button {
				aria_describedby: context.content_id.to_string(),
				"data-state-open": context.state_attribute.clone(),
				id: context.trigger_id.to_string(),
				aria_haspopup: "dialog",
				aria_expanded: *context.open.read(),
				aria_controls: context.content_id.to_string(),
				extra_attributes: attributes.clone(),
				"data-state": if *context.open.read() { "open" } else { "closed" },
				onmouseenter: move |_| {
						if !*has_pointer_move_opened_ref.peek()
								&& !*provider_context.is_pointer_in_transit_ref.peek()
						{
								context.on_trigger_enter.call(());
								has_pointer_move_opened_ref.set(true);
						}
				},
				onmouseleave: move |_| {
						context.on_trigger_leave.call(());
						has_pointer_move_opened_ref.set(false);
				},
				onmousedown: move |_| {
						if *context.open.peek() {
								context.on_close.call(());
						}
						is_pointer_down_ref.set(true);
				},
				onfocus: move |_| {
						if !is_pointer_down_ref() {
								context.on_open.call(());
						}
				},
				onblur: move |_| {
						context.on_close.call(());
				},
				onclick: move |event| {
						context.on_close.call(());
						if let Some(callback) = onclick {
								callback.call(event);
						}
				},
				{children}
			}
		}
	}
}

#[derive(Clone, PartialEq, Props)]
pub struct TooltipContentProps {
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
	#[props(extends = GlobalAttributes)]
	pub attributes: Vec<Attribute>,
	pub children: Element,
}

#[component]
pub fn TooltipContent(props: TooltipContentProps) -> Element {
	let TooltipContentProps { side, side_offset, align, align_offset, avoid_collisions, collision_padding, attributes, children } = props;

	let context = use_context::<TooltipContext>();
	use_ref_provider();

	let mut attrs = attributes.clone();
	attrs.push(Attribute::new("--maestro-tooltip-anchor-height", "var(--maestro-popper-anchor-height)", Some("style"), false));
	attrs.push(Attribute::new("--maestro-tooltip-anchor-width", "var(--maestro-popper-anchor-width)", Some("style"), false));
	attrs.push(Attribute::new("--maestro-tooltip-content-height", "var(--maestro-popper-content-height)", Some("style"), false));
	attrs.push(Attribute::new("--maestro-tooltip-content-width", "var(--maestro-popper-content-width)", Some("style"), false));

	rsx! {
		Presence { present: *context.open.read(),
			PopperContent {
				role: "popup",
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
				"data-state-open": context.state_attribute.clone(),
				extra_attributes: attrs.clone(),
				{children}
			}
		}
	}
}

#[derive(Clone, PartialEq, Props)]
pub struct TooltipArrowProps {
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
pub fn TooltipArrow(props: TooltipArrowProps) -> Element {
	let TooltipArrowProps { width, height, attributes, children } = props;

	rsx! {
		PopperArrow {
			width,
			height,
			extra_attributes: attributes.clone(),
			children,
		}
	}
}
