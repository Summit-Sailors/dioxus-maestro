use {dioxus::prelude::*, tailwind_fuse::*};

#[derive(TwClass)]
#[tw(
	class = "inline-flex items-center justify-center gap-2 whitespace-nowrap rounded-md text-sm font-medium ring-offset-background transition-colors focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-ring focus-visible:ring-offset-2 disabled:pointer-events-none disabled:opacity-50 [&_svg]:pointer-events-none [&_svg]:size-4 [&_svg]:shrink-0"
)]
pub struct ButtonClass {
	pub variant: ButtonVariant,
	pub size: ButtonSize,
}

#[derive(PartialEq, TwVariant)]
pub enum ButtonVariant {
	#[tw(default, class = "bg-primary text-primary-foreground hover:bg-primary/90")]
	Default,
	#[tw(class = "bg-destructive text-destructive-foreground hover:bg-destructive/90")]
	Destructive,
	#[tw(class = "border border-primary text-primary-foreground bg-transparent hover:border-primary/90")]
	Outline,
	#[tw(class = "bg-secondary text-secondary-foreground hover:bg-secondary/90")]
	Secondary,
	#[tw(class = "hover:bg-accent hover:text-accent-foreground")]
	Ghost,
	#[tw(class = "text-primary-foreground underline-offset-4 hover:underline")]
	Link,
	#[tw(class = "w-fit h-fit !p-0 text-primary-foreground hover:text-primary-foreground/90")]
	Icon,
	#[tw(class = "w-fit h-fit !p-0 border border-primary text-primary-foreground bg-transparent hover:border-primary/90")]
	Rounded,
}

#[derive(PartialEq, TwVariant)]
pub enum ButtonSize {
	#[tw(default, class = "h-10 px-4 py-2")]
	Default,
	#[tw(class = "px-3 rounded-md h-9")]
	Sm,
	#[tw(class = "px-8 rounded-md h-11")]
	Lg,
	#[tw(class = "w-10 h-10 !p-0")]
	IconLg,
	#[tw(class = "w-8 h-8 !p-0")]
	IconMd,
	#[tw(class = "w-6 h-6 !p-0")]
	IconSm,
}

#[derive(PartialEq, Clone, Copy)]
pub enum ButtonType {
	Submit,
	Button,
	Reset,
}

impl ButtonType {
	fn as_str(&self) -> &'static str {
		match self {
			ButtonType::Submit => "submit",
			ButtonType::Reset => "reset",
			ButtonType::Button => "button",
		}
	}
}

#[derive(PartialEq, Clone, Props)]
pub struct ButtonProps {
	#[props(default = ButtonVariant::Default)]
	pub variant: ButtonVariant,
	#[props(default = ButtonSize::Default)]
	pub size: ButtonSize,
	#[props(default = false)]
	pub disabled: bool,
	pub on_click: Option<EventHandler>,
	pub id: Option<String>,
	pub class: Option<String>,
	pub style: Option<String>,
	pub children: Element,
	#[props(default = ButtonType::Submit)]
	pub button_type: ButtonType,
	#[props(default = true)]
	pub prevent_default: bool,
	#[props(default = false)]
	pub stop_propagation: bool,
}

#[component]
pub fn Button(props: ButtonProps) -> Element {
	let class = ButtonClass { variant: props.variant, size: props.size }.with_class(props.class.unwrap_or_default());

	rsx! {
		button {
			id: props.id.unwrap_or_default(),
			class,
			style: props.style.unwrap_or_default(),
			disabled: props.disabled,
			onclick: move |event| {
					if props.prevent_default {
							event.prevent_default();
					}
					if props.stop_propagation {
							event.stop_propagation();
					}
					props.on_click.unwrap_or_default().call(())
			},
			r#type: props.button_type.as_str(),
			{props.children}
		}
	}
}
