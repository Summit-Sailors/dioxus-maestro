use {
	crate::shared::{EAlign, EClass, ERound, ESide, ESize, EVariant},
	dioxus::prelude::*,
	tailwind_fuse::*,
};

#[derive(Clone, PartialEq, Props)]
pub struct PopoverProps {
	#[props(optional, default = ReadOnlySignal::new(Signal::new(None)))]
	pub open: ReadOnlySignal<Option<bool>>,
	#[props(optional, default = false)]
	pub default_open: bool,
	#[props(optional)]
	pub on_open_change: Option<Callback<bool>>,
	#[props(default = String::new())]
	pub class: String,
	#[props(extends = div, extends = GlobalAttributes)]
	pub attributes: Vec<Attribute>,
	pub children: Element,
}

#[component]
pub fn Popover(props: PopoverProps) -> Element {
	let PopoverProps { open, default_open, on_open_change, children, attributes, class } = props;

	rsx! {
		maestro_headless::popover::PopoverRoot {
			open,
			default_open,
			on_open_change,
			extra_attributes: attributes,
			class: tw_merge!("w-fit", class.clone()),
			{children}
		}
	}
}

#[derive(Clone, PartialEq, Props)]
pub struct PopoverTriggerProps {
	#[props(default = String::new())]
	pub class: String,
	#[props(default = ReadOnlySignal::new(Signal::new(EVariant::Outline)))]
	pub variant: ReadOnlySignal<EVariant>,
	#[props(default = ReadOnlySignal::new(Signal::new(ERound::Md)))]
	pub round: ReadOnlySignal<ERound>,
	#[props(default = ReadOnlySignal::new(Signal::new(ESize::Md)))]
	pub size: ReadOnlySignal<ESize>,
	#[props(extends = GlobalAttributes, extends = button)]
	pub attributes: Vec<Attribute>,
	#[props(optional)]
	pub children: Element,
}

#[component]
pub fn PopoverTrigger(props: PopoverTriggerProps) -> Element {
	let PopoverTriggerProps { class, variant, size, round, attributes, children } = props;

	let class = EClass { variant: variant(), size: size(), round: round() }.with_class(class.clone());

	rsx! {
		maestro_headless::popover::PopoverTrigger { class, extra_attributes: attributes, {children} }
	}
}

#[derive(Clone, PartialEq, Props)]
pub struct PopoverContentProps {
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
	#[props(extends = GlobalAttributes)]
	pub attributes: Vec<Attribute>,
	pub children: Element,
}

#[component]
pub fn PopoverContent(props: PopoverContentProps) -> Element {
	let PopoverContentProps { class, side, side_offset, align, align_offset, avoid_collisions, collision_padding, attributes, children } = props;

	rsx! {
		maestro_headless::popover::PopoverContent {
			side,
			side_offset,
			align,
			align_offset,
			avoid_collisions,
			collision_padding,
			extra_attributes: attributes.clone(),
			class: tw_merge!(
					"bg-popover text-popover-foreground data-[state=open]:animate-in data-[state=closed]:animate-out data-[state=closed]:fade-out-0 data-[state=open]:fade-in-0 data-[side=bottom]:slide-in-from-top-2 data-[side=left]:slide-in-from-right-2 data-[side=right]:slide-in-from-left-2 data-[side=top]:slide-in-from-bottom-2 z-50 w-72 rounded-md p-4 shadow-md outline-hidden",
					class.clone()
			),
			{children}
		}
	}
}
