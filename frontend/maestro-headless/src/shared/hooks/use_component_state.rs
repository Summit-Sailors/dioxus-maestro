use {
	dioxus::prelude::*,
	serde::{Deserialize, Serialize},
	std::fmt::Debug,
};

pub struct UseControllableStateParams<T: Clone + PartialEq + Debug + Default + Serialize + for<'a> Deserialize<'a> + 'static> {
	pub is_controlled: bool,
	pub prop: ReadOnlySignal<Option<T>>,
	pub default_prop: T,
	pub on_change: Option<Callback<T>>,
}

pub fn use_controllable_state<T: Clone + PartialEq + Debug + Default + Serialize + for<'a> Deserialize<'a> + 'static>(
	UseControllableStateParams { is_controlled, prop, default_prop, on_change }: UseControllableStateParams<T>,
) -> (Memo<T>, Callback<T>) {
	let mut uncontrolled_state = use_uncontrolled_state(UseUncontrollableStateParams { default_prop, on_change });
	let value = use_memo(move || match is_controlled {
		true => prop().unwrap_or_default(),
		false => uncontrolled_state(),
	});

	let set_value = use_callback(move |next_value: T| {
		if is_controlled {
			if next_value != prop().unwrap_or_default() {
				if let Some(on_change) = on_change {
					on_change.call(next_value);
				}
			}
		} else {
			uncontrolled_state.set(next_value);
		}
	});

	(value, set_value)
}

pub struct UseUncontrollableStateParams<T: Clone + PartialEq + Debug + Default + Serialize + for<'a> Deserialize<'a> + 'static> {
	pub default_prop: T,
	pub on_change: Option<Callback<T>>,
}

pub fn use_uncontrolled_state<T: Clone + PartialEq + Debug + Default + Serialize + for<'a> Deserialize<'a> + 'static>(
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
