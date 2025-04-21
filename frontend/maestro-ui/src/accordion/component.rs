use {
	dioxus::prelude::*,
	maestro_headless::{accordion::AccordionVariant, shared::EOrientation},
	tailwind_fuse::tw_merge,
};

#[derive(Props, Clone, PartialEq)]
pub struct AccordionProps {
	#[props(default = String::new())]
	pub class: String,
	#[props(optional, default = ReadOnlySignal::new(Signal::new(None)))]
	pub value: ReadOnlySignal<Option<Vec<String>>>,
	#[props(optional, default = Vec::from([String::new()]))]
	pub default_value: Vec<String>,
	#[props(optional)]
	pub on_value_change: Option<Callback<Vec<String>>>,

	#[props(optional, default = ReadOnlySignal::new(Signal::new(EOrientation::Vertical)))]
	pub orientation: ReadOnlySignal<EOrientation>,
	#[props(optional, default = ReadOnlySignal::new(Signal::new(true)))]
	pub collapsible: ReadOnlySignal<bool>,
	#[props(optional, default = ReadOnlySignal::new(Signal::new(false)))]
	pub disabled: ReadOnlySignal<bool>,
	#[props(optional, default = ReadOnlySignal::new(Signal::new(AccordionVariant::Single)))]
	variant: ReadOnlySignal<AccordionVariant>,

	#[props(extends = ul, extends = GlobalAttributes)]
	attributes: Vec<Attribute>,
	pub children: Element,
}

#[component]
pub fn Accordion(props: AccordionProps) -> Element {
	let AccordionProps { value, default_value, on_value_change, orientation, collapsible, disabled, variant, attributes, class, children } = props;

	rsx! {
		maestro_headless::accordion::AccordionRoot {
			class: tw_merge!(
					"relative w-full flex-grow flex data-[orientation=vertical]:flex-col flex-row bg-background p-0.5 transition-all",
					class
			),
			value,
			default_value,
			on_value_change,
			orientation,
			collapsible,
			disabled,
			variant,
			extra_attributes: attributes,
			{children}
		}
	}
}

#[derive(Props, Clone, PartialEq)]
pub struct AccordionItemProps {
	#[props(default = String::new())]
	pub class: String,
	pub value: String,
	#[props(optional, default = ReadOnlySignal::new(Signal::new(false)))]
	pub disabled: ReadOnlySignal<bool>,

	#[props(extends = li, extends = GlobalAttributes)]
	pub attributes: Vec<Attribute>,
	#[props(optional, default = None)]
	pub children: Element,
}

#[component]
pub fn AccordionItem(props: AccordionItemProps) -> Element {
	let AccordionItemProps { value, disabled, class, attributes, children } = props;

	rsx! {
		maestro_headless::accordion::AccordionItem {
			class: tw_merge!(
					"border-border flex data-[orientation=vertical]:flex-col flex-row data-[disabled=true]:opacity-50 data-[disabled=true]:pointer-events-none data-[orientation=vertical]:border-b data-[orientation=horizontal]:border-r",
					class
			),
			value,
			disabled,
			extra_attributes: attributes,
			{children}
		}
	}
}

#[derive(Clone, PartialEq, Props)]
pub struct AccordionTriggerProps {
	#[props(default = ReadOnlySignal::new(Signal::new(false)))]
	pub hide_icon: ReadOnlySignal<bool>,
	#[props(default = String::new())]
	pub class: String,
	#[props(extends = GlobalAttributes, extends = button)]
	pub attributes: Vec<Attribute>,
	pub children: Element,
}

#[component]
pub fn AccordionTrigger(props: AccordionTriggerProps) -> Element {
	let AccordionTriggerProps { hide_icon, class, attributes, children } = props;
	rsx! {
		maestro_headless::accordion::AccordionHeader {
			maestro_headless::accordion::AccordionTrigger {
				class: tw_merge!(
						"flex items-center justify-between gap-3 [&>svg]:flex-shrink-0 [&>svg]:transition-transform [&>svg]:text-muted [&[data-state=open]>svg]:rotate-180 [&[data-orientation=horizontal]>svg]:-rotate-90 [&[data-orientation=horizontal][data-state=open]>svg]:rotate-0 px-3 py-2 h-full w-full transition-all focus-visible:ring-2 focus-visible:ring-ring/50 focus-visible:outline-none hover:underline hover:underline-offset-3",
						class.clone()
				),
				extra_attributes: attributes.clone(),
				{children}
				svg {
					display: hide_icon().then_some("none"),
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
		}
	}
}

#[derive(Clone, PartialEq, Props)]
pub struct AccordionContentProps {
	#[props(default = String::new())]
	pub class: String,
	#[props(extends = div, extends = GlobalAttributes)]
	attributes: Vec<Attribute>,
	pub children: Element,
}

#[component]
pub fn AccordionContent(props: AccordionContentProps) -> Element {
	rsx! {
		maestro_headless::accordion::AccordionContent {
			class: tw_merge!(
					"flex w-full h-full data-[orientation=horizontal]:data-[state=open]:w-fit transition-all data-[orientation=horizontal]:data-[state=open]:animate-slide-in-accordion-horizontal data-[orientation=horizontal]:data-[state=closed]:animate-slide-out-accordion-horizontal data-[orientation=vertical]:data-[state=open]:animate-slide-in-accordion-vertical data-[orientation=vertical]:data-[state=closed]:animate-slide-out-accordion-vertical transition-all overflow-hidden",
					props.class.clone()
			),
			extra_attributes: props.attributes,
			div { class: "px-3 pb-2 pt-0 data-[orientation=horizontal]:pt-2", {props.children} }
		}
	}
}
