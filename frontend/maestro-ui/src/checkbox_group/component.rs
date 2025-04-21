use {
	crate::checkbox::{Checkbox, CheckboxProps},
	dioxus::prelude::*,
	maestro_headless::shared::EOrientation,
	tailwind_fuse::*,
};

#[derive(Clone, PartialEq, Props)]
pub struct CheckboxGroupProps {
	#[props(default = ReadOnlySignal::new(Signal::new(String::new())))]
	pub class: ReadOnlySignal<String>,

	#[props(optional, default = ReadOnlySignal::new(Signal::new(None)))]
	pub value: ReadOnlySignal<Option<Vec<String>>>,
	#[props(optional, default = None)]
	default_value: Option<Vec<String>>,
	#[props(optional)]
	pub on_value_change: Option<Callback<Vec<String>>>,
	#[props(optional, default = ReadOnlySignal::new(Signal::new(false)))]
	pub disabled: ReadOnlySignal<bool>,
	#[props(optional, default = ReadOnlySignal::new(Signal::new(false)))]
	pub required: ReadOnlySignal<bool>,
	#[props(optional, default = ReadOnlySignal::new(Signal::new(EOrientation::Horizontal)))]
	pub orientation: ReadOnlySignal<EOrientation>,

	#[props(extends = div, extends = GlobalAttributes)]
	pub attributes: Vec<Attribute>,
	#[props(default = Vec::new())]
	pub extra_attributes: Vec<Attribute>,
	pub children: Element,
}

#[component]
pub fn CheckboxGroup(props: CheckboxGroupProps) -> Element {
	let CheckboxGroupProps { class, value, default_value, on_value_change, disabled, required, orientation, attributes, extra_attributes, children } = props;
	let class = tw_merge!("flex data-[orientation=vertical]:flex-col gap-3 data-[orientation=horizontal]:items-center", class());

	rsx! {
		maestro_headless::checkbox_group::CheckboxGroup {
			class,
			disabled,
			on_value_change,
			value,
			default_value,
			required,
			orientation,
			extra_attributes: attributes,
			{children}
		}
	}
}

#[component]
pub fn CheckboxGroupItem(props: CheckboxProps) -> Element {
	let CheckboxProps {
		indicator_class,
		indicator_variant,
		round,
		size,
		class,
		checked,
		default_checked,
		on_change,
		value,
		disabled,
		required,
		attributes,
		extra_attributes,
	} = props;
	let mut attrs = attributes.clone();
	attrs.extend(extra_attributes);

	rsx! {
		Checkbox {
			indicator_class,
			indicator_variant,
			round,
			size,
			class,
			checked,
			default_checked,
			on_change,
			value,
			disabled,
			required,
			extra_attributes: attrs,
		}
	}
}
