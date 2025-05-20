use {crate::shared::EOrientation, dioxus::prelude::*, tailwind_fuse::*};

#[derive(Clone, PartialEq, Props)]
pub struct TabsProps {
	#[props(default = String::new())]
	pub class: String,
	#[props(optional, default = ReadOnlySignal::new(Signal::new(None)))]
	pub value: ReadOnlySignal<Option<String>>,
	#[props(optional, default = String::new())]
	pub default_value: String,
	#[props(optional)]
	pub on_value_change: Option<Callback<String>>,

	#[props(optional, default = ReadOnlySignal::new(Signal::new(EOrientation::Horizontal)))]
	pub orientation: ReadOnlySignal<EOrientation>,

	#[props(extends = div, extends = GlobalAttributes)]
	attributes: Vec<Attribute>,
	pub children: Element,
}

#[component]
pub fn Tabs(props: TabsProps) -> Element {
	let TabsProps { class, value, default_value, on_value_change, children, orientation, attributes } = props;

	rsx! {
		maestro_headless::tabs::TabsRoot {
			class: tw_merge!("flex flex-col data-[orientation=vertical]:flex-row gap-2", class.clone()),
			value,
			default_value,
			on_value_change,
			orientation,
			extra_attributes: attributes,
			{children}
		}
	}
}

#[derive(Clone, PartialEq, Props)]
pub struct TabsListProps {
	#[props(default = String::new())]
	pub class: String,
	#[props(extends = div, extends = GlobalAttributes)]
	pub attributes: Vec<Attribute>,
	#[props(optional, default = None)]
	pub children: Element,
}

#[component]
pub fn TabsList(props: TabsListProps) -> Element {
	rsx! {
		maestro_headless::tabs::TabsList {
			class: tw_merge!(
					"w-fit flex items-center data-[orientation=vertical]:flex-col gap-2 border border-transparent data-[orientation=vertical]:border-r-muted/30 data-[orientation=horizontal]:border-b-muted/30",
					props.class.clone()
			),
			extra_attributes: props.attributes.clone(),
			{props.children}
		}
	}
}

#[derive(Clone, PartialEq, Props)]
pub struct TabsTriggerProps {
	#[props(default = String::new())]
	pub class: String,
	pub value: ReadOnlySignal<String>,
	#[props(optional, default = ReadOnlySignal::new(Signal::new(false)))]
	pub disabled: ReadOnlySignal<bool>,

	#[props(extends = GlobalAttributes, extends = button)]
	pub attributes: Vec<Attribute>,
	#[props(optional, default = None)]
	pub children: Element,
}

#[component]
pub fn TabsTrigger(props: TabsTriggerProps) -> Element {
	let TabsTriggerProps { class, value, disabled, attributes, children } = props;

	rsx! {
		maestro_headless::tabs::TabsTrigger {
			class: tw_merge!(
					"w-full h-full font-medium text-foreground border border-transparent data-[orientation=vertical]:border-r data-[orientation=vertical]:pr-2 data-[orientation=horizontal]:pb-2 data-[orientation=horizontal]:border-b-transparent data-[orientation=vertical]:border-r-transparent data-[orientation=horizontal]:hover:border-b-primary/80 data-[orientation=vertical]:hover:border-r-primary/80 data-[orientation=horizontal]:data-[state=active]:border-b-primary data-[orientation=vertical]:data-[state=active]:border-r-primary transition-all ease-linear focus-visible:outline-none data-[orientation=horizontal]:data-[state=active]:focus-visible:border-b-primary data-[orientation=vertical]:data-[state=active]:focus-visible:border-r-primary focus-visible:outline-none data-[disabled=true]:opacity-50 data-[disabled=true]:pointer-events-none data-[state=active]:shadow-sm [&_svg]:pointer-events-none [&_svg]:shrink-0",
					class.clone()
			),
			value,
			disabled,
			extra_attributes: attributes.clone(),
			{children}
		}
	}
}

#[derive(Clone, PartialEq, Props)]
pub struct TabsContentProps {
	#[props(default = String::new())]
	pub class: String,
	pub value: ReadOnlySignal<String>,

	#[props(extends = div, extends = GlobalAttributes)]
	attributes: Vec<Attribute>,
	pub children: Element,
}

#[component]
pub fn TabsContent(props: TabsContentProps) -> Element {
	let TabsContentProps { class, value, attributes, children } = props;

	rsx! {
		maestro_headless::tabs::TabsContent {
			value,
			extra_attributes: attributes.clone(),
			class: tw_merge!("flex-1", class.clone()),
			{children}
		}
	}
}
