use {
	crate::{
		focus_trap::FocusTrap,
		utils::{EAlign, ESide},
	},
	dioxus::{prelude::*, web::WebEventExt},
	serde::{Deserialize, Serialize},
	std::rc::Rc,
	web_sys::{
		wasm_bindgen::{JsCast, prelude::Closure},
		window,
	},
};

#[derive(Clone, Debug, PartialEq)]
pub struct Rect {
	pub x: f32,
	pub y: f32,
	pub width: f32,
	pub height: f32,
}

#[derive(Clone, Debug, PartialEq)]
pub struct Placement {
	pub side: ESide,
	pub alignment: Option<Alignment>,
}

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum Alignment {
	Start,
	End,
}

impl Placement {
	pub fn side(&self) -> ESide {
		self.side
	}

	pub fn alignment(&self) -> Option<Alignment> {
		self.alignment
	}
}

#[derive(Clone, Debug, PartialEq)]
struct PopperContext {
	anchor: Signal<Option<Rc<MountedData>>>,
	is_arrow_hidden: bool,
}

#[derive(Clone, Debug, PartialEq)]
struct PopperContentContext {
	content: Signal<Option<Rc<MountedData>>>,
	placement: Signal<Placement>,
	arrow_data: Signal<ArrowData>,
	arrow: Signal<Option<Rc<MountedData>>>,
}

#[derive(Clone, Debug, Default, PartialEq)]
struct FloatingStyles {
	position: String,
	top: String,
	left: String,
	transform: Option<String>,
}

impl FloatingStyles {
	pub fn style_position(&self) -> String {
		self.position.clone()
	}

	pub fn style_top(&self) -> String {
		self.top.clone()
	}

	pub fn style_left(&self) -> String {
		self.left.clone()
	}

	pub fn style_transform(&self) -> Option<String> {
		self.transform.clone()
	}
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
struct ArrowData {
	x: Option<f32>,
	y: Option<f32>,
	center_offset: f32,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
struct TransformOriginData {
	x: String,
	y: String,
}

fn get_element_rect(ref_element: Signal<Option<Rc<MountedData>>>) -> Option<Rect> {
	if let Some(data) = ref_element.peek().as_ref() {
		if let Some(element) = data.try_as_web_event() {
			let rect = element.get_bounding_client_rect();
			return Some(Rect { x: rect.left() as f32, y: rect.top() as f32, width: rect.width() as f32, height: rect.height() as f32 });
		};
	}
	None
}

fn calculate_position(
	reference_rect: &Rect,
	floating_rect: &Rect,
	placement: &Placement,
	arrow_width: f32,
	arrow_height: f32,
	offset: f32,
	align_offset: f32,
) -> (FloatingStyles, ArrowData, TransformOriginData) {
	let side = placement.side();
	let alignment = placement.alignment();

	let (left, top) = match side {
		ESide::Top => ((reference_rect.width / 2.0) - (floating_rect.width / 2.0), -floating_rect.height - offset - arrow_height),
		ESide::Right => (reference_rect.width + offset + arrow_height, (reference_rect.height / 2.0) - (floating_rect.height / 2.0)),
		ESide::Bottom => ((reference_rect.width / 2.0) - (floating_rect.width / 2.0), reference_rect.height + offset + arrow_height),
		ESide::Left => (-floating_rect.width - offset - arrow_height, (reference_rect.height / 2.0) - (floating_rect.height / 2.0)),
	};

	let (left, top) = match (side, alignment) {
		(ESide::Top | ESide::Bottom, Some(Alignment::Start)) => (align_offset, top),
		(ESide::Top | ESide::Bottom, Some(Alignment::End)) => (reference_rect.width - floating_rect.width - align_offset, top),
		(ESide::Right | ESide::Left, Some(Alignment::Start)) => (left, align_offset),
		(ESide::Right | ESide::Left, Some(Alignment::End)) => (left, reference_rect.height - floating_rect.height - align_offset),
		_ => (left, top),
	};

	let (arrow_x, arrow_y) = match side {
		ESide::Top | ESide::Bottom => {
			let x = floating_rect.width / 2.0 - arrow_width / 2.0;
			(Some(x), None)
		},
		ESide::Right | ESide::Left => {
			let y = floating_rect.height / 2.0 - arrow_width / 2.0;
			(None, Some(y))
		},
	};

	let styles = FloatingStyles { position: "absolute".to_string(), top: format!("{}px", top), left: format!("{}px", left), transform: None };

	let arrow_data = ArrowData { x: arrow_x, y: arrow_y, center_offset: 0.0 };

	let (origin_x, origin_y) = match side {
		ESide::Top => (format!("{}px", arrow_x.unwrap_or(floating_rect.width / 2.0)), format!("{}px", floating_rect.height)),
		ESide::Right => ("0px".to_string(), format!("{}px", arrow_y.unwrap_or(floating_rect.height / 2.0))),
		ESide::Bottom => (format!("{}px", arrow_x.unwrap_or(floating_rect.width / 2.0)), "0px".to_string()),
		ESide::Left => (format!("{}px", floating_rect.width), format!("{}px", arrow_y.unwrap_or(floating_rect.height / 2.0))),
	};

	let transform_origin = TransformOriginData { x: origin_x, y: origin_y };

	(styles, arrow_data, transform_origin)
}

#[derive(Props, Clone, PartialEq)]
pub struct PopperProps {
	#[props(default = false)]
	pub is_arrow_hidden: bool,

	#[props(optional, default = None)]
	pub onmounted: Option<EventHandler<Event<MountedData>>>,

	#[props(extends = div, extends = GlobalAttributes)]
	pub attributes: Vec<Attribute>,
	#[props(default = Vec::new())]
	pub extra_attributes: Vec<Attribute>,
	pub children: Element,
}

#[component]
pub fn Popper(props: PopperProps) -> Element {
	use_context_provider::<PopperContext>(|| PopperContext { is_arrow_hidden: props.is_arrow_hidden, anchor: Signal::new(None::<Rc<MountedData>>) });
	rsx! {
		div {
			onmounted: move |event| {
					if let Some(callback) = props.onmounted {
							callback.call(event);
					}
			},
			..props.attributes,
			..props.extra_attributes,
			{props.children}
		}
	}
}

#[derive(Props, PartialEq, Clone)]
pub struct PopperAnchorProps {
	#[props(optional, default = None)]
	onclick: Option<EventHandler<MouseEvent>>,
	#[props(extends = div, extends = GlobalAttributes)]
	pub attributes: Vec<Attribute>,
	#[props(default = Vec::new())]
	pub extra_attributes: Vec<Attribute>,
	#[props(optional)]
	children: Element,
}

#[component]
pub fn PopperAnchor(props: PopperAnchorProps) -> Element {
	let mut context = use_context::<PopperContext>();

	rsx! {
		div {
			onclick: move |event| {
					if let Some(onclick) = props.onclick {
							onclick.call(event);
					}
			},
			onmounted: move |event| context.anchor.set(Some(event.data())),
			..props.attributes,
			..props.extra_attributes,
			{props.children}
		}
	}
}

#[derive(Props, PartialEq, Clone)]
pub struct PopperContentProps {
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
	// #[props(default = false)]
	// hide_when_detached: bool,
	#[props(optional, default = None)]
	pub onmounted: Option<EventHandler<Event<MountedData>>>,
	#[props(optional)]
	onplaced: Option<EventHandler<()>>,
	#[props(extends = div, extends = GlobalAttributes)]
	pub attributes: Vec<Attribute>,
	#[props(default = Vec::new())]
	pub extra_attributes: Vec<Attribute>,
	#[props(optional)]
	children: Element,
}

#[component]
pub fn PopperContent(props: PopperContentProps) -> Element {
	let popper_context = use_context::<PopperContext>();

	let mut content = use_signal(|| None::<Rc<MountedData>>);
	let arrow = use_signal(|| None::<Rc<MountedData>>);
	let mut closure = use_signal(|| None::<Closure<dyn FnMut()>>);

	let placement = use_signal(|| Placement {
		side: props.side,
		alignment: match props.align {
			EAlign::Start => Some(Alignment::Start),
			EAlign::Center => None,
			EAlign::End => Some(Alignment::End),
		},
	});

	let mut floating_styles = use_signal(|| FloatingStyles::default());
	let mut is_positioned = use_signal(|| false);
	let mut arrow_data = use_signal(|| ArrowData { x: None, y: None, center_offset: 0.0 });
	let mut transform_origin = use_signal(|| TransformOriginData { x: "50%".to_string(), y: "50%".to_string() });

	let context = use_context_provider::<PopperContentContext>(|| PopperContentContext { placement, arrow_data, arrow, content });
	let mut new_placement = use_context_provider(|| Signal::new(placement()));

	let mut update_position = move || {
		if let (Some(reference_rect), Some(floating_rect)) = (get_element_rect(popper_context.anchor), get_element_rect(content)) {
			let (arrow_width, arrow_height) = if let Some(arrow) = context.arrow.peek().as_ref() {
				if let Some(element) = arrow.try_as_web_event() {
					let rect = element.get_bounding_client_rect();
					(rect.width() as f32, rect.height() as f32)
				} else {
					(0.0, 0.0)
				}
			} else {
				(0.0, 0.0)
			};

			let init_placement: Placement = placement.peek().clone();
			let current_placement = new_placement.peek().clone();
			let window = window().expect("should have a window in this context");
			let scroll_x = window.page_x_offset().unwrap_or(0.0) as f32;
			let scroll_y = window.page_y_offset().unwrap_or(0.0) as f32;
			let window_width = window.inner_width().unwrap_or(0.into()).as_f64().unwrap_or(0.0) as f32;
			let window_height = window.inner_height().unwrap_or(0.into()).as_f64().unwrap_or(0.0) as f32;

			let should_flip = match current_placement.side {
				ESide::Top => reference_rect.y < floating_rect.height + props.side_offset + arrow_height + props.collision_padding,
				ESide::Right =>
					reference_rect.x + reference_rect.width + floating_rect.width + props.side_offset + arrow_height + props.collision_padding > window_width + scroll_x,
				ESide::Bottom =>
					reference_rect.y + reference_rect.height + floating_rect.height + props.side_offset + arrow_height + props.collision_padding
						> window_height + scroll_y,
				ESide::Left => reference_rect.x < floating_rect.width + props.side_offset + arrow_height + props.collision_padding,
			};

			let should_flip_back = match init_placement.side {
				ESide::Top => reference_rect.y < floating_rect.height + props.side_offset + arrow_height + props.collision_padding,
				ESide::Right =>
					reference_rect.x + reference_rect.width + floating_rect.width + props.side_offset + arrow_height + props.collision_padding > window_width + scroll_x,
				ESide::Bottom =>
					reference_rect.y + reference_rect.height + floating_rect.height + props.side_offset + arrow_height + props.collision_padding
						> window_height + scroll_y,
				ESide::Left => reference_rect.x < floating_rect.width + props.side_offset + arrow_height + props.collision_padding,
			};

			if props.avoid_collisions && ((init_placement.side != current_placement.side && should_flip_back) || should_flip) {
				new_placement.with_mut(|state| {
					state.side = state.side.opposite();
				});
			}

			let (styles, arrow, transform) = calculate_position(
				&reference_rect,
				&floating_rect,
				&new_placement.peek(),
				arrow_width as f32,
				arrow_height as f32,
				props.side_offset,
				props.align_offset,
			);

			floating_styles.set(styles);
			arrow_data.set(arrow);
			transform_origin.set(transform);

			is_positioned.set(true);

			if let Some(onplaced) = props.onplaced {
				onplaced.call(());
			}
		}
	};

	use_effect(move || {
		let window = window().expect("should have a window in this context");

		if popper_context.anchor.read().as_ref().is_some() && content().is_some() {
			update_position();
			if closure.peek().is_none() {
				let closure_fn = Closure::wrap(Box::new(move || {
					update_position();
				}) as Box<dyn FnMut()>);
				closure.set(Some(closure_fn));
			}
		}

		if let Some(closure_fn) = &*closure.read() {
			window.add_event_listener_with_callback("resize", closure_fn.as_ref().unchecked_ref()).expect("should register event listener");
			window.add_event_listener_with_callback("scroll", closure_fn.as_ref().unchecked_ref()).expect("should register event listener");
		}
	});

	use_drop(move || {
		let window = window().expect("should have a window in this context");
		if let Some(closure_fn) = &*closure.read() {
			window.remove_event_listener_with_callback("resize", closure_fn.as_ref().unchecked_ref()).unwrap();
			window.remove_event_listener_with_callback("scroll", closure_fn.as_ref().unchecked_ref()).unwrap();
		}
		// closure.set(None);
	});

	let placed_side = new_placement().side;
	let placed_align = match placement().alignment {
		Some(Alignment::Start) => EAlign::Start,
		Some(Alignment::End) => EAlign::End,
		None => EAlign::Center,
	};

	rsx! {
		FocusTrap {
			position: floating_styles().style_position(),
			top: floating_styles().style_top(),
			left: floating_styles().style_left(),
			transform: floating_styles()
					.style_transform()
					.unwrap_or_else(|| {
							if is_positioned() {
									"none".to_string()
							} else {
									"translate(0, -200%)".to_string()
							}
					}),
			transform_origin: "{transform_origin().x} {transform_origin().y}",
			min_width: "max-content",
			z_index: 100,
			div {
				"data-side": format!("{:?}", placed_side).to_lowercase(),
				"data-align": format!("{:?}", placed_align).to_lowercase(),
				style: if !is_positioned() { "animation: none".to_string() } else { "".to_string() },
				onmounted: move |event| {
						content.set(Some(event.data()));
						if let Some(callback) = props.onmounted {
								callback.call(event)
						}
				},
				..props.attributes,
				..props.extra_attributes,
				{props.children.clone()}
			}
		}
	}
}

#[derive(Props, PartialEq, Clone)]
pub struct PopperArrowProps {
	#[props(default = 10.0)]
	width: f32,
	#[props(default = 5.0)]
	height: f32,
	#[props(extends = svg, extends = GlobalAttributes)]
	pub attributes: Vec<Attribute>,
	#[props(default = Vec::new())]
	pub extra_attributes: Vec<Attribute>,
	#[props(default = None)]
	pub children: Option<Element>,
}

#[component]
pub fn PopperArrow(props: PopperArrowProps) -> Element {
	let popper_context = use_context::<PopperContext>();
	let mut content_context = use_context::<PopperContentContext>();
	let new_placement = use_context::<Signal<Placement>>();
	let arrow_data = content_context.arrow_data.read().clone();

	let arrow_transform = match new_placement().side() {
		ESide::Top => "rotate(0)",
		ESide::Right => "rotate(90deg) translateY(50%) translateX(0)",
		ESide::Bottom => "rotate(180deg)",
		ESide::Left => "rotate(-90deg) translateY(-50%) translateX(-50%)",
	};

	let (left, top) = match new_placement().side() {
		ESide::Top => (arrow_data.x.map(|x| format!("{}px", x - props.width / 2.0)).unwrap_or_else(|| "50%".to_string()), "100%".to_string()),
		ESide::Right => ("0%".to_string(), arrow_data.y.map(|y| format!("{}px", y - props.width / 2.0)).unwrap_or_else(|| "50%".to_string())),
		ESide::Bottom => {
			let x = arrow_data.x.unwrap_or_else(|| props.width / 2.0);
			(format!("{}px", x - props.width / 2.0), "0".to_string())
		},
		ESide::Left => ("100%".to_string(), arrow_data.y.map(|y| format!("{}px", y - props.width / 2.0)).unwrap_or_else(|| "50%".to_string())),
	};

	let arrow_transform_origin = match new_placement().side() {
		ESide::Top => "0 0",
		ESide::Right => "0 50%",
		ESide::Bottom => "center 0",
		ESide::Left => "center center",
	};

	rsx! {
		span {
			position: "absolute",
			transform: arrow_transform,
			left,
			top,
			transform_origin: arrow_transform_origin,
			hidden: popper_context.is_arrow_hidden,
			if let Some(children) = props.children {
				{children}
			} else {
				svg {
					width: "{props.width}",
					height: "{props.height}",
					view_box: "0 0 30 10",
					style: "display: block",
					preserve_aspect_ratio: "none",
					onmounted: move |event| content_context.arrow.set(Some(event.data())),
					..props.attributes,
					..props.extra_attributes,
					polygon { points: "0,0 30,0 15,10", fill: "currentColor" }
				}
			}
		}
	}
}
