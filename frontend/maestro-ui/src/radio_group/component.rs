use {
	crate::{
		radio::{Radio, RadioClass, RadioIndicatorClass, RadioIndicatorVariant, RadioProps},
		shared::EOrientation,
	},
	dioxus::prelude::*,
	tailwind_fuse::*,
};

#[derive(Props, Clone, PartialEq)]
pub struct RadioGroupProps {
	#[props(default = ReadOnlySignal::new(Signal::new(String::new())))]
	pub class: ReadOnlySignal<String>,
	#[props(optional, default = ReadOnlySignal::new(Signal::new(None)))]
	pub value: ReadOnlySignal<Option<String>>,
	#[props(optional, default = None)]
	default_value: Option<String>,
	#[props(optional)]
	pub on_value_change: Option<Callback<String>>,
	#[props(optional, default = ReadOnlySignal::new(Signal::new(false)))]
	pub disabled: ReadOnlySignal<bool>,
	#[props(optional, default = false)]
	pub required: bool,
	#[props(optional, default = ReadOnlySignal::new(Signal::new(EOrientation::Horizontal)))]
	pub orientation: ReadOnlySignal<EOrientation>,

	#[props(extends = div, extends = GlobalAttributes)]
	pub attributes: Vec<Attribute>,
	pub children: Element,
}

#[component]
pub fn RadioGroup(props: RadioGroupProps) -> Element {
	let RadioGroupProps { class, value, default_value, on_value_change, disabled, required, orientation, children, attributes } = props;

	let class = tw_merge!("flex data-[orientation=vertical]:flex-col gap-3 data-[orientation=horizontal]:items-center data-[disabled=true]:opacity-50 data-[disabled=true]:*:pointer-events-none data-[disabled=true]:*:cursor-auto", class());

	rsx! {
		maestro_headless::radio_group::RadioGroupRoot {
			class,
			value,
			default_value,
			on_value_change,
			disabled,
			required,
			orientation,
			extra_attributes: attributes,
			{children}
		}
	}
}

#[component]
pub fn RadioGroupItem(props: RadioProps) -> Element {
	let RadioProps {
		indicator_class,
		indicator_variant,
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

	let class = RadioClass { size: size() }.with_class(class.clone());
	let indicator_class = RadioIndicatorClass { variant: indicator_variant() }.with_class(indicator_class.clone());

	rsx! {
		maestro_headless::radio_group::RadioGroupItem { value, disabled, class, extra_attributes: attrs,
			maestro_headless::radio_group::RadioGroupIndicator {
				if indicator_variant() == RadioIndicatorVariant::Circle {
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
