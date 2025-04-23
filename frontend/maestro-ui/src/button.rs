use dioxus::prelude::*;
use tailwind_fuse::*;

#[derive(TwClass)]
#[tw(class = "px-4 py-3 text-sm font-medium disabled:pointer-events-none disabled:opacity-50 maestro-button")]
pub struct ButtonClass {
	pub variant: ButtonVariant,
	pub size: ButtonSize,
}

#[derive(PartialEq, TwVariant)]
pub enum ButtonVariant {
	#[tw(default, class = "variant-default")]
	Default,
	#[tw(class = "variant-outline")]
	Outline,
	#[tw(class = "variant-ghost")]
	Ghost,
	#[tw(class = "variant-link")]
	Link,
	#[tw(class = "variant-icon")]
	Icon,
}

#[derive(PartialEq, TwVariant)]
pub enum ButtonSize {
	#[tw(default, class = "size-default")]
	Default,
	#[tw(class = "size-sm")]
	Sm,
	#[tw(class = "size-lg")]
	Lg,
	#[tw(class = "size-xl")]
	Xl,
	#[tw(class = "size-icon-xl")]
	IconXl,
	#[tw(class = "size-icon-lg")]
	IconLg,
	#[tw(class = "size-icon-md")]
	IconMd,
	#[tw(class = "size-icon-sm")]
	IconSm,
}

#[derive(Clone, PartialEq, Props)]
pub struct ButtonProps {
	#[props(default = ButtonVariant::Default)]
	pub variant: ButtonVariant,
	#[props(default = ButtonSize::Default)]
	pub size: ButtonSize,
	pub onclick: Option<EventHandler<Event<MouseData>>>,
	#[props(default = String::new())]
	pub class: String,
	pub style: Option<String>,
	pub children: Element,
	#[props(extends = GlobalAttributes, extends = button)]
	pub attributes: Vec<Attribute>,
}

// class may be extended also by using "maestro-button" classname

#[component]
pub fn Button(props: ButtonProps) -> Element {
	let class = ButtonClass { variant: props.variant, size: props.size }.with_class(tw_merge!(props.class.clone(), "maestro-button"));

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
