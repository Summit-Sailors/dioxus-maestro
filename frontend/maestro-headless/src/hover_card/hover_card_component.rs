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
struct HoverCardContext {
	pub open: Memo<Option<bool>>,
	pub set_open: Callback<Option<bool>>,
	pub content_id: Uuid,
	pub trigger_id: Uuid,
	pub on_open: EventHandler<()>,
	pub on_close: EventHandler<()>,
	pub has_selection_ref: Signal<bool>,
	pub is_pointer_down_on_content_ref: Signal<bool>,
}

#[derive(Props, PartialEq, Clone)]
pub struct HoverCardProps {
	#[props(optional, default = ReadOnlySignal::new(Signal::new(None)))]
	pub open: ReadOnlySignal<Option<bool>>,
	#[props(optional, default = false)]
	pub default_open: bool,
	#[props(optional)]
	pub on_open_change: Option<Callback<Option<bool>>>,
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
	let HoverCardProps { open, default_open, on_open_change, is_arrow_hidden, children, attributes, .. } = props;
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
			set_open(Some(true));
		}) as Box<dyn FnMut()>);

		let timer_id =
			window.set_timeout_with_callback_and_timeout_and_arguments_0(closure.as_ref().unchecked_ref(), props.open_delay as i32).expect("Cannot add timeout");
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
				set_open(Some(false));
			}) as Box<dyn FnMut()>);
			let timer_id =
				window.set_timeout_with_callback_and_timeout_and_arguments_0(closure.as_ref().unchecked_ref(), props.close_delay as i32).expect("Cannot add timeout");
			closure.forget();
			close_timer_ref.set(Some(timer_id));
		}
	});

	use_context_provider(|| HoverCardContext {
		open,
		set_open,
		on_open,
		on_close,
		has_selection_ref,
		is_pointer_down_on_content_ref,
		trigger_id: Uuid::new_v4(),
		content_id: Uuid::new_v4(),
	});

	rsx! {
		Popper {
			position: "relative",
			is_arrow_hidden,
			role: "presentation",
			"data-state": if open().unwrap_or_default() { "open" } else { "closed" },
			extra_attributes: attributes,
			{children}
		}
	}
}

#[derive(Clone, PartialEq, Props)]
pub struct HoverCardTriggerProps {
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

	#[props(extends = GlobalAttributes, extends = div)]
	pub attributes: Vec<Attribute>,
	#[props(default = Vec::new())]
	pub container_attributes: Vec<Attribute>,
	pub children: Element,
}

#[component]
pub fn HoverCardTrigger(props: HoverCardTriggerProps) -> Element {
	let context = use_context::<HoverCardContext>();

	rsx! {
		PopperAnchor { extra_attributes: props.container_attributes.clone(),
			Button {
				aria_describedby: context.content_id.to_string(),
				id: context.trigger_id.to_string(),
				aria_haspopup: "dialog",
				aria_expanded: *context.open.read(),
				aria_controls: context.content_id.to_string(),
				extra_attributes: props.attributes.clone(),
				"data-state": if context.open.read().unwrap_or_default() { "open" } else { "closed" },
				tabindex: 0,
				onmouseenter: move |event| {
						context.on_open.call(());
						if let Some(handler) = props.onmouseenter.as_ref() {
								handler.call(event);
						}
				},
				onmouseleave: move |event| {
						context.on_close.call(());
						if let Some(handler) = props.onmouseleave {
								handler.call(event);
						}
				},
				onfocus: move |event| {
						context.on_open.call(());
						if let Some(handler) = props.onfocus {
								handler.call(event);
						}
				},
				onblur: move |event| {
						context.on_close.call(());
						if let Some(handler) = props.onblur {
								handler.call(event);
						}
				},
				onclick: move |event| {
						if let Some(handler) = props.onclick {
								handler.call(event);
						}
				},
				{props.children}
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
	#[props(default = 0.0)]
	arrow_padding: f32,
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
	let context = use_context::<HoverCardContext>();
	let mut current_ref = use_signal(|| None::<Rc<MountedData>>);

	rsx! {
		Presence {
			node_ref: current_ref,
			present: context.open.read().unwrap_or_default(),
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
				aria_labelledby: context.trigger_id.to_string(),
				aria_hidden: !context.open.read().unwrap_or_default(),
				"data-state": if context.open.read().unwrap_or_default() { "open" } else { "closed" },
				extra_attributes: props.attributes,
				onmounted: move |event: Event<MountedData>| current_ref.set(Some(event.data())),
				onmouseenter: move |event| {
						context.on_open.call(());
						if let Some(callback) = props.onmouseenter.as_ref() {
								callback.call(event);
						}
				},
				onmouseleave: move |event| {
						context.on_close.call(());
						if let Some(callback) = props.onmouseleave.as_ref() {
								callback.call(event);
						}
				},
				{props.children}
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
	rsx! {
		PopperArrow {
			width: props.width,
			height: props.height,
			extra_attributes: props.attributes.clone(),
			children: props.children,
		}
	}
}
