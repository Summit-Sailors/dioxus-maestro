use {
	crate::{
		button::Button,
		popper::{Popper, PopperAnchor, PopperContent},
		presence::Presence,
		shared::{EAlign, ESide, UseControllableStateParams, use_controllable_state, use_escape, use_outside_click, use_ref_provider},
	},
	dioxus::prelude::*,
	std::{fmt::Debug, rc::Rc},
	uuid::Uuid,
};

#[derive(Clone, Debug, PartialEq)]
pub struct SelectOptionContext {
	pub selected: Memo<bool>,
	pub disabled: ReadOnlySignal<bool>,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct SelectContext {
	pub value: Memo<Vec<String>>,
	pub set_value: Callback<Vec<String>>,
	pub open: Memo<bool>,
	pub set_open: Callback<bool>,
	pub disabled: ReadOnlySignal<bool>,
	pub required: ReadOnlySignal<bool>,
	pub multi: ReadOnlySignal<bool>,
	pub trigger_id: Uuid,
	pub container_id: Uuid,
}

impl SelectContext {
	pub fn new(
		value: Memo<Vec<String>>,
		set_value: Callback<Vec<String>>,
		open: Memo<bool>,
		set_open: Callback<bool>,
		disabled: ReadOnlySignal<bool>,
		required: ReadOnlySignal<bool>,
		multi: ReadOnlySignal<bool>,
	) -> Self {
		Self { value, set_value, open, set_open, disabled, required, multi, trigger_id: Uuid::new_v4(), container_id: Uuid::new_v4() }
	}
}

#[derive(Clone, PartialEq, Props)]
pub struct SelectRootProps {
	#[props(optional, default = ReadOnlySignal::new(Signal::new(None)))]
	pub value: ReadOnlySignal<Option<Vec<String>>>,
	#[props(optional, default = Vec::new())]
	pub default_value: Vec<String>,
	#[props(optional)]
	pub on_value_change: Option<Callback<Vec<String>>>,

	#[props(optional, default = ReadOnlySignal::new(Signal::new(None)))]
	pub open: ReadOnlySignal<Option<bool>>,
	#[props(optional, default = false)]
	pub default_open: bool,
	#[props(optional)]
	pub on_open_change: Option<Callback<bool>>,

	#[props(default = ReadOnlySignal::new(Signal::new(false)))]
	pub disabled: ReadOnlySignal<bool>,
	#[props(default = ReadOnlySignal::new(Signal::new(false)))]
	pub required: ReadOnlySignal<bool>,
	#[props(default = ReadOnlySignal::new(Signal::new(false)))]
	pub multi: ReadOnlySignal<bool>,

	#[props(extends = select, extends = GlobalAttributes)]
	attributes: Vec<Attribute>,
	#[props(default = Vec::new())]
	pub extra_attributes: Vec<Attribute>,
	pub children: Element,
}

#[component]
pub fn SelectRoot(props: SelectRootProps) -> Element {
	let SelectRootProps {
		open,
		default_open,
		on_open_change,
		disabled,
		value,
		default_value,
		on_value_change,
		required,
		multi,
		attributes,
		extra_attributes,
		children,
	} = props;
	let is_controlled = use_hook(move || value().is_some());
	let (value, set_value) =
		use_controllable_state(UseControllableStateParams { is_controlled, prop: value, default_prop: default_value, on_change: on_value_change });

	let is_open_controlled = use_hook(move || open().is_some());
	let (open, set_open) =
		use_controllable_state(UseControllableStateParams { is_controlled: is_open_controlled, prop: open, default_prop: default_open, on_change: on_open_change });

	use_context_provider::<SelectContext>(|| SelectContext::new(value, set_value, open, set_open, disabled, required, multi));

	let handle_close = use_callback(move |()| {
		set_open.call(false);
	});

	let mut current_ref = use_signal(|| None::<Rc<MountedData>>);
	use_outside_click(current_ref, handle_close, open);

	let mut attrs = attributes.clone();
	attrs.extend(extra_attributes);

	rsx! {
    Popper {
      aria_haspopup: "listbox",
      aria_expanded: open(),
      aria_disabled: disabled(),
      aria_required: required(),
      "data-disabled": disabled(),
      "data-required": required(),
      "data-role": "select",
      role: "combobox",
      onmounted: move |event: Event<MountedData>| current_ref.set(Some(event.data())),
      extra_attributes: attrs.clone(),
      {children}
    }
  }
}

#[derive(Clone, PartialEq, Props)]
pub struct SelectTriggerProps {
	#[props(default = None)]
	pub onclick: Option<EventHandler<Event<MouseData>>>,

	#[props(extends = button, extends = GlobalAttributes)]
	attributes: Vec<Attribute>,
	#[props(default = Vec::new())]
	pub extra_attributes: Vec<Attribute>,
	pub children: Element,
}

#[component]
pub fn SelectTrigger(props: SelectTriggerProps) -> Element {
	let SelectTriggerProps { onclick, attributes, extra_attributes, children } = props;

	let context = use_context::<SelectContext>();

	let mut attrs = attributes.clone();
	attrs.extend(extra_attributes);

	rsx! {
    PopperAnchor {
      Button {
        id: context.trigger_id.to_string(),
        disabled: *context.disabled.read(),
        aria_controls: context.container_id.to_string(),
        aria_haspopup: "listbox",
        aria_expanded: *context.open.read(),
        "data-state": if *context.open.read() { "open" } else { "closed" },
        r#type: "button",
        pointer_events: if *context.disabled.read() { "none" } else { "auto" },
        cursor: if *context.disabled.read() { "" } else { "pointer" },
        tabindex: if *context.disabled.read() { "-1" } else { "0" },
        onclick: move |event| {
            let open = *context.open.peek();
            onclick.unwrap_or_default().call(event);
            context.set_open.call(!open);
        },
        extra_attributes: attrs.clone(),
        {children}
      }
    }
  }
}

#[derive(Clone, PartialEq, Props)]
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
      svg {
        stroke: "currentColor",
        fill: "currentColor",
        stroke_width: "0",
        view_box: "0 0 512 512",
        height: "16px",
        width: "16px",
        xmlns: "http://www.w3.org/2000/svg",
        path {
          fill: "none",
          stroke_linecap: "round",
          stroke_linejoin: "round",
          stroke_width: "48",
          d: "m112 184 144 144 144-144",
        }
      }
    }
	};

	rsx! {
    span {
      transform: if *context.open.read() { "rotateX(180deg)" } else { "rotateX(0)" },
      ..props.attributes,
      {icon_down}
    }
  }
}

#[derive(Clone, PartialEq, Props)]
pub struct SelectValueProps {
	#[props(default = String::default())]
	pub placeholder: String,

	#[props(extends = span, extends = GlobalAttributes)]
	attributes: Vec<Attribute>,
	#[props(default = Vec::new())]
	pub extra_attributes: Vec<Attribute>,
	#[props(default = None)]
	children: Option<Element>,
}

#[component]
pub fn SelectValue(props: SelectValueProps) -> Element {
	let SelectValueProps { placeholder, attributes, extra_attributes, children } = props;

	let context = use_context::<SelectContext>();
	let value = context.value.read().clone();
	let display_value = if let Some(children) = children {
		rsx! {
      {children}
    }
	} else {
		let v = if *context.multi.read() { value.join(", ") } else { (value.first().unwrap_or(&String::new())).to_string() };
		rsx! { "{v}" }
	};

	rsx! {
    span {
      "data-state": if !value.is_empty() { "value" } else { "placeholder" },
      ..attributes.clone(),
      ..extra_attributes.clone(),
      if value.is_empty() {
        "{placeholder}"
      } else {
        {display_value}
      }
    }
  }
}

#[derive(Clone, PartialEq, Props)]
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
	#[props(default = Vec::new())]
	pub extra_attributes: Vec<Attribute>,
	children: Element,
}

#[component]
pub fn SelectDropdown(props: SelectDropdownProps) -> Element {
	let SelectDropdownProps { side, side_offset, align, align_offset, avoid_collisions, collision_padding, attributes, extra_attributes, children } = props;

	let context = use_context::<SelectContext>();

	let handle_close = use_callback(move |()| {
		context.set_open.call(false);
	});

	use_ref_provider();

	use_escape(handle_close, context.open);

	let mut attrs = attributes.clone();
	attrs.extend(extra_attributes.clone());

	let mut styled_attrs: Vec<Attribute> = Vec::new();
	styled_attrs.push(Attribute::new("--maestro-headless-select-anchor-height", "var(--maestro-headless-popper-anchor-height)", Some("style"), false));
	styled_attrs.push(Attribute::new("--maestro-headless-select-anchor-width", "var(--maestro-headless-popper-anchor-width)", Some("style"), false));
	styled_attrs.push(Attribute::new("--maestro-headless-select-content-height", "var(--maestro-headless-popper-content-height)", Some("style"), false));
	styled_attrs.push(Attribute::new("--maestro-headless-select-content-width", "var(--maestro-headless-popper-content-width)", Some("style"), false));
	styled_attrs.push(Attribute::new(
		"--maestro-headless-select-content-transform-origin",
		"var(--maestro-headless-popper-content-transform-origin)",
		Some("style"),
		false,
	));

	rsx! {
    Presence { present: *context.open.read(),
      PopperContent {
        role: "listbox",
        id: context.container_id.to_string(),
        side,
        side_offset,
        align,
        align_offset,
        avoid_collisions,
        collision_padding,
        aria_labelledby: context.trigger_id.to_string(),
        aria_hidden: !*context.open.read(),
        "data-state": if *context.open.read() { "open" } else { "closed" },
        extra_attributes: attrs.clone(),
        styled_attributes: styled_attrs.clone(),
        {children}
      }
    }
  }
}

#[derive(Clone, PartialEq, Props)]
pub struct SelectOptionProps {
	pub value: String,
	#[props(optional, default = ReadOnlySignal::new(Signal::new(false)))]
	pub disabled: ReadOnlySignal<bool>,

	#[props(extends = GlobalAttributes, extends = div)]
	attributes: Vec<Attribute>,
	#[props(default = Vec::new())]
	pub extra_attributes: Vec<Attribute>,
	#[props(default = None)]
	children: Element,
}

#[component]
pub fn SelectOption(props: SelectOptionProps) -> Element {
	let SelectOptionProps { value, disabled, attributes, extra_attributes, children } = props;

	let context = use_context::<SelectContext>();
	let current_value = value.clone();
	let selected = use_memo(move || context.value.read().contains(&current_value.clone()));

	use_context_provider::<SelectOptionContext>(|| SelectOptionContext { selected, disabled });

	let handle_change = use_callback(move |_| {
		let mut current_value = context.value.peek().clone();
		let is_multi = *context.multi.read();
		if is_multi {
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
		context.set_value.call(current_value);
		if !is_multi {
			context.set_open.call(false);
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
      onclick: move |event| {
          event.stop_propagation();
          event.prevent_default();
          handle_change(());
      },
      onkeydown: move |event| {
          if event.key() == Key::Enter || event.code() == Code::Space {
              event.stop_propagation();
              event.prevent_default();
              handle_change(());
          }
      },
      ..attributes,
      ..extra_attributes,
      {children}
    }
  }
}

#[derive(Clone, PartialEq, Props)]
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
      svg {
        stroke: "currentColor",
        fill: "currentColor",
        stroke_width: "0",
        view_box: "0 0 512 512",
        height: "16px",
        width: "16px",
        xmlns: "http://www.w3.org/2000/svg",
        path {
          fill: "none",
          stroke_linecap: "round",
          stroke_linejoin: "round",
          stroke_width: "32",
          d: "M416 128 192 384l-96-96",
        }
      }
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
