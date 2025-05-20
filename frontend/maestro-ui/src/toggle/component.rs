use {
	crate::toggle::{ToggleClass, ToggleRound, ToggleSize, ToggleVariant},
	dioxus::prelude::*,
	tailwind_fuse::*,
};

#[derive(Clone, Debug, PartialEq, Props)]
pub struct ToggleProps {
	#[props(default = String::new())]
	pub class: String,
	#[props(default = ReadOnlySignal::new(Signal::new(ToggleVariant::Outline)))]
	pub variant: ReadOnlySignal<ToggleVariant>,
	#[props(default = ReadOnlySignal::new(Signal::new(ToggleRound::Md)))]
	pub round: ReadOnlySignal<ToggleRound>,
	#[props(default = ReadOnlySignal::new(Signal::new(ToggleSize::Md)))]
	pub size: ReadOnlySignal<ToggleSize>,
	pub value: ReadOnlySignal<String>,
	#[props(default = ReadOnlySignal::new(Signal::new(None)))]
	pub pressed: ReadOnlySignal<Option<bool>>,
	#[props(default = false)]
	pub default_pressed: bool,
	#[props(default = None)]
	pub on_toggle_change: Option<Callback<bool>>,
	#[props(optional, default = ReadOnlySignal::new(Signal::new(false)))]
	pub disabled: ReadOnlySignal<bool>,

	#[props(extends = GlobalAttributes, extends = button)]
	pub attributes: Vec<Attribute>,
	#[props(default = None)]
	pub children: Option<Element>,
}

#[component]
pub fn Toggle(props: ToggleProps) -> Element {
	let ToggleProps { class, variant, size, round, value, disabled, attributes, on_toggle_change, pressed, default_pressed, children } = props;

	let class = ToggleClass { variant: variant(), size: size(), round: round() }.with_class(class.clone());

	rsx! {
		maestro_headless::toggle::Toggle {
			class,
			value,
			disabled,
			extra_attributes: attributes.clone(),
			on_toggle_change,
			pressed,
			default_pressed,
			if children.is_some() {
				{children}
			}
		}
	}
}
