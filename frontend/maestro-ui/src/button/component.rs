use {
	crate::shared::{EClass, ERound, ESize, EVariant},
	dioxus::prelude::*,
	tailwind_fuse::*,
};

#[derive(Clone, PartialEq, Props)]
pub struct ButtonProps {
	#[props(default = ReadOnlySignal::new(Signal::new(EVariant::Primary)))]
	pub variant: ReadOnlySignal<EVariant>,
	#[props(default = ReadOnlySignal::new(Signal::new(ERound::Md)))]
	pub round: ReadOnlySignal<ERound>,
	#[props(default = ReadOnlySignal::new(Signal::new(ESize::Md)))]
	pub size: ReadOnlySignal<ESize>,
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
	#[props(default = Vec::new())]
	pub extra_attributes: Vec<Attribute>,
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
		extra_attributes,
		children,
	} = props;
	let class = EClass { variant: variant(), size: size(), round: round() }.with_class(class().clone());

	let mut attrs = attributes.clone();
	attrs.extend(extra_attributes);

	rsx! {
		maestro_headless::button::Button {
			class,
			pending: pending(),
			disabled: disabled(),
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
			extra_attributes: attrs,
			{children}
		}
	}
}
