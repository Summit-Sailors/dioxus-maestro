use {
	crate::{
		button::Button,
		focus_trap::FocusTrap,
		hooks::{use_escape, use_interaction_state, use_outside_click},
		select::{SelectContext, SelectOption, UseSelectProps, use_select},
	},
	dioxus::prelude::*,
	dioxus_free_icons::{
		Icon,
		icons::{
			bs_icons::{BsCheck, BsChevronDown, BsSearch},
			ld_icons::LdX,
		},
	},
	std::{
		fmt::{Debug, Display},
		rc::Rc,
	},
};

// TO DO: custom options: currently because of search component gets options on init

#[derive(Clone, PartialEq, Props)]
pub struct SelectProps<T>
where
	T: Clone + PartialEq + Display + Debug + 'static,
{
	pub options: Vec<SelectOption<T>>,
	#[props(default = Signal::new(None))]
	pub value: Signal<Option<T>>,
	pub onchange: Option<Callback<T>>,
	pub onopenchange: Option<Callback<bool>>,
	#[props(default = false)]
	pub is_searchable: bool,
	#[props(default = false)]
	pub default_open: bool,
	#[props(default = false)]
	pub disabled: bool,
	pub children: Element,
	#[props(extends = select, extends = GlobalAttributes)]
	attributes: Vec<Attribute>,
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
}

#[component]
pub fn Select<T: Clone + PartialEq + Display + Debug + 'static>(props: SelectProps<T>) -> Element {
	let SelectProps { options, default_open, disabled, onchange, onopenchange, value, is_searchable, children, attributes, .. } = props;
	let mut select_context = use_select::<T>(UseSelectProps { value, options, onchange, default_open, onopenchange, is_searchable, disabled });
	let mut interaction_state = use_interaction_state(Signal::new(false), Signal::new(props.disabled));

	use_effect(use_reactive!(|disabled| {
		if disabled != *(*select_context.peek()).disabled.peek() {
			select_context.with_mut(|ctx| ctx.disabled.set(disabled));
		}
	}));

	rsx! {
		div {
			"data-pressed": *interaction_state.is_pressed.read(),
			"data-hovered": *interaction_state.is_hovered.read(),
			"data-focused": *interaction_state.is_focused.read(),
			"data-focuse-visible": *interaction_state.is_focused.read(),
			aria_haspopup: "listbox",
			aria_expanded: select_context().open,
			aria_disabled: select_context().disabled,
			"data-disabled": select_context().disabled,
			role: "combobox",
			onmousedown: move |event| {
					interaction_state.onmousedown();
					if let Some(handler) = props.onmousedown {
							handler.call(event);
					}
			},
			onkeydown: move |event| {
					interaction_state.onkeydown();
					if let Some(handler) = props.onkeydown {
							handler.call(event);
					}
			},
			onkeyup: move |event| {
					interaction_state.onkeyup();
					if let Some(handler) = props.onkeyup {
							handler.call(event);
					}
			},
			onmouseup: move |event| {
					interaction_state.onmouseup();
					if let Some(handler) = props.onmouseup {
							handler.call(event);
					}
			},
			onmouseenter: move |event| {
					interaction_state.onmouseenter();
					if let Some(handler) = props.onmouseenter {
							handler.call(event);
					}
			},
			onmouseleave: move |event| {
					interaction_state.onmouseleave();
					if let Some(handler) = props.onmouseleave {
							handler.call(event);
					}
			},
			onfocus: move |event| {
					interaction_state.onfocus();
					if let Some(handler) = props.onfocus {
							handler.call(event);
					}
			},
			onblur: move |event| {
					interaction_state.onblur();
					if let Some(handler) = props.onblur {
							handler.call(event);
					}
			},
			..attributes,
			{children}
		}
	}
}

#[derive(Props, PartialEq, Clone)]
pub struct SelectTriggerProps {
	pub children: Element,
	#[props(default = None)]
	pub onclick: Option<EventHandler<MouseEvent>>,
	#[props(default = None)]
	pub onkeydown: Option<EventHandler<KeyboardEvent>>,
	#[props(extends = button, extends = GlobalAttributes)]
	attributes: Vec<Attribute>,
	#[props(optional, default=None)]
	icon: Option<Element>,
}

#[component]
pub fn SelectTrigger<T: Clone + PartialEq + Debug + Display + 'static>(props: SelectTriggerProps) -> Element {
	let select_context = use_context::<Signal<SelectContext<T>>>();

	let mut attributes = props.attributes.clone();
	attributes.push(Attribute::new("aria-haspopup", "listbox", None, false));
	attributes.push(Attribute::new("aria-expanded", select_context().open, None, false));
	attributes.push(Attribute::new("aria-disabled", select_context().disabled, None, false));

	let icon_down = props.icon.clone().unwrap_or_else(|| {
		rsx! {
			Icon {
				width: 16,
				height: 16,
				icon: BsChevronDown,
				style: if *select_context().open.read() { "transform: rotateX(180deg);" } else { "" },
			}
		}
	});
	rsx! {
		Button {
			disabled: select_context().disabled,
			r#type: "button",
			onclick: move |event| {
					let open = *select_context().open.peek();
					props.onclick.unwrap_or_default().call(event);
					select_context().toggle(!open);
			},
			onkeydown: move |event: Event<KeyboardData>| {
					if event.code() == Code::Space || event.code() == Code::Enter {
							let open = *select_context().open.peek();
							props.onkeydown.unwrap_or_default().call(event);
							select_context().toggle(!open);
					}
			},
			additional_attributes: attributes.clone(),
			{props.children}
			{icon_down}
		}
	}
}

#[derive(Props, PartialEq, Clone)]
pub struct SelectValueProps {
	#[props(extends = span, extends = GlobalAttributes)]
	attributes: Vec<Attribute>,
	#[props(default = String::default())]
	pub placeholder: String,
}

#[component]
pub fn SelectValue<T: Clone + PartialEq + Display + Debug + 'static>(props: SelectValueProps) -> Element {
	let select_context = use_context::<Signal<SelectContext<T>>>();
	let value = select_context().value.read().clone();

	rsx! {
		span {
			"data-state": if value.is_some() { "selected" } else { "placeholder" },
			..props.attributes.clone(),
			if value.is_some() {
				"{&select_context().init_options.iter().find(|option| option.value == value.clone().unwrap()).unwrap().label}"
			} else {
				"{props.placeholder}"
			}
		}
	}
}

#[derive(Props, PartialEq, Clone)]
pub struct OptionProps<T>
where
	T: Clone + PartialEq + Display + Debug + 'static,
{
	pub option: SelectOption<T>,
	#[props(optional, default=None)]
	pub select_indicator: Option<Element>,
	#[props(optional, default=String::default())]
	pub class: String,
}

#[component]
fn Option<T: Clone + PartialEq + Display + Debug + 'static>(props: OptionProps<T>) -> Element {
	let select_context = use_context::<Signal<SelectContext<T>>>();
	let SelectOption { value, label, disabled } = props.option;
	let is_selected = select_context().value.read().clone().is_some_and(|x| x == value.clone());
	let is_focused = select_context().focused_index.read().clone().map_or(false, |i| i == value.clone());
	let cloned_value = value.clone();

	let select_indicator = props.select_indicator.clone().unwrap_or_else(|| {
		rsx! {
			Icon { width: 16, height: 16, icon: BsCheck }
		}
	});

	let handle_change = move |_| {
		select_context().on_change(cloned_value.clone());
		if let Some(onchange) = &select_context().onchange {
			onchange.call(cloned_value.clone());
		}
		if let Some(onopenchange) = &select_context().onopenchange {
			onopenchange.call(false);
		}
		select_context().toggle(false);
	};

	rsx! {
		li {
			"data-selected": is_selected,
			"data-focused": is_focused,
			aria_selected: is_selected,
			"data-role": "option",
			aria_disabled: disabled,
			"data-disabled": disabled,
			role: "option",
			tabindex: if !disabled { "0" } else { "-1" },
			style: format!("{}", if disabled { "pointer-events:none;" } else { "cursor:pointer;" }),
			class: props.class.clone(),
			onclick: handle_change,
			onkeydown: move |event| {
					if event.key() == Key::Enter || event.code() == Code::Space {
							select_context().on_change(value.clone());
							if let Some(onchange) = &select_context().onchange {
									onchange.call(value.clone());
							}
							if let Some(onopenchange) = &select_context().onopenchange {
									onopenchange.call(false);
							}
							select_context().toggle(false);
					}
			},
			"{label}"
			if is_selected {
				{select_indicator}
			}
		}
	}
}

#[derive(Props, PartialEq, Clone)]
pub struct SelectDropdownProps {
	#[props(extends = span, extends = GlobalAttributes)]
	attributes: Vec<Attribute>,
	#[props(optional, default=None)]
	search_icon: Option<Element>,
	#[props(optional, default = true)]
	use_search_icon: bool,
	#[props(optional, default=None)]
	select_indicator: Option<Element>,
	#[props(optional, default=String::default())]
	options_list_class: String,
	#[props(optional, default=String::default())]
	option_class: String,
}

#[component]
pub fn SelectDropdown<T: Clone + PartialEq + Display + Debug + 'static>(props: SelectDropdownProps) -> Element {
	let select_context = use_context::<Signal<SelectContext<T>>>();
	let mut current_ref = use_signal(|| None::<Rc<MountedData>>);
	let handle_close = use_callback(move |()| {
		if *select_context.peek().open.peek() {
			select_context().toggle(false);
		}
	});

	use_outside_click(current_ref, handle_close, select_context().open);
	use_escape(handle_close, select_context().open);

	let search_icon = props.search_icon.clone().unwrap_or_else(|| {
		rsx! {
			span { "data-role": "search-icon",
				Icon { width: 16, height: 16, icon: BsSearch }
			}
		}
	});

	if *select_context().open.read() {
		rsx! {
			FocusTrap {
				div {
					"data-open": select_context().open,
					role: "listbox",
					onclick: move |event| event.stop_propagation(),
					onmounted: move |event| current_ref.set(Some(event.data())),
					..props.attributes,
					ul {
						"data-role": "listbox",
						class: props.options_list_class.clone(),
						if select_context().is_searchable {
							li {
								"data-role": "search-container",
								class: props.option_class.clone(),
								if props.use_search_icon {
									{search_icon}
								}
								input {
									r#type: "text",
									"data-role": "search",
									value: select_context().search_input.clone(),
									oninput: move |event: Event<FormData>| select_context().on_search_change(event.value()),
								}
								Button {
									r#type: "button",
									role: "button",
									"data-role": "clear",
									aria_hidden: (*select_context().search_input.read()).is_empty(),
									aria_label: "clear search input",
									onclick: move |event: Event<MouseData>| {
											event.stop_propagation();
											select_context().on_search_change(String::new());
									},
									Icon { width: 16, icon: LdX }
								}
							}
							for option in select_context().options.iter() {
								Option {
									key: option.clone().value,
									option: option.clone(),
									select_indicator: props.select_indicator.clone(),
									class: &props.option_class,
								}
							}
						}
					}
				}
			}
		}
	} else {
		rsx! {}
	}
}
