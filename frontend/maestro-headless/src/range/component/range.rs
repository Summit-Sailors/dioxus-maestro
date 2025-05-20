use {
	crate::{
		range::component::utils::{
			clamp, convert_value_to_percentage, get_closest_value_index, get_decimal_count, get_label, get_thumb_in_bounds_offset, has_min_steps_between_values,
			linear_scale, round_value,
		},
		shared::{EOrientation, UseControllableStateParams, use_controllable_state},
	},
	dioxus::{prelude::*, web::WebEventExt},
	std::{fmt::Debug, rc::Rc},
	uuid::Uuid,
	web_sys::{
		EventTarget, HtmlElement,
		wasm_bindgen::{JsCast, prelude::Closure},
	},
};

#[derive(Clone, Debug, PartialEq)]
pub struct RangeContext {
	pub disabled: ReadOnlySignal<bool>,
	pub min: f32,
	pub max: f32,
	pub value_index_to_change: Signal<usize>,
	pub thumbs: Signal<Vec<(Uuid, Rc<MountedData>)>>,
	pub values: Memo<Vec<f32>>,
	pub set_values: Callback<Vec<f32>>,
	pub orientation: ReadOnlySignal<EOrientation>,
}

#[derive(Clone, PartialEq, Props)]
pub struct RangeRootProps {
	#[props(optional, default = ReadOnlySignal::new(Signal::new(None)))]
	pub value: ReadOnlySignal<Option<Vec<f32>>>,
	#[props(optional, default = Vec::new())]
	pub default_value: Vec<f32>,
	#[props(default = None)]
	pub on_value_change: Option<Callback<Vec<f32>>>,

	#[props(optional, default = ReadOnlySignal::new(Signal::new(false)))]
	pub disabled: ReadOnlySignal<bool>,
	#[props(default = false)]
	pub required: bool,
	#[props(optional, default = ReadOnlySignal::new(Signal::new(EOrientation::Horizontal)))]
	pub orientation: ReadOnlySignal<EOrientation>,
	#[props(optional, default = 0.0)]
	pub min: f32,
	#[props(optional, default = 100.0)]
	pub max: f32,
	#[props(optional, default = 1.0)]
	pub step: f32,
	#[props(optional, default = 0.0)]
	pub min_steps_between_thumbs: f32,

	#[props(extends = GlobalAttributes, extends = div)]
	pub attributes: Vec<Attribute>,
	#[props(default = Vec::new())]
	pub extra_attributes: Vec<Attribute>,
	pub children: Element,
}

#[component]
pub fn RangeRoot(props: RangeRootProps) -> Element {
	let RangeRootProps {
		value,
		default_value,
		on_value_change,
		disabled,
		required,
		orientation,
		min,
		max,
		step,
		min_steps_between_thumbs,
		attributes,
		extra_attributes,
		children,
	} = props;

	let is_controlled = use_hook(move || value().is_some());
	let default = if default_value.is_empty() { Vec::from([min]) } else { default_value.clone() };

	let mut value_index_to_change = use_signal(|| 0_usize);
	let thumb_refs = use_signal::<Vec<(Uuid, Rc<MountedData>)>>(Vec::new);
	let mut range_ref = use_signal(|| None::<Rc<MountedData>>);
	let mut rect_ref = use_signal(|| None::<web_sys::DomRect>);

	let mut closure_pointer_down_ref = use_signal(|| None::<Closure<dyn FnMut(web_sys::PointerEvent)>>);
	let mut closure_pointer_move_ref = use_signal(|| None::<Closure<dyn FnMut(web_sys::PointerEvent)>>);

	let mut dragging_thumb_id = use_signal(|| None::<Uuid>);

	let callback = use_callback(move |values: Vec<f32>| {
		if let Some(callback) = on_value_change {
			callback(values);
		}
	});

	let (values, set_values) =
		use_controllable_state(UseControllableStateParams { is_controlled, prop: value, default_prop: default, on_change: Some(callback) });

	use_effect(move || {
		let _ = orientation();
		if let Some(range_ref) = range_ref.read().as_ref().and_then(|node| node.try_as_web_event()) {
			rect_ref.set(Some(range_ref.get_bounding_client_rect()));
		}
	});
	let update_values = use_callback(move |(new_value, at_index): (f32, usize)| {
		let current_values = values().clone();
		let decimal_count = get_decimal_count(step);
		let snap_to_step = round_value(((new_value - min) / step).round() * step + min, decimal_count);
		let next_value = clamp(snap_to_step, min, max);

		let mut next_values = current_values.clone();
		next_values[at_index] = next_value;

		if has_min_steps_between_values(&next_values, min_steps_between_thumbs * step) {
			let has_changed = next_values != current_values;
			value_index_to_change.set(at_index);

			if has_changed {
				set_values(next_values.clone());
			}
		}
	});

	let handle_slide_start = use_callback(move |new_value: f32| {
		let closest_index = get_closest_value_index(&values(), new_value);
		if let Some((thumb_id, _)) = thumb_refs.peek().get(closest_index) {
			dragging_thumb_id.set(Some(*thumb_id));
		}
		update_values((new_value, closest_index));
	});

	let handle_slide_move = use_callback(move |new_value: f32| {
		let idx = *value_index_to_change.peek();
		update_values((new_value, idx));
	});

	let mut get_value_from_pointer = move |pointer_position: f32| {
		let rect = rect_ref.peek().clone().or_else(|| {
			if let Some(range_ref) = range_ref.peek().clone() {
				let rect = range_ref.try_as_web_event().unwrap().get_bounding_client_rect();
				return Some(rect);
			}
			None
		});

		if let Some(rect) = rect {
			let input = [0.0, if orientation() == EOrientation::Horizontal { rect.width() as f32 } else { rect.height() as f32 }];
			let output = if orientation() == EOrientation::Horizontal { [min, max] } else { [max, min] };
			let value = linear_scale(input, output);
			rect_ref.set(Some(rect.clone()));
			if orientation() == EOrientation::Horizontal { value(pointer_position - rect.left() as f32) } else { value(pointer_position - rect.top() as f32) }
		} else {
			0.0
		}
	};

	use_effect(move || {
		if closure_pointer_down_ref.peek().is_none() {
			let closure = Closure::wrap(Box::new(move |event: web_sys::PointerEvent| {
				if let (Some(range_element), Some(target)) =
					(range_ref.peek().as_ref().and_then(|node| node.try_as_web_event()), event.target().and_then(|t: EventTarget| t.dyn_into::<HtmlElement>().ok()))
				{
					rect_ref.set(Some(range_element.get_bounding_client_rect()));
					if range_element.contains(Some(&target)) {
						target.set_pointer_capture(event.pointer_id());
						event.prevent_default();
						if thumb_refs.peek().clone().iter().any(|data: &(Uuid, Rc<MountedData>)| {
							if let Some(thumb_element) = data.1.try_as_web_event().and_then(|element| element.dyn_into::<HtmlElement>().ok()) {
								thumb_element == target
							} else {
								false
							}
						}) {
							target.focus().ok();
						} else {
							handle_slide_start(get_value_from_pointer(if orientation() == EOrientation::Horizontal {
								event.client_x() as f32
							} else {
								event.client_y() as f32
							}));
						}
					}
				}
			}) as Box<dyn FnMut(web_sys::PointerEvent)>);
			closure_pointer_down_ref.set(Some(closure));
		}

		if let Some(closure) = &*closure_pointer_down_ref.read() {
			if let Some(range_element) = range_ref.read().as_ref().and_then(|node| node.try_as_web_event()) {
				range_element.add_event_listener_with_callback("pointerdown", closure.as_ref().unchecked_ref()).expect("Cannot add onpointerdown listener");
			}
		}
	});

	use_effect(move || {
		if closure_pointer_move_ref.peek().is_none() {
			let closure = Closure::wrap(Box::new(move |event: web_sys::PointerEvent| {
				if let Some(target) = event.target().and_then(|t: EventTarget| t.dyn_into::<HtmlElement>().ok()) {
					if target.has_pointer_capture(event.pointer_id()) {
						handle_slide_move(get_value_from_pointer(if orientation() == EOrientation::Horizontal {
							event.client_x() as f32
						} else {
							event.client_y() as f32
						}));
					}
				}
			}) as Box<dyn FnMut(web_sys::PointerEvent)>);
			closure_pointer_move_ref.set(Some(closure));
		}

		if let Some(closure) = &*closure_pointer_move_ref.read() {
			if let Some(range_element) = range_ref.read().as_ref().and_then(|node| node.try_as_web_event()) {
				range_element.add_event_listener_with_callback("pointermove", closure.as_ref().unchecked_ref()).expect("Cannot add onpointermove listener");
			}
		}
	});

	#[cfg(target_arch = "wasm32")]
	{
		use_drop(move || {
			spawn(async move {
				if let Some(closure) = closure_pointer_down_ref.peek().as_ref() {
					if let Some(range_element) = range_ref.read().as_ref().and_then(|node| node.try_as_web_event()) {
						range_element.remove_event_listener_with_callback("pointerdown", closure.as_ref().unchecked_ref()).expect("Cannot remove onpointerdown listener");
					}
				}
				closure_pointer_down_ref.set(None);
				if let Some(closure) = closure_pointer_move_ref.peek().as_ref() {
					if let Some(range_element) = range_ref.read().as_ref().and_then(|node| node.try_as_web_event()) {
						range_element.remove_event_listener_with_callback("pointermove", closure.as_ref().unchecked_ref()).expect("Cannot remove onpointermove listener");
					}
				}
			});
		})
	};

	use_context_provider::<RangeContext>(|| RangeContext { disabled, min, max, value_index_to_change, thumbs: thumb_refs, values, set_values, orientation });

	rsx! {
		div {
			position: "relative",
			role: "group",
			aria_roledescription: "range slider",
			aria_disabled: disabled(),
			aria_required: required,
			"data-disabled": disabled(),
			"data-required": required,
			"data-orientation": orientation().to_string(),
			onmounted: move |event| range_ref.set(Some(event.data())),
			onkeydown: move |event: KeyboardEvent| {
					if !disabled() {
							match event.key() {
									Key::Home => {
											update_values((min, 0));
											event.prevent_default();
									}
									Key::End => {
											update_values((max, values().len() - 1));
											event.prevent_default();
									}
									Key::ArrowUp
									| Key::ArrowRight
									| Key::ArrowDown
									| Key::ArrowLeft
									| Key::PageUp
									| Key::PageDown => {
											let is_page_key = [Key::PageUp, Key::PageDown]
													.contains(&event.key());
											let multiplier = if is_page_key { 10.0 } else { 1.0 };
											let at_index = *value_index_to_change.peek();
											let current_value = values.peek().clone()[at_index];
											let is_increment = match (event.key(), orientation()) {
													(Key::ArrowUp | Key::ArrowRight | Key::PageUp, _) => true,
													(Key::ArrowDown | Key::ArrowLeft | Key::PageDown, _) => false,
													_ => true,
											};
											let step_in_direction = step * multiplier
													* if is_increment { 1.0 } else { -1.0 };
											update_values((current_value + step_in_direction, at_index));
											event.prevent_default();
									}
									_ => {}
							}
					}
			},
			..attributes,
			..extra_attributes,
			{children}
		}
	}
}

#[derive(Clone, PartialEq, Props)]
pub struct RangeTrackProps {
	#[props(extends = GlobalAttributes, extends = div)]
	pub attributes: Vec<Attribute>,
	#[props(default = Vec::new())]
	pub extra_attributes: Vec<Attribute>,
	pub children: Element,
}

#[component]
pub fn RangeTrack(props: RangeTrackProps) -> Element {
	let context = use_context::<RangeContext>();

	rsx! {
		div {
			position: "relative",
			"data-disabled": *context.disabled.read(),
			"data-orientation": context.orientation.read().to_string(),
			aria_orientation: context.orientation.read().to_string(),
			aria_disabled: *context.disabled.read(),
			..props.attributes,
			..props.extra_attributes,
			{props.children.clone()}
		}
	}
}

#[derive(Clone, PartialEq, Props)]
pub struct RangeProps {
	#[props(extends = div, extends = GlobalAttributes)]
	pub attributes: Vec<Attribute>,
	#[props(default = Vec::new())]
	pub extra_attributes: Vec<Attribute>,
	pub children: Element,
}

#[component]
pub fn Range(props: RangeProps) -> Element {
	let context = use_context::<RangeContext>();
	let percentages = context.values.read().clone().iter().map(|&value| convert_value_to_percentage(value, context.min, context.max)).collect::<Vec<f32>>();
	let offset_start = if context.values.read().clone().len() > 1 { percentages.iter().cloned().reduce(f32::min).unwrap_or(0.0) } else { 0.0 };
	let offset_end = 100.0 - percentages.iter().cloned().reduce(f32::max).unwrap_or(100.0);
	let (start, end) = match *context.orientation.read() {
		EOrientation::Horizontal => ("left", "right"),
		EOrientation::Vertical => ("bottom", "top"),
	};

	let dim = match *context.orientation.read() {
		EOrientation::Horizontal => "height",
		EOrientation::Vertical => "width",
	};

	let mut attributes = props.attributes.clone();
	attributes.extend(props.extra_attributes.clone());
	attributes.push(Attribute::new(start, format!("{}%", offset_start), Some("style"), false));
	attributes.push(Attribute::new(end, format!("{}%", offset_end), Some("style"), false));
	attributes.push(Attribute::new(dim, "100%", Some("style"), false));

	rsx! {
		div {
			position: "absolute",
			"data-disabled": *context.disabled.read(),
			"data-orientation": context.orientation.read().to_string(),
			aria_orientation: context.orientation.read().to_string(),
			aria_disabled: *context.disabled.read(),
			..attributes.clone(),
			{props.children}
		}
	}
}

#[derive(Clone, PartialEq, Props)]
pub struct RangeThumbProps {
	#[props(extends = div, extends = GlobalAttributes)]
	pub attributes: Vec<Attribute>,
	#[props(default = Vec::new())]
	pub extra_attributes: Vec<Attribute>,
	pub children: Element,
}

#[component]
pub fn RangeThumb(props: RangeThumbProps) -> Element {
	let mut context = use_context::<RangeContext>();
	let id = use_signal(Uuid::new_v4);
	let mut index = use_signal(|| 0_i32);
	let mut size = use_signal(|| None::<web_sys::DomRect>);

	use_effect(move || {
		let thumbs = context.thumbs.read();
		let idx = thumbs.iter().position(|(uuid, _)| *uuid == id());
		if let Some(i) = idx {
			index.set(i as i32);
		}
	});

	use_drop(move || {
		context.thumbs.write().retain(|(uuid, _)| *uuid != id());
	});

	use_effect(move || {
		if let Some(thumb) = context.thumbs.read().clone().iter().find(|(uuid, _)| uuid == &id()) {
			index.set(context.thumbs.peek().iter().position(|(t, _)| t == &thumb.0).map(|i| i as i32).unwrap_or(0));
			if let Some(rect) = thumb.1.try_as_web_event().and_then(|thumb| Some(thumb.get_bounding_client_rect())) {
				size.set(Some(rect));
			}
		}
	});

	let percent = match *context.orientation.read() {
		EOrientation::Horizontal =>
			context.values.read().clone().get(index() as usize).map(|v| convert_value_to_percentage(*v, context.min, context.max)).unwrap_or(0.0),
		EOrientation::Vertical =>
			context.values.read().clone().get(index() as usize).map(|v| convert_value_to_percentage(*v, context.max, context.min)).unwrap_or(0.0),
	};

	let label = get_label(index() as usize, context.values.read().clone().len());
	let orientation_size = size().as_ref().map(|s| if *context.orientation.read() == EOrientation::Horizontal { s.width() as f32 } else { s.height() as f32 });

	let thumb_in_bounds_offset = orientation_size.map(|os| get_thumb_in_bounds_offset(os, percent)).unwrap_or(0.0);

	let start = match *context.orientation.read() {
		EOrientation::Horizontal => "left",
		EOrientation::Vertical => "top",
	};

	let mut attributes = props.attributes.clone();
	attributes.extend(props.extra_attributes.clone());
	attributes.push(Attribute::new(start, format!("calc({}% + {}px)", percent, thumb_in_bounds_offset), Some("style"), false));

	rsx! {
		div {
			aria_label: label,
			role: "slider",
			aria_valuemin: context.min,
			aria_valuenow: context.values.read().clone().get(index() as usize).cloned(),
			aria_valuemax: context.max,
			aria_orientation: context.orientation.read().to_string(),
			"data-orientation": context.orientation.read().to_string(),
			"data-disabled": *context.disabled.read(),
			aria_disabled: *context.disabled.read(),
			tabindex: if *context.disabled.read() { "-1" } else { "0" },
			display: if context.values.read().clone().get(index() as usize).is_none() { "none" } else { "flex" },
			transform: if *context.orientation.read() == EOrientation::Horizontal { "translateY(0%) translateX(-50%)" } else { "translateX(0%) translateY(-50%)" },
			position: "absolute",
			onmounted: move |event| {
					let new_index = context.thumbs.peek().len();
					context.thumbs.write().push((id(), event.data().clone()));
					index.set(new_index as i32);
			},
			onfocus: move |_| {
					context.value_index_to_change.set(*index.peek() as usize);
			},
			..attributes.clone(),
			{props.children}
		}
	}
}
