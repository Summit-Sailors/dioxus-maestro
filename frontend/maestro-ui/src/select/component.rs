use {
	crate::shared::{EAlign, EClass, ERound, ESide, ESize, EVariant},
	dioxus::prelude::*,
	tailwind_fuse::*,
};

#[derive(Clone, PartialEq, Props)]
pub struct SelectProps {
	#[props(default = String::new())]
	pub class: String,
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
	pub children: Element,
}

#[component]
pub fn Select(props: SelectProps) -> Element {
	let SelectProps { open, default_open, on_open_change, disabled, value, default_value, on_value_change, required, multi, attributes, class, children } = props;

	rsx! {
		maestro_headless::select::SelectRoot {
			class: tw_merge!("w-fit relative", class.clone()),
			open,
			default_open,
			on_open_change,
			disabled,
			value,
			default_value,
			on_value_change,
			required,
			multi,
			extra_attributes: attributes.clone(),
			{children}
		}
	}
}

#[derive(Clone, PartialEq, Props)]
pub struct SelectTriggerProps {
	#[props(default = String::new())]
	pub class: String,
	#[props(default = ReadOnlySignal::new(Signal::new(EVariant::Outline)))]
	pub variant: ReadOnlySignal<EVariant>,
	#[props(default = ReadOnlySignal::new(Signal::new(ERound::Md)))]
	pub round: ReadOnlySignal<ERound>,
	#[props(default = ReadOnlySignal::new(Signal::new(ESize::Md)))]
	pub size: ReadOnlySignal<ESize>,
	#[props(default = None)]
	pub onclick: Option<EventHandler<Event<MouseData>>>,

	#[props(extends = button, extends = GlobalAttributes)]
	attributes: Vec<Attribute>,
	pub children: Element,
}

#[component]
pub fn SelectTrigger(props: SelectTriggerProps) -> Element {
	let SelectTriggerProps { class, variant, size, round, onclick, attributes, children } = props;

	let class = EClass { variant: variant(), size: size(), round: round() }.with_class(tw_merge!("[&>svg]:shrink-0", class.clone()));

	rsx! {
		maestro_headless::select::SelectTrigger { class, onclick, extra_attributes: attributes,
			{children}
			maestro_headless::select::SelectIcon { class: "size-4 shrink-0" }
		}
	}
}

#[derive(Clone, PartialEq, Props)]
pub struct SelectValueProps {
	#[props(default = String::new())]
	pub class: String,
	#[props(default = String::default())]
	pub placeholder: String,

	#[props(extends = span, extends = GlobalAttributes)]
	attributes: Vec<Attribute>,
	#[props(default = None)]
	children: Option<Element>,
}

#[component]
pub fn SelectValue(props: SelectValueProps) -> Element {
	let SelectValueProps { class, placeholder, attributes, .. } = props;

	rsx! {
		maestro_headless::select::SelectValue {
			class: tw_merge!(
					"data-[placeholder]:text-muted-foreground overflow-hidden overflow-ellipsis whitespace-nowrap max-w-[90%] block",
					class
			),
			extra_attributes: attributes,
			placeholder,
		}
	}
}

#[derive(Clone, PartialEq, Props)]
pub struct SelectDropdownProps {
	#[props(default = String::new())]
	pub class: String,
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
	let SelectDropdownProps { class, side, side_offset, align, align_offset, avoid_collisions, collision_padding, attributes, children, .. } = props;

	rsx! {
		maestro_headless::select::SelectDropdown {
			class: tw_merge!(
					"bg-popover text-popover-foreground data-[state=open]:animate-in data-[state=closed]:animate-out data-[state=closed]:fade-out-0 data-[state=open]:fade-in-0 data-[side=bottom]:slide-in-from-top-2 data-[side=left]:slide-in-from-right-2 data-[side=right]:slide-in-from-left-2 data-[side=top]:slide-in-from-bottom-2 relative z-50 max-h-52 min-w-[8rem] px-2 py-4 origin-(--radix-select-content-transform-origin) overflow-x-hidden overflow-y-auto rounded-md border border-input shadow-md",
					class
			),
			side,
			side_offset,
			align,
			align_offset,
			avoid_collisions,
			collision_padding,
			extra_attributes: attributes.clone(),
			{children}
		}
	}
}

#[derive(Clone, PartialEq, Props)]
pub struct SelectOptionProps {
	#[props(default = String::new())]
	pub class: String,
	pub value: String,
	#[props(optional, default = ReadOnlySignal::new(Signal::new(false)))]
	pub disabled: ReadOnlySignal<bool>,

	#[props(extends = GlobalAttributes, extends = div)]
	attributes: Vec<Attribute>,
	#[props(default = None)]
	children: Element,
}

#[component]
pub fn SelectOption(props: SelectOptionProps) -> Element {
	let SelectOptionProps { class, value, disabled, attributes, children } = props;

	rsx! {
		maestro_headless::select::SelectOption {
			class: tw_merge!(
					"group transition-colors focus-visible:bg-accent hover:bg-accent focus-visible:text-accent-foreground hover:text-accent-foreground relative flex w-full items-center justify-between gap-4 rounded-sm py-1.5 px-2 max-w-[96%] w-full text-sm outline-hidden select-none data-[disabled=true]:pointer-events-none data-[disabled=true]:opacity-50 [&_svg]:pointer-events-none",
					class
			),
			value,
			disabled,
			extra_attributes: attributes.clone(),
			{children}
			maestro_headless::select::OptionSelectedIndicator { class: "transition-colors shrink-0 size-4 text-popover-foreground/80 group-hover:text-popover-foreground " }
		}
	}
}
