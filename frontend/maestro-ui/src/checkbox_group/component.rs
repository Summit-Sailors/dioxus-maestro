use {
	crate::checkbox::{Checkbox, CheckboxClass, CheckboxIndicatorClass, CheckboxIndicatorVariant, CheckboxProps},
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
	let class = tw_merge!("flex data-[orientation=vertical]:flex-col gap-3 data-[orientation=horizontal]:items-center data-[disabled=true]:opacity-50 data-[disabled=true]:*:pointer-events-none data-[disabled=true]:*:cursor-auto", class());

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

	let class = CheckboxClass { size: size(), round: round() }.with_class(class().clone());
	let indicator_class = CheckboxIndicatorClass { variant: indicator_variant() }.with_class(indicator_class().clone());

	rsx! {
		maestro_headless::checkbox_group::CheckboxGroupItem {
			class,
			value,
			disabled,
			extra_attributes: attrs,
			maestro_headless::checkbox_group::CheckboxGroupIndicator {
				if indicator_variant() == CheckboxIndicatorVariant::Square {
					span { class: indicator_class.clone() }
				} else {
					svg {
						stroke: "currentColor",
						fill: "currentColor",
						stroke_width: "0",
						view_box: "0 0 512 512",
						height: "16px",
						width: "16px",
						xmlns: "http://www.w3.org/2000/svg",
						path {
							fill: "none",
							stroke_linecap: "round",
							stroke_linejoin: "round",
							stroke_width: "32",
							d: "M416 128 192 384l-96-96",
						}
					}
				}
			}
		}
	}
}
