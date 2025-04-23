use {
	crate::shared::{EAlign, EClass, ERound, ESide, ESize, EVariant},
	dioxus::prelude::*,
	tailwind_fuse::*,
};

#[derive(Props, PartialEq, Clone)]
pub struct TooltipRootProps {
	#[props(default = String::new())]
	pub class: String,
	#[props(default = 700.0)]
	pub delay_duration_ms: f32,
	#[props(default = 300.0)]
	pub skip_delay_duration_ms: f32,

	#[props(extends = GlobalAttributes, extends = div)]
	pub attributes: Vec<Attribute>,
	children: Element,
}

#[component]
pub fn TooltipRoot(props: TooltipRootProps) -> Element {
	let TooltipRootProps { class, delay_duration_ms, skip_delay_duration_ms, attributes, children } = props;

	rsx! {
		maestro_headless::tooltip::TooltipRoot {
			class: tw_merge!("w-fit", class.clone()),
			extra_attributes: attributes.clone(),
			{children}
		}
	}
}

#[derive(Props, PartialEq, Clone)]
pub struct TooltipProps {
	#[props(default = String::new())]
	pub class: String,
	#[props(optional, default = ReadOnlySignal::new(Signal::new(None)))]
	pub open: ReadOnlySignal<Option<bool>>,
	#[props(optional, default = false)]
	pub default_open: bool,
	#[props(optional)]
	pub on_open_change: Option<Callback<bool>>,
	#[props(optional)]
	delay_duration: Option<f32>,

	#[props(extends = GlobalAttributes, extends = div)]
	pub attributes: Vec<Attribute>,
	children: Element,
}

#[component]
pub fn Tooltip(props: TooltipProps) -> Element {
	let TooltipProps { class, open, default_open, on_open_change, delay_duration, children, attributes } = props;

	rsx! {
		maestro_headless::tooltip::Tooltip {
			open,
			default_open,
			on_open_change,
			delay_duration,
			extra_attributes: attributes,
			class: tw_merge!("w-fit", class.clone()),
			{children}
		}
	}
}

#[derive(Clone, PartialEq, Props)]
pub struct TooltipTriggerProps {
	#[props(default = String::new())]
	pub class: String,
	#[props(default = ReadOnlySignal::new(Signal::new(EVariant::Outline)))]
	pub variant: ReadOnlySignal<EVariant>,
	#[props(default = ReadOnlySignal::new(Signal::new(ERound::Md)))]
	pub round: ReadOnlySignal<ERound>,
	#[props(default = ReadOnlySignal::new(Signal::new(ESize::Md)))]
	pub size: ReadOnlySignal<ESize>,
	#[props(optional)]
	onclick: Option<EventHandler<MouseEvent>>,

	#[props(extends = GlobalAttributes, extends = button)]
	pub attributes: Vec<Attribute>,
	#[props(default = Vec::new())]
	pub children: Element,
}

#[component]
pub fn TooltipTrigger(props: TooltipTriggerProps) -> Element {
	let TooltipTriggerProps { class, variant, size, round, attributes, onclick, children } = props;
	let class = EClass { variant: variant(), size: size(), round: round() }.with_class(class.clone());

	rsx! {
		maestro_headless::tooltip::TooltipTrigger { class, extra_attributes: attributes.clone(), onclick, {children} }
	}
}

#[derive(Clone, PartialEq, Props)]
pub struct TooltipContentProps {
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
pub fn TooltipContent(props: TooltipContentProps) -> Element {
	let TooltipContentProps { class, side, side_offset, align, align_offset, avoid_collisions, collision_padding, attributes, children } = props;

	rsx! {
		maestro_headless::tooltip::TooltipContent {
			class: tw_merge!(
					"bg-popover text-popover-foreground animate-in fade-in-0 data-[state=closed]:animate-out data-[state=closed]:fade-out-0 data-[side=bottom]:slide-in-from-top-2 data-[side=left]:slide-in-from-right-2 data-[side=right]:slide-in-from-left-2 data-[side=top]:slide-in-from-bottom-2 z-50 w-fit origin-(--maestro-headless-tooltip-content-transform-origin) rounded-md px-3 py-1.5 text-xs text-balance",
					class.clone()
			),
			side,
			side_offset,
			align,
			align_offset,
			avoid_collisions,
			collision_padding,
			extra_attributes: attributes.clone(),
			{children}
			maestro_headless::tooltip::TooltipArrow { class: "text-popover z-50" }
		}
	}
}

// #[derive(Clone, PartialEq, Props)]
// pub struct TooltipArrowProps {
// 	#[props(default = 10.0)]
// 	width: f32,
// 	#[props(default = 5.0)]
// 	height: f32,
// 	#[props(extends = svg, extends = GlobalAttributes)]
// 	pub attributes: Vec<Attribute>,
// 	#[props(default = None)]
// 	pub children: Option<Element>,
// }

// #[component]
// pub fn TooltipArrow(props: TooltipArrowProps) -> Element {
// 	let TooltipArrowProps { width, height, attributes, children } = props;

// 	rsx! {
// 		PopperArrow {
// 			width,
// 			height,
// 			extra_attributes: attributes.clone(),
// 			children,
// 		}
// 	}
// }
