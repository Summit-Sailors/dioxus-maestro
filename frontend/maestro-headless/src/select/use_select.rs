use {
	dioxus::prelude::*,
	std::{
		cmp::Ordering,
		fmt::{Debug, Display},
	},
};

#[derive(Clone, PartialEq, Debug)]
pub struct SelectContext<T>
where
	T: Clone + PartialEq + Display + Debug + 'static,
{
	pub options: Signal<Vec<SelectOption<T>>>,
	pub init_options: Vec<SelectOption<T>>,
	pub value: Signal<Option<T>>,
	pub onchange: Option<Callback<T>>,
	pub open: Signal<bool>,
	pub disabled: Signal<bool>,
	pub onopenchange: Option<Callback<bool>>,
	pub search_input: Signal<String>,
	pub is_searchable: bool,
	pub focused_index: Signal<Option<T>>,
}

impl<T> SelectContext<T>
where
	T: Clone + PartialEq + Display + Debug + 'static,
{
	pub fn new(
		value: Signal<Option<T>>,
		options: Vec<SelectOption<T>>,
		onchange: Option<Callback<T>>,
		open: bool,
		onopenchange: Option<Callback<bool>>,
		is_searchable: bool,
		disabled: bool,
	) -> Self {
		Self {
			value,
			options: Signal::new(options.clone()),
			onchange,
			open: Signal::new(open),
			onopenchange,
			is_searchable,
			disabled: Signal::new(disabled),
			search_input: Signal::new(String::default()),
			focused_index: Signal::new(None),
			init_options: options.clone(),
		}
	}

	pub fn handle_options_change(&mut self) {
		let search = self.search_input.peek().clone();
		let bind_value = self.value.peek();
		let current_value = bind_value.as_ref();
		let options = self.init_options.clone();
		let mut filtered = options.into_iter().filter(|option| option.label.to_lowercase().contains(&search.to_lowercase())).collect::<Vec<SelectOption<T>>>();
		filtered.sort_by(|a, b| {
			if let Some(current_value) = &current_value {
				if &a.value == *current_value {
					Ordering::Less
				} else if &b.value == *current_value {
					Ordering::Greater
				} else {
					Ordering::Equal
				}
			} else {
				Ordering::Equal
			}
		});
		self.options.set(filtered);
	}

	pub fn on_change(&mut self, value: T) {
		self.value.set(Some(value.clone()));
		self.search_input.set(String::new());
		self.handle_options_change();
	}

	pub fn on_search_change(&mut self, value: String) {
		self.search_input.set(value);
		self.handle_options_change();
	}

	pub fn toggle(&mut self, value: bool) {
		if !value {
			self.search_input.set(String::new());
			self.handle_options_change();
		}
		self.open.set(value);
		if let Some(onopenchange) = self.onopenchange {
			onopenchange.call(value);
		}
	}
}

#[derive(Clone, PartialEq, Debug)]
pub struct SelectOption<T> {
	pub label: String,
	pub value: T,
	pub disabled: bool,
}

#[derive(Clone, PartialEq)]
pub struct UseSelectProps<T>
where
	T: Clone + PartialEq + Display + Debug + 'static,
{
	pub options: Vec<SelectOption<T>>,
	pub value: Signal<Option<T>>,
	pub onchange: Option<Callback<T>>,
	pub onopenchange: Option<Callback<bool>>,
	pub is_searchable: bool,
	pub default_open: bool,
	pub disabled: bool,
}

pub fn use_select<T: Clone + PartialEq + Display + Debug + 'static>(props: UseSelectProps<T>) -> Signal<SelectContext<T>> {
	let UseSelectProps { options, default_open, disabled, onchange, onopenchange, value, is_searchable } = props;
	let context = use_signal(|| SelectContext::new(value, options, onchange, default_open, onopenchange, is_searchable, disabled));

	let mut select_context = use_context_provider(|| context);

	use_effect(use_reactive!(|disabled| {
		if disabled != *select_context().disabled.peek() {
			select_context.with_mut(|ctx| ctx.disabled.set(disabled));
		}
	}));
	select_context
}
