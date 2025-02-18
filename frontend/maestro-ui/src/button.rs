use {dioxus::prelude::*, tailwind_fuse::*};

#[derive(TwClass)]
#[tw(
	class = "inline-flex items-center justify-center gap-2 whitespace-nowrap rounded-md text-sm font-medium ring-offset-transparent transition-colors focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-ring focus-visible:ring-offset-2 disabled:pointer-events-none disabled:opacity-50 [&_svg]:pointer-events-none [&_svg]:size-4 [&_svg]:shrink-0"
)]
pub struct ButtonClass {
	pub variant: ButtonVariant,
	pub size: ButtonSize,
}

#[derive(PartialEq, TwVariant)]
pub enum ButtonVariant {
	#[tw(default, class = "")]
	Default,
	#[tw(class = "text-primary-foreground underline-offset-4 hover:underline")]
	Link,
	#[tw(class = "w-fit h-fit !p-0 text-primary-foreground hover:text-primary-foreground/90")]
	Icon,
	#[tw(class = "w-fit h-fit !p-0 border bg-transparent")]
	Rounded,
}

#[derive(PartialEq, TwVariant)]
pub enum ButtonSize {
	#[tw(default, class = "h-10 px-4 py-2")]
	Default,
	#[tw(class = "px-9 rounded-md h-9")]
	Sm,
	#[tw(class = "px-4 rounded-md h-11")]
	Lg,
	#[tw(class = "px-4 rounded-md h-12")]
	Xl,
	#[tw(class = "w-12 h-12 !p-0")]
	IconXl,
	#[tw(class = "w-11 h-11 !p-0")]
	IconLg,
	#[tw(class = "w-10 h-10 !p-0")]
	IconMd,
	#[tw(class = "w-9 h-9 !p-0")]
	IconSm,
}

#[derive(Clone, PartialEq, Props)]
pub struct ButtonProps {
	#[props(default = ButtonVariant::Default)]
	pub variant: ButtonVariant,
	#[props(default = ButtonSize::Default)]
	pub size: ButtonSize,
	pub onclick: Option<EventHandler<Event<MouseData>>>,
	pub class: Option<String>,
	pub style: Option<String>,
	pub children: Element,
	#[props(extends = GlobalAttributes, extends = button)]
	pub attributes: Vec<Attribute>,
}

#[component]
pub fn Button(props: ButtonProps) -> Element {
	let class = ButtonClass { variant: props.variant, size: props.size }.with_class(props.class.unwrap_or_default());

	rsx! {
		button {
			class,
			style: props.style.unwrap_or_default(),
			onclick: move |event| props.onclick.unwrap_or_default().call(event),
			..props.attributes,
			{props.children}
		}
	}
}
