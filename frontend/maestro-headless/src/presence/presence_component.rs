use {
	dioxus::{prelude::*, web::WebEventExt},
	std::rc::Rc,
	web_sys::{
		AnimationEvent, CssStyleDeclaration, HtmlElement, TransitionEvent,
		wasm_bindgen::{JsCast, prelude::Closure},
	},
};

#[derive(Props, PartialEq, Clone)]
pub struct PresenceProps {
	present: ReadOnlySignal<bool>,
	node_ref: Signal<Option<Rc<MountedData>>>,
	children: Element,
}

#[component]
pub fn Presence(props: PresenceProps) -> Element {
	let PresenceProps { present, node_ref, children } = props;
	let memo_present = use_memo(move || present());
	let is_present = use_presence(memo_present, node_ref);

	rsx! {
		if is_present() {
			{children}
		}
	}
}

#[derive(Clone, Debug)]
struct AnimationState {
	animation_name: String,
	has_transition: bool,
	is_animating: bool,
}

#[derive(Clone, Debug, PartialEq, Eq, Copy)]
enum EMountState {
	Mounted,
	Unmounted,
	UnmountSuspend,
}

pub fn use_presence(present: Memo<bool>, node_ref: Signal<Option<Rc<MountedData>>>) -> Memo<bool> {
	let mut styles_ref = use_signal(|| None::<CssStyleDeclaration>);
	let mut prev_present_ref = use_signal(|| present());
	let mut animation_state = use_signal(|| AnimationState { animation_name: "none".to_string(), has_transition: false, is_animating: false });
	let mut state = use_signal(|| if present() { EMountState::Mounted } else { EMountState::Unmounted });
	let is_present = use_memo(move || match state() {
		EMountState::Unmounted => false,
		_ => true,
	});

	use_effect(move || {
		let prev_present = *prev_present_ref.peek();
		let has_present_changed = prev_present != present();

		if has_present_changed {
			if present() {
				state.set(EMountState::Mounted);
			} else {
				if let Some(element) = node_ref().and_then(|node| node.try_as_web_event().and_then(|x| x.dyn_into::<HtmlElement>().ok())) {
					if let Some(computed_style) = web_sys::window().and_then(|w| w.get_computed_style(&element).ok()).unwrap_or(None) {
						styles_ref.set(Some(computed_style.clone()));
						let has_transition = check_for_transitions(&computed_style);
						let has_animation = check_for_animations(&computed_style);
						animation_state.set(AnimationState {
							animation_name: if has_animation { "detected" } else { "none" }.to_string(),
							has_transition: has_transition || has_animation,
							is_animating: has_transition || has_animation,
						});
					}
				}

				let anim_state = animation_state.peek().clone();

				if is_element_effectively_hidden(styles_ref.peek().as_ref()) {
					state.set(EMountState::Unmounted);
				} else if anim_state.has_transition {
					state.set(EMountState::UnmountSuspend);

					// if let Some(window) = web_sys::window() {
					// 	let closure = Closure::once_into_js(move || {
					// 		if !present() && *state.peek() == EMountState::UnmountSuspend {
					// 			state.set(EMountState::Unmounted);
					// 		}
					// 	});

					// 	window.set_timeout_with_callback_and_timeout_and_arguments_0(closure.as_ref().unchecked_ref(), 700).unwrap();
					// }
				} else {
					state.set(EMountState::Unmounted);
				}
			}
			prev_present_ref.set(present());
		}
	});

	use_effect(move || {
		if let Some(element) = node_ref().and_then(|node| node.try_as_web_event().and_then(|x| x.dyn_into::<HtmlElement>().ok())) {
			if *is_present.peek() {
				let handle_animation_start = Closure::wrap(Box::new(move |event: AnimationEvent| {
					animation_state.set(AnimationState { animation_name: event.animation_name(), has_transition: true, is_animating: true });
				}) as Box<dyn FnMut(AnimationEvent)>);

				let handle_animation_end = Closure::wrap(Box::new(move |event: AnimationEvent| {
					animation_state.set(AnimationState { animation_name: "none".to_string(), has_transition: false, is_animating: false });

					if !present() && *state.peek() == EMountState::UnmountSuspend {
						state.set(EMountState::Unmounted);
					}
				}) as Box<dyn FnMut(AnimationEvent)>);

				let handle_transition_start = Closure::wrap(Box::new(move |event: TransitionEvent| {
					animation_state.set(AnimationState { animation_name: "none".to_string(), has_transition: true, is_animating: true });
				}) as Box<dyn FnMut(TransitionEvent)>);

				let handle_transition_end = Closure::wrap(Box::new(move |event: TransitionEvent| {
					animation_state.set(AnimationState { animation_name: "none".to_string(), has_transition: false, is_animating: false });

					if !present() && *state.peek() == EMountState::UnmountSuspend {
						state.set(EMountState::Unmounted);
					}
				}) as Box<dyn FnMut(TransitionEvent)>);

				element.add_event_listener_with_callback("animationstart", handle_animation_start.as_ref().unchecked_ref()).unwrap();
				element.add_event_listener_with_callback("animationend", handle_animation_end.as_ref().unchecked_ref()).unwrap();
				element.add_event_listener_with_callback("animationcancel", handle_animation_end.as_ref().unchecked_ref()).unwrap();
				element.add_event_listener_with_callback("transitionstart", handle_transition_start.as_ref().unchecked_ref()).unwrap();
				element.add_event_listener_with_callback("transitionend", handle_transition_end.as_ref().unchecked_ref()).unwrap();
				element.add_event_listener_with_callback("transitioncancel", handle_transition_end.as_ref().unchecked_ref()).unwrap();

				handle_animation_start.forget();
				handle_animation_end.forget();
				handle_transition_start.forget();
				handle_transition_end.forget();
			} else {
				state.set(EMountState::Unmounted);
			}
		} else {
			state.set(EMountState::Unmounted);
		}
	});

	is_present
}

fn check_for_transitions(style: &CssStyleDeclaration) -> bool {
	let mut has_transition = false;

	if let Ok(transition) = style.get_property_value("transition") {
		if let Ok(duration) = style.get_property_value("transition-duration") {
			if !duration.is_empty()
				&& duration != "0s"
				&& duration != "0"
				&& !transition.is_empty()
				&& transition != "none"
				&& !transition.starts_with("0s")
				&& !transition.starts_with("none")
			{
				has_transition = true;
			}
		}
	}

	if let Ok(property) = style.get_property_value("transition-property") {
		if let Ok(duration) = style.get_property_value("transition-duration") {
			if !property.is_empty() && property != "none" && !duration.is_empty() && duration != "0s" && duration != "0" {
				has_transition = true;
			}
		}
	}

	has_transition
}

fn check_for_animations(style: &CssStyleDeclaration) -> bool {
	let mut has_animation = false;

	if let Ok(name) = style.get_property_value("animation-name") {
		if !name.is_empty() && name != "none" {
			has_animation = true;
		}
	}

	if let Ok(duration) = style.get_property_value("animation-duration") {
		if !duration.is_empty() && duration != "0s" && duration != "0" {
			has_animation = true;
		}
	}

	if let Ok(animation) = style.get_property_value("animation") {
		if !animation.is_empty() && animation != "none" && !animation.starts_with("0s") && !animation.starts_with("none") {
			has_animation = true;
		}
	}

	has_animation
}

fn is_element_effectively_hidden(styles: Option<&CssStyleDeclaration>) -> bool {
	if let Some(style) = styles {
		if let Ok(display) = style.get_property_value("display") {
			if display == "none" {
				return true;
			}
		}
		if let Ok(visibility) = style.get_property_value("visibility") {
			if visibility == "hidden" || visibility == "collapse" {
				return true;
			}
		}
		if let Ok(opacity) = style.get_property_value("opacity") {
			if opacity == "0" || opacity == "0.0" || (opacity.parse::<f64>().map_or(false, |v| v < 0.01)) {
				return true;
			}
		}
		if let Ok(transform) = style.get_property_value("transform") {
			if transform.contains("scale(0)") || transform.contains("scale(0,") || transform.contains("scale(0 ") {
				return true;
			}
		}
	}
	false
}
