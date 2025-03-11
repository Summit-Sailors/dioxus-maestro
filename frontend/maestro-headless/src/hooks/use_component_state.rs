use {dioxus::prelude::*, std::fmt::Debug};

pub struct UseControllableStateParams<T: Clone + PartialEq + Debug + 'static> {
	pub is_controlled: bool,
	pub prop: T,
	pub default_prop: T,
	pub on_change: Option<Callback<T>>,
}

pub fn use_controllable_state<T: Clone + PartialEq + Debug + 'static>(
	UseControllableStateParams { is_controlled, prop, default_prop, on_change }: UseControllableStateParams<T>,
) -> (Signal<T>, Callback<T>) {
	let mut uncontrolled_state = use_uncontrolled_state(UseUncontrollableStateParams { default_prop, on_change });
	let mut controlled_state = use_signal(|| prop.clone());

	use_effect(use_reactive!(|is_controlled, prop| {
		if is_controlled && prop != *controlled_state.peek() {
			controlled_state.set(prop.clone());
		}
	}));

	let set_value = use_callback(move |next_value: T| {
		if is_controlled {
			if next_value != prop {
				if let Some(on_change) = on_change {
					on_change.call(next_value.clone());
				}
				controlled_state.set(next_value.clone());
			}
		} else {
			uncontrolled_state.set(next_value.clone());
		}
	});

	(if is_controlled { controlled_state } else { uncontrolled_state }, set_value)
}

pub struct UseUncontrollableStateParams<T: Clone + PartialEq + Debug + 'static> {
	pub default_prop: T,
	pub on_change: Option<Callback<T>>,
}

pub fn use_uncontrolled_state<T: Clone + PartialEq + Debug + 'static>(
	UseUncontrollableStateParams { default_prop, on_change }: UseUncontrollableStateParams<T>,
) -> Signal<T> {
	let uncontrolled_state = use_signal(|| default_prop);
	let mut prev_value = use_signal(|| uncontrolled_state.peek().clone());

	use_effect(move || {
		let value = uncontrolled_state();
		if *prev_value.peek() != value {
			if let Some(on_change) = on_change {
				on_change.call(value.clone());
			}
			prev_value.set(value);
		}
	});

	uncontrolled_state
}
