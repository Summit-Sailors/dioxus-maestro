use {
	crate::{
		focus_trap::FocusTrap,
		popper::component::utils::{
			Alignment, ArrowData, FloatingStyles, Placement, Rect, TransformOriginData, calculate_position, find_positioned_parent, find_scrollable_parents,
			get_element_rect,
		},
		shared::{EAlign, ESide},
	},
	dioxus::{prelude::*, web::WebEventExt},
	std::rc::Rc,
	web_sys::{
		wasm_bindgen::{JsCast, prelude::Closure},
		window,
	},
};

#[derive(Clone, Debug, PartialEq)]
pub struct PopperContext {
	anchor: Signal<Option<Rc<MountedData>>>,
}

#[derive(Clone, Debug, PartialEq)]
pub struct PopperContentContext {
	content: Signal<Option<Rc<MountedData>>>,
	placement: Signal<Placement>,
	arrow_data: Signal<ArrowData>,
	arrow: Signal<Option<Rc<MountedData>>>,
}

#[derive(Props, Clone, PartialEq)]
pub struct PopperProps {
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
	let PopperProps { onmounted, attributes, extra_attributes, children } = props;

	let anchor = use_signal(|| None::<Rc<MountedData>>);

	use_context_provider::<PopperContext>(|| PopperContext { anchor });

	rsx! {
		div {
			onmounted: move |event| {
					if let Some(callback) = onmounted {
							callback.call(event);
					}
			},
			..attributes,
			..extra_attributes,
			{children}
		}
	}
}

#[derive(Props, PartialEq, Clone)]
pub struct PopperAnchorProps {
	#[props(optional, default = None)]
	onclick: Option<EventHandler<MouseEvent>>,
	#[props(default = None)]
	pub onkeydown: Option<EventHandler<Event<KeyboardData>>>,
	#[props(default = None)]
	pub onkeyup: Option<EventHandler<Event<KeyboardData>>>,
	#[props(default = None)]
	pub onfocus: Option<EventHandler<Event<FocusData>>>,
	#[props(default = None)]
	pub onblur: Option<EventHandler<Event<FocusData>>>,
	#[props(default = None)]
	pub onmousedown: Option<EventHandler<Event<MouseData>>>,
	#[props(default = None)]
	pub onmouseup: Option<EventHandler<Event<MouseData>>>,
	#[props(default = None)]
	pub onmouseenter: Option<EventHandler<Event<MouseData>>>,
	#[props(default = None)]
	pub onmouseleave: Option<EventHandler<Event<MouseData>>>,
	#[props(extends = div, extends = GlobalAttributes)]
	pub attributes: Vec<Attribute>,
	#[props(default = Vec::new())]
	pub extra_attributes: Vec<Attribute>,
	#[props(optional)]
	children: Element,
}

#[component]
pub fn PopperAnchor(props: PopperAnchorProps) -> Element {
	let PopperAnchorProps {
		onclick,
		onkeydown,
		onkeyup,
		onfocus,
		onblur,
		onmousedown,
		onmouseup,
		onmouseenter,
		onmouseleave,
		attributes,
		extra_attributes,
		children,
	} = props;

	let mut context = use_context::<PopperContext>();

	rsx! {
		div {
			max_width: "max-content",
			width: "max-content",
			onclick: move |event| {
					if let Some(onclick) = onclick {
							onclick.call(event);
					}
			},
			onmousedown: move |event| {
					if let Some(handler) = onmousedown {
							handler.call(event);
					}
			},
			onkeydown: move |event| {
					if let Some(handler) = onkeydown {
							handler.call(event);
					}
			},
			onkeyup: move |event| {
					if let Some(handler) = onkeyup {
							handler.call(event);
					}
			},
			onmouseup: move |event| {
					if let Some(handler) = onmouseup {
							handler.call(event);
					}
			},
			onmouseenter: move |event| {
					if let Some(handler) = onmouseenter {
							handler.call(event);
					}
			},
			onmouseleave: move |event| {
					if let Some(handler) = onmouseleave {
							handler.call(event);
					}
			},
			onfocus: move |event| {
					if let Some(handler) = onfocus {
							handler.call(event);
					}
			},
			onblur: move |event| {
					if let Some(handler) = onblur {
							handler.call(event);
					}
			},
			onmounted: move |event| context.anchor.set(Some(event.data())),
			..attributes,
			..extra_attributes,
			{children}
		}
	}
}

#[derive(Props, PartialEq, Clone)]
pub struct PopperContentProps {
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
	onplaced: Option<EventHandler<()>>,

	#[props(default = None)]
	pub onkeydown: Option<EventHandler<Event<KeyboardData>>>,
	#[props(default = None)]
	pub onkeyup: Option<EventHandler<Event<KeyboardData>>>,
	#[props(default = None)]
	pub onfocus: Option<EventHandler<Event<FocusData>>>,
	#[props(default = None)]
	pub onblur: Option<EventHandler<Event<FocusData>>>,
	#[props(default = None)]
	pub onmousedown: Option<EventHandler<Event<MouseData>>>,
	#[props(default = None)]
	pub onmouseup: Option<EventHandler<Event<MouseData>>>,
	#[props(default = None)]
	pub onmouseenter: Option<EventHandler<Event<MouseData>>>,
	#[props(default = None)]
	pub onmouseleave: Option<EventHandler<Event<MouseData>>>,

	#[props(extends = div, extends = GlobalAttributes)]
	pub attributes: Vec<Attribute>,
	#[props(default = Vec::new())]
	pub extra_attributes: Vec<Attribute>,
	#[props(default = Vec::new())]
	styled_attributes: Vec<Attribute>,
	#[props(optional)]
	children: Element,
}

#[component]
pub fn PopperContent(props: PopperContentProps) -> Element {
	let PopperContentProps {
		side,
		side_offset,
		align,
		align_offset,
		avoid_collisions,
		collision_padding,
		onplaced,
		onkeydown,
		onkeyup,
		onfocus,
		onblur,
		onmousedown,
		onmouseup,
		onmouseenter,
		onmouseleave,
		attributes,
		extra_attributes,
		styled_attributes,
		children,
	} = props;

	let popper_context = use_context::<PopperContext>();

	let content = use_context::<Signal<Option<Rc<MountedData>>>>();
	let arrow = use_signal(|| None::<Rc<MountedData>>);
	let mut closure = use_signal(|| None::<Closure<dyn FnMut()>>);
	let mut parents = use_signal::<Vec<web_sys::Element>>(Vec::new);
	let mut parent = use_signal::<Option<web_sys::Element>>(|| None);
	let mut last_update = use_signal(|| 0.0);
	let placement = Signal::new(Placement {
		side: side(),
		alignment: match align() {
			EAlign::Start => Some(Alignment::Start),
			EAlign::Center => None,
			EAlign::End => Some(Alignment::End),
		},
	});
	let mut floating_styles = use_signal(|| FloatingStyles::default());
	let mut is_positioned = use_signal(|| false);
	let mut arrow_data = use_signal(|| ArrowData { x: None, y: None, center_offset: 0.0 });
	let mut transform_origin = use_signal(|| TransformOriginData { x: "50%".to_string(), y: "50%".to_string() });
	let mut frame_id = use_signal(|| None::<i32>);

	let context = use_context_provider::<PopperContentContext>(|| PopperContentContext { placement, arrow_data, arrow, content });
	let mut new_placement = use_context_provider(|| Signal::new(placement()));
	let mut anchor_width = use_signal(|| None::<f32>);
	let mut anchor_height = use_signal(|| None::<f32>);
	let mut floating_width = use_signal(|| None::<f32>);
	let mut floating_height = use_signal(|| None::<f32>);

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

			anchor_width.set(Some(reference_rect.width));
			anchor_height.set(Some(reference_rect.height));
			floating_width.set(Some(floating_rect.width));
			floating_height.set(Some(floating_rect.height));

			let current_placement = new_placement.peek().clone();
			let init_placement = placement.peek().clone();

			let window = window().expect("should have a window in this context");
			let scroll_x = window.page_x_offset().unwrap_or(0.0) as f32;
			let scroll_y = window.page_y_offset().unwrap_or(0.0) as f32;
			let window_width = window.inner_width().unwrap_or(0.into()).as_f64().unwrap_or(0.0) as f32;
			let window_height = window.inner_height().unwrap_or(0.into()).as_f64().unwrap_or(0.0) as f32;

			let adjusted_reference_rect =
				Rect { x: reference_rect.x - scroll_x, y: reference_rect.y - scroll_y, width: reference_rect.width, height: reference_rect.height };

			let should_flip = match current_placement.side() {
				ESide::Top => adjusted_reference_rect.y < floating_rect.height + side_offset() + arrow_height + collision_padding(),
				ESide::Right =>
					adjusted_reference_rect.x + adjusted_reference_rect.width + floating_rect.width + side_offset() + arrow_height + collision_padding() > window_width,
				ESide::Bottom =>
					adjusted_reference_rect.y + adjusted_reference_rect.height + floating_rect.height + side_offset() + arrow_height + collision_padding() > window_height,
				ESide::Left => adjusted_reference_rect.x < floating_rect.width + side_offset() + arrow_height + collision_padding(),
			};

			let has_collision_with_original = match init_placement.side() {
				ESide::Top => adjusted_reference_rect.y < floating_rect.height + side_offset() + arrow_height + collision_padding(),
				ESide::Right =>
					adjusted_reference_rect.x + adjusted_reference_rect.width + floating_rect.width + side_offset() + arrow_height + collision_padding() > window_width,
				ESide::Bottom =>
					adjusted_reference_rect.y + adjusted_reference_rect.height + floating_rect.height + side_offset() + arrow_height + collision_padding() > window_height,
				ESide::Left => adjusted_reference_rect.x < floating_rect.width + side_offset() + arrow_height + collision_padding(),
			};

			if avoid_collisions() {
				if should_flip {
					new_placement.with_mut(|state| {
						state.side = state.side.opposite();
					});
				} else if current_placement.side != init_placement.side && !has_collision_with_original {
					new_placement.with_mut(|state| {
						state.side = init_placement.side;
					});
				}
			}

			let (styles, arrow, transform) = calculate_position(
				parent(),
				&reference_rect,
				&floating_rect,
				&new_placement.peek(),
				arrow_width as f32,
				arrow_height as f32,
				side_offset(),
				align_offset(),
			);

			floating_styles.set(styles);
			arrow_data.set(arrow);
			transform_origin.set(transform);

			is_positioned.set(true);

			if let Some(onplaced) = onplaced {
				onplaced.call(());
			}
		}
	};

	let throttled_update_position = use_callback(move |l_update: f64| {
		let window = window().expect("No window");
		let performance = window.performance().expect("No Performance");
		let now = performance.now();
		if now - l_update > 15.0 {
			update_position();
			last_update.set(now);
		}
	});

	use_effect(move || {
		let window = window().expect("should have a window in this context");

		if popper_context.anchor.read().as_ref().is_some() && content().is_some() {
			update_position();
			if closure.peek().is_none() {
				let win_clone = window.clone();
				let closure_fn = Closure::wrap(Box::new(move || {
					let inner_closure = Closure::wrap(Box::new(move || {
						let l_update = *last_update.peek();
						throttled_update_position(l_update);
					}) as Box<dyn FnMut()>);
					let frame_id_value = win_clone.request_animation_frame(inner_closure.as_ref().unchecked_ref()).expect("should register request frame");
					frame_id.set(Some(frame_id_value));
					inner_closure.forget();
				}) as Box<dyn FnMut()>);
				closure.set(Some(closure_fn));
			}
		}

		if let Some(content) = content().as_ref() {
			let containers = find_scrollable_parents(content);
			parents.set(containers)
		}

		if let Some(closure_fn) = &*closure.read() {
			for parent in parents() {
				parent.add_event_listener_with_callback("resize", closure_fn.as_ref().unchecked_ref()).expect("should register event listener");
				parent.add_event_listener_with_callback("scroll", closure_fn.as_ref().unchecked_ref()).expect("should register event listener");
			}
			window.add_event_listener_with_callback("resize", closure_fn.as_ref().unchecked_ref()).expect("should register event listener");
			window.add_event_listener_with_callback("scroll", closure_fn.as_ref().unchecked_ref()).expect("should register event listener");
		}
	});

	use_effect(move || {
		if let Some(anchor) = popper_context.anchor.read().as_ref() {
			let fixed_parent = find_positioned_parent(anchor);
			parent.set(fixed_parent);
		}
	});

	#[cfg(target_arch = "wasm32")]
	{
		use_drop(move || {
			let window = window().expect("should have a window in this context");
			if let Some(frame_id) = *frame_id.peek() {
				window.cancel_animation_frame(frame_id).ok();
			}
			if let Some(closure_fn) = &*closure.peek() {
				window.remove_event_listener_with_callback("resize", closure_fn.as_ref().unchecked_ref()).expect("should remove event listener");
				window.remove_event_listener_with_callback("scroll", closure_fn.as_ref().unchecked_ref()).expect("should remove event listener");
				for parent in parents() {
					parent.remove_event_listener_with_callback("resize", closure_fn.as_ref().unchecked_ref()).expect("should remove event listener");
					parent.remove_event_listener_with_callback("scroll", closure_fn.as_ref().unchecked_ref()).expect("should remove event listener");
				}
			}
		});
	};

	let placed_side = new_placement().side;

	let placed_align = match placement().alignment {
		Some(Alignment::Start) => EAlign::Start,
		Some(Alignment::End) => EAlign::End,
		None => EAlign::Center,
	};

	let mut attrs = attributes.clone();
	attrs.extend(extra_attributes.clone());

	let mut style_attrs: Vec<Attribute> = styled_attributes.clone();

	if !is_positioned() {
		style_attrs.push(Attribute::new("animation", "none", Some("style"), false));
	}
	if let Some(height) = anchor_height() {
		style_attrs.push(Attribute::new("--maestro-headless-popper-anchor-height", format!("{}px", height), Some("style"), false));
	}
	if let Some(width) = anchor_width() {
		style_attrs.push(Attribute::new("--maestro-headless-popper-anchor-width", format!("{}px", width), Some("style"), false));
	}
	if let Some(height) = floating_height() {
		style_attrs.push(Attribute::new("--maestro-headless-popper-content-height", format!("{}px", height), Some("style"), false));
	}
	if let Some(width) = floating_width() {
		style_attrs.push(Attribute::new("--maestro-headless-popper-content-width", format!("{}px", width), Some("style"), false));
	}
	style_attrs.push(Attribute::new(
		"--maestro-headless-popper-content-transform-origin",
		format!("{} {}", transform_origin().x, transform_origin().y),
		Some("style"),
		false,
	));

	rsx! {
		div {
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
			min_width: "max-content",
			max_height: "max-content",
			will_change: "transform",
			..style_attrs.clone(),
			FocusTrap {
				"data-side": format!("{:?}", placed_side).to_lowercase(),
				"data-align": format!("{:?}", placed_align).to_lowercase(),
				onmousedown: move |event| {
						if let Some(handler) = onmousedown {
								handler.call(event);
						}
				},
				onkeydown: move |event| {
						if let Some(handler) = onkeydown {
								handler.call(event);
						}
				},
				onkeyup: move |event| {
						if let Some(handler) = onkeyup {
								handler.call(event);
						}
				},
				onmouseup: move |event| {
						if let Some(handler) = onmouseup {
								handler.call(event);
						}
				},
				onmouseenter: move |event| {
						if let Some(handler) = onmouseenter {
								handler.call(event);
						}
				},
				onmouseleave: move |event| {
						if let Some(handler) = onmouseleave {
								handler.call(event);
						}
				},
				onfocus: move |event| {
						if let Some(handler) = onfocus {
								handler.call(event);
						}
				},
				onblur: move |event| {
						if let Some(handler) = onblur {
								handler.call(event);
						}
				},
				extra_attributes: attrs.clone(),
				{children.clone()}
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
	let PopperArrowProps { width, height, attributes, extra_attributes, children } = props;

	let mut content_context = use_context::<PopperContentContext>();
	let new_placement = use_context::<Signal<Placement>>();
	let arrow_data = content_context.arrow_data.read().clone();

	let arrow_transform = match new_placement().side() {
		ESide::Top => "rotate(0) translateX(-50%)",
		ESide::Right => "rotate(90deg) translateY(50%) translateX(-50%)",
		ESide::Bottom => "rotate(180deg) translateX(50%)",
		ESide::Left => "rotate(-90deg) translateY(-50%)",
	};

	let (left, top) = match new_placement().side() {
		ESide::Top => (arrow_data.x.map(|x| format!("{}px", x)).unwrap_or_else(|| "50%".to_string()), "100%".to_string()),
		ESide::Right => ("0%".to_string(), arrow_data.y.map(|y| format!("{}px", y)).unwrap_or_else(|| "50%".to_string())),
		ESide::Bottom => (arrow_data.x.map(|x| format!("{}px", x)).unwrap_or_else(|| "50%".to_string()), "0".to_string()),
		ESide::Left => ("100%".to_string(), arrow_data.y.map(|y| format!("{}px", y)).unwrap_or_else(|| "50%".to_string())),
	};

	let arrow_transform_origin = match new_placement().side() {
		ESide::Top => "center bottom",
		ESide::Right => "0 center",
		ESide::Bottom => "center top",
		ESide::Left => "center center",
	};

	rsx! {
		span {
			position: "absolute",
			transform: arrow_transform,
			left,
			top,
			transform_origin: arrow_transform_origin,
			if let Some(children) = children {
				{children}
			} else {
				svg {
					width,
					height,
					view_box: "0 0 30 10",
					style: "display: block",
					preserve_aspect_ratio: "none",
					onmounted: move |event| content_context.arrow.set(Some(event.data())),
					..attributes,
					..extra_attributes,
					polygon { points: "0,0 30,0 15,10", fill: "currentColor" }
				}
			}
		}
	}
}
