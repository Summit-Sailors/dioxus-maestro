use {
	super::{AvatarClass, AvatarSize},
	dioxus::prelude::*,
	tailwind_fuse::{tw_merge, IntoTailwindClass},
};

#[derive(Clone, PartialEq, Props)]
pub struct AvatarProps {
	#[props(default = String::new())]
	pub class: String,
	#[props(default = ReadOnlySignal::new(Signal::new(AvatarSize::Md)))]
	pub size: ReadOnlySignal<AvatarSize>,
	#[props(extends = div, extends = GlobalAttributes)]
	pub attributes: Vec<Attribute>,
	pub children: Element,
}

#[component]
pub fn Avatar(props: AvatarProps) -> Element {
	let AvatarProps { class, size, attributes, children } = props;
	let class = AvatarClass { size: size() }.with_class(class.clone());

	rsx! {
		maestro_headless::avatar::AvatarRoot { class, extra_attributes: attributes, {children} }
	}
}

#[derive(Clone, PartialEq, Props)]
pub struct AvatarImageProps {
	#[props(default = String::new())]
	pub class: String,
	#[props(default = ReadOnlySignal::new(Signal::new(String::new())))]
	pub src: ReadOnlySignal<String>,
	#[props(extends = GlobalAttributes, extends = img)]
	pub attributes: Vec<Attribute>,
}

#[component]
pub fn AvatarImage(props: AvatarImageProps) -> Element {
	rsx! {
		maestro_headless::avatar::AvatarImage {
			class: tw_merge!("w-full h-full object-cover", props.class.clone()),
			extra_attributes: props.attributes,
			src: props.src,
		}
	}
}

#[derive(Clone, PartialEq, Props)]
pub struct AvatarFallbackProps {
	#[props(default = String::new())]
	pub class: String,
	#[props(default = None)]
	pub delay_ms: Option<u32>,

	#[props(extends = GlobalAttributes, extends = span)]
	pub attributes: Vec<Attribute>,
	#[props(default = None)]
	pub children: Element,
}

#[component]
pub fn AvatarFallback(props: AvatarFallbackProps) -> Element {
	let AvatarFallbackProps { class, delay_ms, attributes, children } = props;

	rsx! {
		maestro_headless::avatar::AvatarFallback {
			class: tw_merge!("text-sm font-medium text-muted-forground", class.clone()),
			delay_ms,
			extra_attributes: attributes.clone(),
			{children}
		}
	}
}
