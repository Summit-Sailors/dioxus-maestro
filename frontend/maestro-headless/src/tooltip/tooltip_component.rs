use {
	crate::{
		button::Button,
		hooks::{UseControllableStateParams, use_controllable_state},
		popper::{Popper, PopperAnchor, PopperArrow, PopperContent},
		presence::Presence,
		utils::{EAlign, ESide},
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

#[derive(Props, PartialEq, Clone)]
pub struct TooltipProviderProps {
	#[props(default = 700.0)]
	pub delay_duration_ms: f32,
	#[props(default = 300.0)]
	pub skip_delay_duration_ms: f32,

	#[props(extends = GlobalAttributes, extends = div)]
	pub attributes: Vec<Attribute>,
	children: Element,
}

#[component]
pub fn TooltipProvider(props: TooltipProviderProps) -> Element {
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
		let skip_delay_duration = props.skip_delay_duration_ms;
		let closure = Closure::wrap(Box::new(move || {
			is_open_delayed_ref.set(true);
		}) as Box<dyn FnMut()>);

		let timer_id =
			window.set_timeout_with_callback_and_timeout_and_arguments_0(closure.as_ref().unchecked_ref(), skip_delay_duration as i32).expect("Cannot add timeout");

		closure.forget();

		skip_delay_timer_ref.set(Some(timer_id));
	});

	let on_pointer_in_transit_change = use_callback(move |in_transit: bool| {
		is_pointer_in_transit_ref.set(in_transit);
	});

	use_context_provider(|| TooltipProviderContext {
		is_open_delayed_ref,
		delay_duration: props.delay_duration_ms,
		on_open,
		on_close,
		on_pointer_in_transit_change,
		is_pointer_in_transit_ref,
	});

	rsx! {
		div { ..props.attributes,{props.children} }
	}
}

#[derive(Clone, Debug, PartialEq, Copy)]
struct TooltipContext {
	pub open: Memo<Option<bool>>,
	pub set_open: Callback<Option<bool>>,
	pub content_id: Uuid,
	pub trigger_id: Uuid,
	pub state_attribute: Memo<String>,
	pub trigger: Signal<Option<Rc<MountedData>>>,
	pub on_trigger_enter: EventHandler<()>,
	pub on_trigger_leave: EventHandler<()>,
	pub on_open: EventHandler<()>,
	pub on_close: EventHandler<()>,
}

#[derive(Props, PartialEq, Clone)]
pub struct TooltipProps {
	#[props(optional, default = ReadOnlySignal::new(Signal::new(None)))]
	pub open: ReadOnlySignal<Option<bool>>,
	#[props(optional, default = false)]
	pub default_open: bool,
	#[props(optional)]
	pub on_open_change: Option<Callback<Option<bool>>>,
	#[props(optional, default = false)]
	is_arrow_hidden: bool,
	#[props(optional)]
	delay_duration: Option<f32>,

	#[props(extends = GlobalAttributes, extends = div)]
	pub attributes: Vec<Attribute>,
	children: Element,
}

#[component]
pub fn Tooltip(props: TooltipProps) -> Element {
	let TooltipProps { open, default_open, on_open_change, is_arrow_hidden, children, attributes, .. } = props;
	let is_controlled = use_hook(move || open().is_some());
	let (open, set_open) =
		use_controllable_state(UseControllableStateParams { is_controlled, prop: open, default_prop: default_open, on_change: on_open_change });
	let provider_context = use_context::<TooltipProviderContext>();
	let trigger = use_signal(|| None::<Rc<MountedData>>);
	let mut open_timer_ref = use_signal(|| None::<i32>);
	let mut was_open_delayed_ref = use_signal(|| false);
	let delay_duration = props.delay_duration.unwrap_or(provider_context.delay_duration);

	let state_attribute = use_memo(move || {
		if let Some(open) = open() {
			match open {
				true =>
					if was_open_delayed_ref() {
						"delayed-open".to_string()
					} else {
						"instant-open".to_string()
					},
				false => "closed".to_string(),
			}
		} else {
			"closed".to_string()
		}
	});

	let on_open = use_callback(move |_| {
		let window = window().expect("Must be window");
		if let Some(timer_id) = open_timer_ref.clone().peek().as_ref() {
			window.clear_timeout_with_handle(*timer_id);
		}
		was_open_delayed_ref.set(false);
		set_open(Some(true))
	});

	let on_close = use_callback(move |_| {
		let window = window().expect("Must be window");
		if let Some(timer_id) = *open_timer_ref.clone().peek() {
			window.clear_timeout_with_handle(timer_id);
		}
		set_open(Some(false));
	});

	let on_delayed_open = use_callback(move |_| {
		let window = window().expect("Must be window");
		if let Some(timer_id) = *open_timer_ref.clone().peek() {
			window.clear_timeout_with_handle(timer_id);
		}

		let closure = Closure::wrap(Box::new(move || {
			was_open_delayed_ref.set(true);
			set_open(Some(true));
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

	use_context_provider(|| TooltipContext {
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
	});

	rsx! {
		Popper {
			position: "relative",
			is_arrow_hidden,
			"data-state": if open().unwrap_or_default() { "open" } else { "closed" },
			extra_attributes: attributes,
			{children}
		}
	}
}

#[derive(Clone, PartialEq, Props)]
pub struct TooltipTriggerProps {
	#[props(optional)]
	onmouseenter: Option<EventHandler<MouseEvent>>,
	#[props(optional)]
	onmouseleave: Option<EventHandler<MouseEvent>>,
	#[props(optional)]
	onmousedown: Option<EventHandler<MouseEvent>>,
	#[props(optional)]
	onmouseup: Option<EventHandler<MouseEvent>>,
	#[props(optional)]
	onfocus: Option<EventHandler<FocusEvent>>,
	#[props(optional)]
	onblur: Option<EventHandler<FocusEvent>>,
	#[props(optional)]
	onclick: Option<EventHandler<MouseEvent>>,

	#[props(extends = GlobalAttributes, extends = button)]
	pub attributes: Vec<Attribute>,
	#[props(default = Vec::new())]
	pub container_attributes: Vec<Attribute>,
	pub children: Element,
}

#[component]
pub fn TooltipTrigger(props: TooltipTriggerProps) -> Element {
	let TooltipTriggerProps { attributes, container_attributes, .. } = props;
	let context = use_context::<TooltipContext>();
	let provider_context = use_context::<TooltipProviderContext>();
	let mut is_pointer_down_ref = use_signal(|| false);
	let mut has_pointer_move_opened_ref = use_signal(|| false);

	rsx! {
		PopperAnchor { extra_attributes: container_attributes, role: "tooltip",
			Button {
				aria_describedby: context.content_id.to_string(),
				"data-state-open": context.state_attribute.clone(),
				id: context.trigger_id.to_string(),
				aria_haspopup: "dialog",
				aria_expanded: *context.open.read(),
				aria_controls: context.content_id.to_string(),
				extra_attributes: attributes.clone(),
				"data-state": if context.open.read().unwrap_or_default() { "open" } else { "closed" },
				onmouseenter: move |event| {
						if !*has_pointer_move_opened_ref.peek()
								&& !*provider_context.is_pointer_in_transit_ref.peek()
						{
								context.on_trigger_enter.call(());
								has_pointer_move_opened_ref.set(true);
						}
						if let Some(callback) = props.onmouseenter {
								callback.call(event);
						}
				},
				onmouseleave: move |event| {
						context.on_trigger_leave.call(());
						has_pointer_move_opened_ref.set(false);
						if let Some(callback) = props.onmouseleave {
								callback.call(event);
						}
				},
				onmousedown: move |event| {
						if let Some(open) = *context.open.peek() {
								if open {
										context.on_close.call(());
								}
						}
						is_pointer_down_ref.set(true);
						if let Some(callback) = props.onmousedown {
								callback.call(event);
						}
				},
				onfocus: move |event| {
						if !is_pointer_down_ref() {
								context.on_open.call(());
						}
						if let Some(callback) = props.onfocus {
								callback.call(event);
						}
				},
				onblur: move |event| {
						context.on_close.call(());
						if let Some(callback) = props.onblur {
								callback.call(event);
						}
				},
				onclick: move |event| {
						context.on_close.call(());
						if let Some(callback) = props.onclick {
								callback.call(event);
						}
				},
				{props.children}
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
pub fn TooltipContent(props: TooltipContentProps) -> Element {
	let context = use_context::<TooltipContext>();
	let mut current_ref = use_signal(|| None::<Rc<MountedData>>);

	rsx! {
		Presence {
			node_ref: current_ref,
			present: context.open.read().unwrap_or_default(),
			PopperContent {
				role: "popup",
				id: context.content_id.to_string(),
				side: props.side,
				side_offset: props.side_offset,
				align: props.align,
				align_offset: props.align_offset,
				arrow_padding: props.arrow_padding,
				avoid_collisions: props.avoid_collisions,
				collision_padding: props.collision_padding,
				aria_labelledby: context.trigger_id.to_string(),
				aria_hidden: !context.open.read().unwrap_or_default(),
				"data-state": if context.open.read().unwrap_or_default() { "open" } else { "closed" },
				"data-state-open": context.state_attribute.clone(),
				extra_attributes: props.attributes,
				onmounted: move |event: Event<MountedData>| current_ref.set(Some(event.data())),
				{props.children}
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
	rsx! {
		PopperArrow {
			width: props.width,
			height: props.height,
			extra_attributes: props.attributes.clone(),
			children: props.children,
		}
	}
}
