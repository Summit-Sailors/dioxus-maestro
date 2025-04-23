use {
	crate::shared::{EAlign, EClass, ERound, ESide, ESize, EVariant},
	dioxus::prelude::*,
	tailwind_fuse::*,
};

#[derive(Props, PartialEq, Clone)]
pub struct HoverCardProps {
	#[props(default = String::new())]
	pub class: String,
	#[props(optional, default = ReadOnlySignal::new(Signal::new(None)))]
	pub open: ReadOnlySignal<Option<bool>>,
	#[props(optional, default = false)]
	pub default_open: bool,
	#[props(optional)]
	pub on_open_change: Option<Callback<bool>>,

	#[props(default = ReadOnlySignal::new(Signal::new(700.0)))]
	open_delay: ReadOnlySignal<f32>,
	#[props(default = ReadOnlySignal::new(Signal::new(300.0)))]
	close_delay: ReadOnlySignal<f32>,

	#[props(extends = GlobalAttributes, extends = div)]
	pub attributes: Vec<Attribute>,
	children: Element,
}

#[component]
pub fn HoverCard(props: HoverCardProps) -> Element {
	let HoverCardProps { class, open, default_open, on_open_change, open_delay, close_delay, children, attributes } = props;

	rsx! {
		maestro_headless::hover_card::HoverCardRoot {
			class: tw_merge!("w-fit", class.clone()),
			open,
			default_open,
			on_open_change,
			open_delay,
			close_delay,
			extra_attributes: attributes,
			{children}
		}
	}
}

#[derive(Clone, PartialEq, Props)]
pub struct HoverCardTriggerProps {
	#[props(default = String::new())]
	pub class: String,
	#[props(default = ReadOnlySignal::new(Signal::new(EVariant::Link)))]
	pub variant: ReadOnlySignal<EVariant>,
	#[props(default = ReadOnlySignal::new(Signal::new(ERound::Md)))]
	pub round: ReadOnlySignal<ERound>,
	#[props(default = ReadOnlySignal::new(Signal::new(ESize::Md)))]
	pub size: ReadOnlySignal<ESize>,
	#[props(extends = GlobalAttributes, extends = a)]
	pub attributes: Vec<Attribute>,
	#[props(optional)]
	pub children: Element,
}

#[component]
pub fn HoverCardTrigger(props: HoverCardTriggerProps) -> Element {
	let HoverCardTriggerProps { class, variant, round, size, attributes, children } = props;
	let class = EClass { variant: variant(), size: size(), round: round() }.with_class(class.clone());

	rsx! {
		maestro_headless::hover_card::HoverCardTrigger { class, extra_attributes: attributes, {children} }
	}
}

#[derive(Clone, PartialEq, Props)]
pub struct HoverCardContentProps {
	#[props(default = ReadOnlySignal::new(Signal::new(ESide::Bottom)))]
	side: ReadOnlySignal<ESide>,
	#[props(default = ReadOnlySignal::new(Signal::new(0.0)))]
	side_offset: ReadOnlySignal<f32>,
	#[props(default = ReadOnlySignal::new(Signal::new(EAlign::Center)))]
	align: ReadOnlySignal<EAlign>,
	#[props(default = ReadOnlySignal::new(Signal::new(0.0)))]
	align_offset: ReadOnlySignal<f32>,
	#[props(default = ReadOnlySignal::new(Signal::new(true)))]
	avoid_collisions: ReadOnlySignal<bool>,
	#[props(default = ReadOnlySignal::new(Signal::new(4.0)))]
	collision_padding: ReadOnlySignal<f32>,
	#[props(default = String::new())]
	pub class: String,

	#[props(optional)]
	onmouseenter: Option<EventHandler<MouseEvent>>,
	#[props(optional)]
	onmouseleave: Option<EventHandler<MouseEvent>>,
	#[props(optional)]
	#[props(extends = GlobalAttributes)]
	pub attributes: Vec<Attribute>,
	pub children: Element,
}

#[component]
pub fn HoverCardContent(props: HoverCardContentProps) -> Element {
	let HoverCardContentProps {
		class,
		side,
		side_offset,
		align,
		align_offset,
		avoid_collisions,
		collision_padding,
		onmouseenter,
		onmouseleave,
		attributes,
		children,
	} = props;

	rsx! {
		maestro_headless::hover_card::HoverCardContent {
			side,
			side_offset,
			align,
			align_offset,
			avoid_collisions,
			collision_padding,
			extra_attributes: attributes,
			onmouseenter,
			onmouseleave,
			class: tw_merge!(
					"bg-popover text-popover-foreground data-[state=open]:animate-in data-[state=closed]:animate-out data-[state=closed]:fade-out-0 data-[state=open]:fade-in-0 data-[side=bottom]:slide-in-from-top-2 data-[side=left]:slide-in-from-right-2 data-[side=right]:slide-in-from-left-2 data-[side=top]:slide-in-from-bottom-2 z-50 w-64 origin-(--radix-hover-card-content-transform-origin) rounded-md p-4 shadow-md outline-hidden",
					class.clone()
			),
			maestro_headless::hover_card::HoverCardArrow { class: "text-popover" }
			{children}
		}
	}
}
