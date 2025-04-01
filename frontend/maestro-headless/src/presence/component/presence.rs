use {
	crate::presence::component::utils::{check_for_animations, check_for_transitions, is_element_effectively_hidden},
	dioxus::{prelude::*, web::WebEventExt},
	std::rc::Rc,
	web_sys::{
		wasm_bindgen::{prelude::Closure, JsCast},
		AnimationEvent, CssStyleDeclaration, HtmlElement, TransitionEvent,
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
				} else {
					state.set(EMountState::Unmounted);
				}
			}
			prev_present_ref.set(present());
		}
	});

	use_effect(move || {
		if let Some(element) = node_ref().and_then(|node| node.try_as_web_event().and_then(|x| x.dyn_into::<HtmlElement>().ok())) {
			if is_present() {
				let handle_animation_start = Closure::wrap(Box::new(move |event: AnimationEvent| {
					animation_state.set(AnimationState { animation_name: event.animation_name(), has_transition: true, is_animating: true });
				}) as Box<dyn FnMut(AnimationEvent)>);

				let handle_animation_end = Closure::wrap(Box::new(move |_event: AnimationEvent| {
					animation_state.set(AnimationState { animation_name: "none".to_string(), has_transition: false, is_animating: false });

					if !present() && *state.peek() == EMountState::UnmountSuspend {
						state.set(EMountState::Unmounted);
					}
				}) as Box<dyn FnMut(AnimationEvent)>);

				let handle_transition_start = Closure::wrap(Box::new(move |_event: TransitionEvent| {
					animation_state.set(AnimationState { animation_name: "none".to_string(), has_transition: true, is_animating: true });
				}) as Box<dyn FnMut(TransitionEvent)>);

				let handle_transition_end = Closure::wrap(Box::new(move |_event: TransitionEvent| {
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
