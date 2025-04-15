use {dioxus::prelude::*, tailwind_fuse::*};

#[derive(TwClass)]
#[tw(
	class = "inline-flex w-fit px-3 py-2 items-center justify-center gap-2 whitespace-nowrap font-medium text-foreground transition-colors ring-ring ring-offset-background focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-offset-2 disabled:pointer-events-none disabled:opacity-50 [&_svg]:pointer-events-none [&_svg]:size-4 [&_svg]:shrink-0"
)]
pub struct ButtonClass {
	pub variant: ButtonVariant,
	pub size: ButtonSize,
	pub round: ButtonRound,
}

#[derive(PartialEq, TwVariant)]
pub enum ButtonRound {
	#[tw(class = "rounded-xs")]
	Xs,
	#[tw(class = "rounded-sm")]
	Sm,
	#[tw(default, class = "rounded-md")]
	Md,
	#[tw(class = "rounded-lg")]
	Lg,
	#[tw(class = "rounded-full")]
	Full,
}

#[derive(PartialEq, TwVariant)]
pub enum ButtonVariant {
	#[tw(default, class = "text-primary-foreground bg-primary-background hover:bg-primary-background/90")]
	Primary,
	#[tw(class = "text-secondary-foreground bg-secondary-background hover:bg-secondary-background/90")]
	Secondary,
	#[tw(class = "bg-background border border-border hover:bg-accent")]
	Outline,
	#[tw(class = "hover:bg-accent")]
	Ghost,
	#[tw(class = "text-link underline-offset-3 hover:underline")]
	Link,
	#[tw(class = "text-danger-foreground bg-danger-background hover:bg-danger-background/90")]
	Danger,
	#[tw(class = "text-danger-foreground bg-danger-background hover:bg-danger-background/90")]
	Muted,
}

#[derive(PartialEq, TwVariant)]
pub enum ButtonSize {
	#[tw(class = "h-8")]
	Xs,
	#[tw(class = "h-9")]
	Sm,
	#[tw(default, class = "h-10")]
	Md,
	#[tw(class = "h-11")]
	Lg,
	#[tw(class = "h-12")]
	Xl,
}

#[derive(Clone, PartialEq, Props)]
pub struct ButtonProps {
	#[props(default = ButtonVariant::Primary)]
	pub variant: ButtonVariant,
	#[props(default = ButtonRound::Md)]
	pub round: ButtonRound,
	#[props(default = ButtonSize::Md)]
	pub size: ButtonSize,
	#[props(default = ReadOnlySignal::new(Signal::new(String::new())))]
	pub class: ReadOnlySignal<String>,

	#[props(default = ReadOnlySignal::new(Signal::new(false)))]
	pub pending: ReadOnlySignal<bool>,
	#[props(default = ReadOnlySignal::new(Signal::new(false)))]
	pub disabled: ReadOnlySignal<bool>,
	#[props(default = None)]
	pub onclick: Option<EventHandler<Event<MouseData>>>,
	#[props(default = None)]
	pub onkeydown: Option<EventHandler<Event<KeyboardData>>>,
	#[props(default = None)]
	pub onkeyup: Option<EventHandler<Event<KeyboardData>>>,
	#[props(default = None)]
	pub onfocus: Option<EventHandler<Event<FocusData>>>,
	#[props(default = None)]
	pub onblur: Option<EventHandler<Event<FocusData>>>,
	#[props(default = None)]
	pub onmousedown: Option<EventHandler<Event<MouseData>>>,
	#[props(default = None)]
	pub onmouseup: Option<EventHandler<Event<MouseData>>>,
	#[props(default = None)]
	pub onmouseenter: Option<EventHandler<Event<MouseData>>>,
	#[props(default = None)]
	pub onmouseleave: Option<EventHandler<Event<MouseData>>>,
	#[props(optional, default = None)]
	pub onmounted: Option<EventHandler<Event<MountedData>>>,

	#[props(extends = GlobalAttributes, extends = button)]
	pub attributes: Vec<Attribute>,
	#[props(default = None)]
	pub children: Element,
}

#[component]
pub fn Button(props: ButtonProps) -> Element {
	let ButtonProps {
		variant,
		round,
		size,
		class,
		pending,
		disabled,
		onclick,
		onkeydown,
		onkeyup,
		onfocus,
		onblur,
		onmousedown,
		onmouseup,
		onmouseenter,
		onmouseleave,
		onmounted,
		attributes,
		children,
	} = props;
	let class = ButtonClass { variant, size, round }.with_class(class().clone());

	rsx! {
		maestro_headless::button::Button {
			class,
			pending,
			disabled,
			onclick,
			onkeydown,
			onkeyup,
			onfocus,
			onblur,
			onmousedown,
			onmouseup,
			onmouseenter,
			onmouseleave,
			onmounted,
			extra_attributes: attributes,
			{children}
		}
	}
}
