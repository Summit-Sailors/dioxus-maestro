use {
	crate::{
		button::Button,
		hooks::{UseControllableStateParams, use_controllable_state, use_escape, use_outside_click},
		popper::{Popper, PopperAnchor, PopperContent},
		presence::Presence,
		utils::{EAlign, ESide},
	},
	dioxus::prelude::*,
	dioxus_free_icons::{
		Icon,
		icons::bs_icons::{BsCheck, BsChevronDown},
	},
	std::{fmt::Debug, rc::Rc},
	uuid::Uuid,
};

#[derive(Clone, PartialEq, Debug)]
pub struct SelectOptionContext {
	pub selected: ReadOnlySignal<bool>,
	pub disabled: ReadOnlySignal<bool>,
}

#[derive(Clone, PartialEq, Debug, Copy)]
pub struct SelectContext {
	pub value: Memo<Option<Vec<String>>>,
	pub set_value: Callback<Option<Vec<String>>>,
	pub open: Memo<Option<bool>>,
	pub set_open: Callback<Option<bool>>,
	pub disabled: ReadOnlySignal<bool>,
	pub required: bool,
	pub multi: bool,
	pub trigger_id: Uuid,
	pub container_id: Uuid,
}

#[derive(Clone, PartialEq, Props)]
pub struct SelectProps {
	#[props(optional, default = ReadOnlySignal::new(Signal::new(None)))]
	pub value: ReadOnlySignal<Option<Vec<String>>>,
	#[props(optional, default = Vec::new())]
	pub default_value: Vec<String>,
	#[props(optional)]
	pub on_value_change: Option<Callback<Option<Vec<String>>>>,

	#[props(optional, default = ReadOnlySignal::new(Signal::new(None)))]
	pub open: ReadOnlySignal<Option<bool>>,
	#[props(optional, default = false)]
	pub default_open: bool,
	#[props(optional)]
	pub on_open_change: Option<Callback<Option<bool>>>,

	#[props(default = ReadOnlySignal::new(Signal::new(false)))]
	pub disabled: ReadOnlySignal<bool>,
	#[props(default = false)]
	pub required: bool,
	#[props(default = false)]
	pub multi: bool,

	#[props(extends = select, extends = GlobalAttributes)]
	attributes: Vec<Attribute>,
	pub children: Element,
}

#[component]
pub fn Select(props: SelectProps) -> Element {
	let SelectProps { open, default_open, on_open_change, disabled, value, default_value, on_value_change, required, multi, .. } = props;
	let is_controlled = use_hook(move || value().is_some());
	let (value, set_value) =
		use_controllable_state(UseControllableStateParams { is_controlled, prop: value, default_prop: default_value, on_change: on_value_change });

	let is_open_controlled = use_hook(move || open().is_some());
	let (open, set_open) =
		use_controllable_state(UseControllableStateParams { is_controlled: is_open_controlled, prop: open, default_prop: default_open, on_change: on_open_change });

	use_context_provider::<SelectContext>(|| SelectContext {
		value,
		set_value,
		open,
		set_open,
		disabled,
		required,
		multi,
		trigger_id: Uuid::new_v4(),
		container_id: Uuid::new_v4(),
	});

	let handle_close = use_callback(move |()| {
		set_open.call(Some(false));
	});

	let mut current_ref = use_signal(|| None::<Rc<MountedData>>);
	use_outside_click(current_ref, handle_close, open);

	rsx! {
		Popper {
			is_arrow_hidden: true,
			aria_haspopup: "listbox",
			aria_expanded: open().unwrap_or_default(),
			aria_disabled: disabled(),
			aria_required: required,
			"data-disabled": disabled(),
			"data-required": required,
			role: "combobox",
			onmounted: move |event: Event<MountedData>| current_ref.set(Some(event.data())),
			extra_attributes: props.attributes.clone(),
			{props.children}
		}
	}
}

#[derive(Props, PartialEq, Clone)]
pub struct SelectTriggerProps {
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
	#[props(default = None)]
	pub onclick: Option<EventHandler<Event<MouseData>>>,

	#[props(extends = button, extends = GlobalAttributes)]
	attributes: Vec<Attribute>,
	#[props(default = Vec::new())]
	pub container_attributes: Vec<Attribute>,
	pub children: Element,
}

#[component]
pub fn SelectTrigger(props: SelectTriggerProps) -> Element {
	let context = use_context::<SelectContext>();

	let mut attributes = props.attributes.clone();
	attributes.push(Attribute::new("aria-haspopup", "listbox", None, false));
	attributes.push(Attribute::new("aria-expanded", context.open.read().unwrap_or_default(), None, false));
	attributes.push(Attribute::new("aria-disabled", *context.disabled.read(), None, false));

	rsx! {
		PopperAnchor { extra_attributes: props.container_attributes.clone(),
			Button {
				id: context.trigger_id.to_string(),
				disabled: *context.disabled.read(),
				aria_controls: context.container_id.to_string(),
				aria_haspopup: "listbox",
				aria_expanded: context.open.read().unwrap_or_default(),
				"data-state": if context.open.read().unwrap_or_default() { "open" } else { "closed" },
				r#type: "button",
				pointer_events: if *context.disabled.read() { "none" } else { "auto" },
				cursor: if *context.disabled.read() { "" } else { "pointer" },
				tabindex: if *context.disabled.read() { "-1" } else { "0" },
				onclick: move |event| {
						let open = context.open.peek().unwrap_or_default();
						props.onclick.unwrap_or_default().call(event);
						context.set_open.call(Some(!open));
				},
				onmousedown: props.onmousedown,
				onkeydown: props.onkeydown,
				onkeyup: props.onkeyup,
				onmouseup: props.onmouseup,
				onmouseenter: props.onmouseenter,
				onmouseleave: props.onmouseleave,
				onfocus: props.onfocus,
				onblur: props.onblur,
				extra_attributes: attributes.clone(),
				{props.children}
			}
		}
	}
}

#[derive(Props, PartialEq, Clone)]
pub struct SelectIconProps {
	#[props(extends = span, extends = GlobalAttributes)]
	attributes: Vec<Attribute>,
	#[props(default = None)]
	pub children: Option<Element>,
}

#[component]
pub fn SelectIcon(props: SelectIconProps) -> Element {
	let context = use_context::<SelectContext>();

	let icon_down = if let Some(children) = props.children {
		rsx! {
			{children}
		}
	} else {
		rsx! {
			Icon { width: 16, height: 16, icon: BsChevronDown }
		}
	};

	rsx! {
		span {
			transform: if context.open.read().unwrap_or_default() { "rotateX(180deg)" } else { "rotateX(0)" },
			..props.attributes,
			{icon_down}
		}
	}
}

#[derive(Props, PartialEq, Clone)]
pub struct SelectValueProps {
	#[props(default = String::default())]
	pub placeholder: String,

	#[props(extends = span, extends = GlobalAttributes)]
	attributes: Vec<Attribute>,
	#[props(default = None)]
	children: Option<Element>,
}

#[component]
pub fn SelectValue(props: SelectValueProps) -> Element {
	let context = use_context::<SelectContext>();
	let value = context.value.read().clone().unwrap_or_default();
	let display_value = if let Some(children) = props.children {
		rsx! {
			{children}
		}
	} else {
		let v = if context.multi { value.join(", ") } else { (value.first().unwrap_or(&String::new())).to_string() };
		rsx! { "{v}" }
	};

	rsx! {
		span {
			"data-state": if !value.is_empty() { "value" } else { "placeholder" },
			..props.attributes.clone(),
			if value.is_empty() {
				"{props.placeholder}"
			} else {
				{display_value}
			}
		}
	}
}

#[derive(Props, PartialEq, Clone)]
pub struct SelectOptionProps {
	pub value: String,
	#[props(optional, default = ReadOnlySignal::new(Signal::new(false)))]
	pub selected: ReadOnlySignal<bool>,
	#[props(optional, default = ReadOnlySignal::new(Signal::new(false)))]
	pub disabled: ReadOnlySignal<bool>,

	#[props(extends = GlobalAttributes, extends = div)]
	attributes: Vec<Attribute>,
	#[props(default = None)]
	children: Element,
}

#[component]
pub fn SelectOption(props: SelectOptionProps) -> Element {
	let context = use_context::<SelectContext>();
	let SelectOptionProps { value, selected, disabled, attributes, children } = props;
	use_context_provider::<SelectOptionContext>(|| SelectOptionContext { selected, disabled });

	let handle_change = use_callback(move |_| {
		let mut current_value = context.value.peek().clone().unwrap_or_default();
		if context.multi {
			if current_value.contains(&value) {
				current_value.retain(|val| val != &value);
			} else {
				current_value.push(value.clone());
			}
		} else {
			let is_present = current_value.contains(&value);
			current_value.clear();
			if !is_present {
				current_value.push(value.clone());
			}
		}
		context.set_value.call(Some(current_value));
		if !context.multi {
			context.set_open.call(Some(false));
		}
	});

	rsx! {
		div {
			"data-selected": selected(),
			aria_selected: selected(),
			"data-role": "option",
			aria_disabled: disabled(),
			"data-disabled": disabled(),
			role: "option",
			tabindex: if !disabled() { "0" } else { "-1" },
			pointer_events: if disabled() { "none" } else { "all" },
			cursor: if disabled() { "auto" } else { "pointer" },
			onclick: move |_| handle_change(()),
			onkeydown: move |event| {
					if event.key() == Key::Enter || event.code() == Code::Space {
							event.stop_propagation();
							handle_change(());
					}
			},
			..attributes,
			{children}
		}
	}
}

#[derive(Props, PartialEq, Clone)]
pub struct OptionSelectedIndicator {
	#[props(extends = span, extends = GlobalAttributes)]
	attributes: Vec<Attribute>,
	#[props(default = None)]
	pub children: Option<Element>,
}

#[component]
pub fn OptionSelectedIndicator(props: OptionSelectedIndicator) -> Element {
	let context = use_context::<SelectOptionContext>();

	let indicator = if let Some(children) = props.children {
		rsx! {
			{children}
		}
	} else {
		rsx! {
			Icon { width: 16, height: 16, icon: BsCheck }
		}
	};

	rsx! {
		span {
			width: "fit",
			height: "fit",
			hidden: !*context.selected.read(),
			aria_hidden: !*context.selected.read(),
			"data-hidden": !*context.selected.read(),
			"data-role": "indicator",
			..props.attributes,
			{indicator}
		}
	}
}

#[derive(Props, PartialEq, Clone)]
pub struct SelectDropdownProps {
	#[props(default = ESide::Bottom)]
	side: ESide,
	#[props(default = 0.0)]
	side_offset: f32,
	#[props(default = EAlign::Center)]
	align: EAlign,
	#[props(default = 0.0)]
	align_offset: f32,
	#[props(default = true)]
	avoid_collisions: bool,
	#[props(default = 4.0)]
	collision_padding: f32,

	#[props(extends = div, extends = GlobalAttributes)]
	attributes: Vec<Attribute>,
	children: Element,
}

#[component]
pub fn SelectDropdown(props: SelectDropdownProps) -> Element {
	let context = use_context::<SelectContext>();

	let handle_close = use_callback(move |()| {
		context.set_open.call(Some(false));
	});

	let mut current_ref = use_signal(|| None::<Rc<MountedData>>);

	use_escape(handle_close, context.open);

	rsx! {
		Presence {
			node_ref: current_ref,
			present: context.open.read().unwrap_or_default(),
			PopperContent {
				role: "listbox",
				id: context.container_id.to_string(),
				side: props.side,
				side_offset: props.side_offset,
				align: props.align,
				align_offset: props.align_offset,
				avoid_collisions: props.avoid_collisions,
				collision_padding: props.collision_padding,
				aria_labelledby: context.trigger_id.to_string(),
				aria_hidden: !context.open.read().unwrap_or_default(),
				"data-state": if context.open.read().unwrap_or_default() { "open" } else { "closed" },
				onmounted: move |event: Event<MountedData>| current_ref.set(Some(event.data())),
				extra_attributes: props.attributes,
				{props.children}
			}
		}
	}
}
