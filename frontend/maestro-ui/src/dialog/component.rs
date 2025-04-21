use {
	crate::button::{ButtonClass, ButtonRound, ButtonSize, ButtonVariant},
	dioxus::prelude::*,
	dioxus_free_icons::{icons::ld_icons::LdX, Icon},
	maestro_headless::dialog::DialogRootProps,
	tailwind_fuse::{merge::tw_merge, *},
};

#[component]
pub fn Dialog(props: DialogRootProps) -> Element {
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
pub struct DialogTriggerProps {
	#[props(default = String::new())]
	pub class: String,
	#[props(default = ReadOnlySignal::new(Signal::new(ButtonVariant::Primary)))]
	pub variant: ReadOnlySignal<ButtonVariant>,
	#[props(default = ReadOnlySignal::new(Signal::new(ButtonRound::Md)))]
	pub round: ReadOnlySignal<ButtonRound>,
	#[props(default = ReadOnlySignal::new(Signal::new(ButtonSize::Md)))]
	pub size: ReadOnlySignal<ButtonSize>,
	#[props(default = ReadOnlySignal::new(Signal::new(false)))]
	disabled: ReadOnlySignal<bool>,
	#[props(extends = GlobalAttributes, extends = button)]
	pub attributes: Vec<Attribute>,
	#[props(optional)]
	pub children: Element,
}

#[component]
pub fn DialogTrigger(props: DialogTriggerProps) -> Element {
	let DialogTriggerProps { class, attributes, disabled, variant, round, size, children } = props;
	let class = ButtonClass { variant: variant(), size: size(), round: round() }.with_class(class.clone());

	rsx! {
		maestro_headless::dialog::DialogTrigger { disabled, extra_attributes: attributes, class, {children} }
	}
}

#[derive(Clone, PartialEq, Props)]
pub struct DialogOverlayProps {
	#[props(default = String::new())]
	pub class: String,
}

#[component]
pub fn DialogOverlay(props: DialogOverlayProps) -> Element {
	rsx! {
		maestro_headless::dialog::DialogOverlay {
			class: tw_merge!(
					"data-[state=closed]:animate-fade-out data-[state=open]:animate-fade-in fixed inset-0 z-50 bg-[#000]/30 backdrop-blur-xs",
					props.class.clone()
			),
		}
	}
}

#[derive(Clone, PartialEq, Props)]
pub struct DialogContentProps {
	#[props(default = String::new())]
	pub class: String,
	#[props(extends = GlobalAttributes)]
	pub attributes: Vec<Attribute>,
	pub children: Element,
}

#[component]
pub fn DialogContent(props: DialogContentProps) -> Element {
	rsx! {
		DialogOverlay {}
		maestro_headless::dialog::DialogContent {
			extra_attributes: props.attributes.clone(),
			class: tw_merge!(
					"relative bg-background data-[state=closed]:animate-fade-out data-[state=open]:animate-fade-in fixed top-[50%] left-[50%] z-50 w-full max-w-[98%] translate-x-[-50%] translate-y-[-50%] gap-4 rounded-md border p-6 shadow-lg duration-200 lg:max-w-lg",
					props.class.clone()
			),
			{props.children}
			maestro_headless::dialog::DialogClose { class: "ring-offset-background focus:ring-ring data-[state=open]:bg-accent data-[state=open]:text-muted-foreground absolute top-4 right-4 rounded-xs opacity-70 transition-opacity hover:opacity-100 focus:ring-2 focus:ring-offset-2 focus:outline-hidden disabled:pointer-events-none [&_svg]:pointer-events-none [&_svg]:shrink-0 [&_svg:not([class*='size-'])]:size-4",
				Icon { icon: LdX }
			}
		}
	}
}

#[derive(Clone, PartialEq, Props)]
pub struct DialogHeaderProps {
	#[props(default = String::new())]
	pub class: String,
	#[props(extends = GlobalAttributes, extends=div)]
	pub attributes: Vec<Attribute>,
	pub children: Element,
}

#[component]
pub fn DialogHeader(props: DialogTitleProps) -> Element {
	rsx! {
		maestro_headless::dialog::DialogHeader {
			class: tw_merge!("flex flex-col gap-2 text-center sm:text-left", props.class.clone()),
			extra_attributes: props.attributes,
			{props.children}
		}
	}
}

#[derive(Clone, PartialEq, Props)]
pub struct DialogFooterProps {
	#[props(default = String::new())]
	pub class: String,
	#[props(extends = GlobalAttributes, extends=div)]
	pub attributes: Vec<Attribute>,
	pub children: Element,
}

#[component]
pub fn DialogFooter(props: DialogTitleProps) -> Element {
	rsx! {
		maestro_headless::dialog::DialogFooter {
			class: tw_merge!("flex flex-col sm:flex-row gap-2", props.class.clone()),
			extra_attributes: props.attributes,
			{props.children}
		}
	}
}

#[derive(Clone, PartialEq, Props)]
pub struct DialogBodyProps {
	#[props(default = String::new())]
	pub class: String,
	#[props(extends = GlobalAttributes, extends=div)]
	pub attributes: Vec<Attribute>,
	pub children: Element,
}

#[component]
pub fn DialogBody(props: DialogTitleProps) -> Element {
	rsx! {
		maestro_headless::dialog::DialogBody {
			class: tw_merge!("flex flex-col gap-3", props.class),
			extra_attributes: props.attributes,
			{props.children}
		}
	}
}

#[derive(Clone, PartialEq, Props)]
pub struct DialogTitleProps {
	#[props(default = String::new())]
	pub class: String,
	#[props(extends = GlobalAttributes)]
	pub attributes: Vec<Attribute>,
	pub children: Element,
}
#[component]
pub fn DialogTitle(props: DialogTitleProps) -> Element {
	rsx! {
		maestro_headless::dialog::DialogTitle {
			class: tw_merge!("text-lg font-semibold", props.class.clone()),
			extra_attributes: props.attributes,
			{props.children}
		}
	}
}

#[derive(Clone, PartialEq, Props)]
pub struct DialogDescriptionProps {
	#[props(default = String::new())]
	pub class: String,
	#[props(extends = GlobalAttributes, extends=div)]
	pub attributes: Vec<Attribute>,
	pub children: Element,
}

#[component]
pub fn DialogDescription(props: DialogDescriptionProps) -> Element {
	rsx! {
		maestro_headless::dialog::DialogDescription {
			class: tw_merge!("text-muted-foreground text-sm", props.class.clone()),
			extra_attributes: props.attributes,
			{props.children}
		}
	}
}

#[derive(Clone, PartialEq, Props)]
pub struct DialogCloseProps {
	#[props(default = String::new())]
	pub class: String,
	#[props(default = ReadOnlySignal::new(Signal::new(ButtonVariant::Primary)))]
	pub variant: ReadOnlySignal<ButtonVariant>,
	#[props(default = ReadOnlySignal::new(Signal::new(ButtonRound::Md)))]
	pub round: ReadOnlySignal<ButtonRound>,
	#[props(default = ReadOnlySignal::new(Signal::new(ButtonSize::Md)))]
	pub size: ReadOnlySignal<ButtonSize>,
	pub children: Element,
	#[props(extends = GlobalAttributes, extends = button)]
	pub attributes: Vec<Attribute>,
}

#[component]
pub fn DialogClose(props: DialogCloseProps) -> Element {
	let class = ButtonClass { variant: *props.variant.read(), size: *props.size.read(), round: *props.round.read() }.with_class(props.class.clone());
	rsx! {
		maestro_headless::dialog::DialogClose { class, extra_attributes: props.attributes, {props.children} }
	}
}
