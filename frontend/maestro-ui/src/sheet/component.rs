use {
	crate::shared::{EClass, ERound, ESide, ESize, EVariant},
	dioxus::prelude::*,
	dioxus_free_icons::{icons::ld_icons::LdX, Icon},
	maestro_headless::dialog::DialogRootProps,
	tailwind_fuse::{merge::tw_merge, *},
};

#[component]
pub fn Sheet(props: DialogRootProps) -> Element {
	let DialogRootProps { open, default_open, on_open_change, on_close, children } = props;
	rsx! {
		maestro_headless::dialog::DialogRoot {
			open,
			default_open,
			on_open_change,
			on_close,
			{children}
		}
	}
}

#[derive(Clone, PartialEq, Props)]
pub struct SheetTriggerProps {
	#[props(default = String::new())]
	pub class: String,
	#[props(default = ReadOnlySignal::new(Signal::new(EVariant::Primary)))]
	pub variant: ReadOnlySignal<EVariant>,
	#[props(default = ReadOnlySignal::new(Signal::new(ERound::Md)))]
	pub round: ReadOnlySignal<ERound>,
	#[props(default = ReadOnlySignal::new(Signal::new(ESize::Md)))]
	pub size: ReadOnlySignal<ESize>,
	#[props(default = ReadOnlySignal::new(Signal::new(false)))]
	disabled: ReadOnlySignal<bool>,
	#[props(extends = GlobalAttributes, extends = button)]
	pub attributes: Vec<Attribute>,
	#[props(optional)]
	pub children: Element,
}

#[component]
pub fn SheetTrigger(props: SheetTriggerProps) -> Element {
	let SheetTriggerProps { class, attributes, disabled, variant, round, size, children } = props;
	let class = EClass { variant: variant(), size: size(), round: round() }.with_class(class.clone());

	rsx! {
		maestro_headless::dialog::DialogTrigger { disabled, extra_attributes: attributes, class, {children} }
	}
}

#[derive(Clone, PartialEq, Props)]
pub struct SheetOverlayProps {
	#[props(default = String::new())]
	pub class: String,
}

#[component]
pub fn SheetOverlay(props: SheetOverlayProps) -> Element {
	rsx! {
		maestro_headless::dialog::DialogOverlay {
			class: tw_merge!(
					"data-[state=open]:animate-in data-[state=closed]:animate-out data-[state=closed]:fade-out-0 data-[state=open]:fade-in-0 fixed inset-0 z-50 bg-[#000]/30 backdrop-blur-xs",
					props.class.clone()
			),
		}
	}
}

#[derive(Clone, PartialEq, Props)]
pub struct SheetContentProps {
	#[props(default = String::new())]
	pub class: String,
	#[props(default = ReadOnlySignal::new(Signal::new(ESide::Right)))]
	side: ReadOnlySignal<ESide>,

	#[props(extends = GlobalAttributes)]
	pub attributes: Vec<Attribute>,
	pub children: Element,
}

#[component]
pub fn SheetContent(props: SheetContentProps) -> Element {
	let side_class = match *props.side.read() {
		ESide::Top => "data-[state=closed]:slide-out-to-top data-[state=open]:slide-in-from-top inset-x-0 top-0 h-auto border-b-border",
		ESide::Right => "data-[state=closed]:slide-out-to-right data-[state=open]:slide-in-from-right inset-y-0 right-0 h-full w-3/4 border-l-border sm:max-w-sm",
		ESide::Bottom => "data-[state=closed]:slide-out-to-bottom data-[state=open]:slide-in-from-bottom inset-x-0 bottom-0 h-auto border-t-border",
		ESide::Left => "data-[state=closed]:slide-out-to-left data-[state=open]:slide-in-from-left inset-y-0 left-0 h-full w-3/4 border-r-border sm:max-w-sm",
	};

	rsx! {
		SheetOverlay {}
		maestro_headless::dialog::DialogContent {
			"data-side": props.side.read().to_string().to_lowercase(),
			extra_attributes: props.attributes.clone(),
			class: tw_merge!(
					"relative border-transparent border bg-background data-[state=open]:animate-in data-[state=closed]:animate-out fixed z-50 w-full shadow-lg transition ease-in-out data-[state=closed]:duration-300 data-[state=open]:duration-500 gap-4 p-6 shadow-lg",
					side_class, props.class.clone()
			),
			{props.children}
			maestro_headless::dialog::DialogClose { class: "ring-offset-background focus:ring-ring data-[state=open]:bg-accent data-[state=open]:text-muted-foreground absolute top-4 right-4 rounded-xs opacity-70 transition-opacity hover:opacity-100 focus:ring-2 focus:ring-offset-2 focus:outline-hidden disabled:pointer-events-none [&_svg]:pointer-events-none [&_svg]:shrink-0 [&_svg:not([class*='size-'])]:size-4",
				Icon { icon: LdX }
			}
		}
	}
}

#[derive(Clone, PartialEq, Props)]
pub struct SheetHeaderProps {
	#[props(default = String::new())]
	pub class: String,
	#[props(extends = GlobalAttributes, extends=div)]
	pub attributes: Vec<Attribute>,
	pub children: Element,
}

#[component]
pub fn SheetHeader(props: SheetTitleProps) -> Element {
	rsx! {
		maestro_headless::dialog::DialogHeader {
			class: tw_merge!("flex flex-col gap-2 text-center sm:text-left", props.class.clone()),
			extra_attributes: props.attributes,
			{props.children}
		}
	}
}

#[derive(Clone, PartialEq, Props)]
pub struct SheetFooterProps {
	#[props(default = String::new())]
	pub class: String,
	#[props(extends = GlobalAttributes, extends=div)]
	pub attributes: Vec<Attribute>,
	pub children: Element,
}

#[component]
pub fn SheetFooter(props: SheetTitleProps) -> Element {
	rsx! {
		maestro_headless::dialog::DialogFooter {
			class: tw_merge!("flex flex-col sm:flex-row gap-2", props.class.clone()),
			extra_attributes: props.attributes,
			{props.children}
		}
	}
}

#[derive(Clone, PartialEq, Props)]
pub struct SheetBodyProps {
	#[props(default = String::new())]
	pub class: String,
	#[props(extends = GlobalAttributes, extends=div)]
	pub attributes: Vec<Attribute>,
	pub children: Element,
}

#[component]
pub fn SheetBody(props: SheetTitleProps) -> Element {
	rsx! {
		maestro_headless::dialog::DialogBody {
			class: tw_merge!("flex flex-col gap-3", props.class),
			extra_attributes: props.attributes,
			{props.children}
		}
	}
}

#[derive(Clone, PartialEq, Props)]
pub struct SheetTitleProps {
	#[props(default = String::new())]
	pub class: String,
	#[props(extends = GlobalAttributes)]
	pub attributes: Vec<Attribute>,
	pub children: Element,
}
#[component]
pub fn SheetTitle(props: SheetTitleProps) -> Element {
	rsx! {
		maestro_headless::dialog::DialogTitle {
			class: tw_merge!("text-lg font-semibold", props.class.clone()),
			extra_attributes: props.attributes,
			{props.children}
		}
	}
}

#[derive(Clone, PartialEq, Props)]
pub struct SheetDescriptionProps {
	#[props(default = String::new())]
	pub class: String,
	#[props(extends = GlobalAttributes, extends=div)]
	pub attributes: Vec<Attribute>,
	pub children: Element,
}

#[component]
pub fn SheetDescription(props: SheetDescriptionProps) -> Element {
	rsx! {
		maestro_headless::dialog::DialogDescription {
			class: tw_merge!("text-muted-foreground text-sm", props.class.clone()),
			extra_attributes: props.attributes,
			{props.children}
		}
	}
}

#[derive(Clone, PartialEq, Props)]
pub struct SheetCloseProps {
	#[props(default = String::new())]
	pub class: String,
	#[props(default = ReadOnlySignal::new(Signal::new(EVariant::Primary)))]
	pub variant: ReadOnlySignal<EVariant>,
	#[props(default = ReadOnlySignal::new(Signal::new(ERound::Md)))]
	pub round: ReadOnlySignal<ERound>,
	#[props(default = ReadOnlySignal::new(Signal::new(ESize::Md)))]
	pub size: ReadOnlySignal<ESize>,
	pub children: Element,
	#[props(extends = GlobalAttributes, extends = button)]
	pub attributes: Vec<Attribute>,
}

#[component]
pub fn SheetClose(props: SheetCloseProps) -> Element {
	let class = EClass { variant: *props.variant.read(), size: *props.size.read(), round: *props.round.read() }.with_class(props.class.clone());
	rsx! {
		maestro_headless::dialog::DialogClose { class, extra_attributes: props.attributes, {props.children} }
	}
}
