use {
	crate::{
		shared::EOrientation,
		toggle::ToggleSize,
		toggle_group::{ToggleItemClass, ToggleItemRound, ToggleItemVariant},
	},
	dioxus::prelude::*,
	tailwind_fuse::*,
};

#[derive(Clone, PartialEq, Props)]
pub struct ToggleGroupProps {
	#[props(default = String::new())]
	pub class: String,
	#[props(optional, default = ReadOnlySignal::new(Signal::new(None)))]
	pub value: ReadOnlySignal<Option<String>>,
	#[props(optional, default = None)]
	default_value: Option<String>,
	#[props(optional)]
	pub on_value_chenge: Option<Callback<String>>,

	#[props(optional, default = ReadOnlySignal::new(Signal::new(false)))]
	pub disabled: ReadOnlySignal<bool>,
	#[props(optional, default = ReadOnlySignal::new(Signal::new(EOrientation::Horizontal)))]
	pub orientation: ReadOnlySignal<EOrientation>,

	#[props(extends = div, extends = GlobalAttributes)]
	pub attributes: Vec<Attribute>,
	pub children: Element,
}

#[component]
pub fn ToggleGroup(props: ToggleGroupProps) -> Element {
	let ToggleGroupProps { class, value, default_value, on_value_chenge, disabled, orientation, children, attributes } = props;

	rsx! {
		maestro_headless::toggle_group::ToggleGroupRoot {
			class: tw_merge!(
					"group/toggle-group flex w-fit items-center justify-center data-[orientation=vertical]:flex-col data-[disabled=true]:*:opacity-50 data-[disabled=true]:*:cursor-auto data-[disabled=true]:*:pointer-events-none",
					class.clone()
			),
			value,
			default_value,
			on_value_chenge,
			disabled,
			orientation,
			extra_attributes: attributes.clone(),
			{children}
		}
	}
}

#[derive(Clone, PartialEq, Props)]
pub struct ToggleGroupItemProps {
	#[props(default = String::new())]
	pub class: String,
	#[props(default = ReadOnlySignal::new(Signal::new(ToggleItemVariant::Outline)))]
	pub variant: ReadOnlySignal<ToggleItemVariant>,
	#[props(default = ReadOnlySignal::new(Signal::new(ToggleItemRound::Md)))]
	pub round: ReadOnlySignal<ToggleItemRound>,
	#[props(default = ReadOnlySignal::new(Signal::new(ToggleSize::Md)))]
	pub size: ReadOnlySignal<ToggleSize>,
	pub value: ReadOnlySignal<String>,
	#[props(optional, default = ReadOnlySignal::new(Signal::new(false)))]
	pub disabled: ReadOnlySignal<bool>,

	#[props(extends = button, extends = GlobalAttributes)]
	pub attributes: Vec<Attribute>,
	pub children: Element,
}

#[component]
pub fn ToggleGroupItem(props: ToggleGroupItemProps) -> Element {
	let ToggleGroupItemProps { class, size, variant, round, value, disabled, attributes, children } = props;

	let class = ToggleItemClass { variant: variant(), size: size(), round: round() }.with_class(class.clone());

	rsx! {
		maestro_headless::toggle_group::ToggleGroupItem {
			class,
			value,
			disabled,
			extra_attributes: attributes.clone(),
			{children}
		}
	}
}
