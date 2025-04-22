use {
	crate::radio::{RadioClass, RadioIndicatorClass, RadioIndicatorVariant, RadioSize},
	dioxus::prelude::*,
	tailwind_fuse::*,
};

#[derive(Props, PartialEq, Debug, Clone)]
pub struct RadioProps {
	#[props(default = String::new())]
	pub class: String,
	#[props(default = ReadOnlySignal::new(Signal::new(RadioIndicatorVariant::Circle)))]
	pub indicator_variant: ReadOnlySignal<RadioIndicatorVariant>,
	#[props(default = ReadOnlySignal::new(Signal::new(RadioSize::Md)))]
	pub size: ReadOnlySignal<RadioSize>,
	#[props(default = String::new())]
	pub indicator_class: String,

	pub value: ReadOnlySignal<String>,
	#[props(optional, default = ReadOnlySignal::new(Signal::new(None)))]
	pub checked: ReadOnlySignal<Option<bool>>,
	#[props(optional, default = false)]
	pub default_checked: bool,
	#[props(default = None)]
	pub on_change: Option<Callback<bool>>,

	#[props(optional, default = ReadOnlySignal::new(Signal::new(false)))]
	pub disabled: ReadOnlySignal<bool>,
	#[props(default = false)]
	pub required: bool,

	#[props(extends = GlobalAttributes, extends = button)]
	pub attributes: Vec<Attribute>,
	#[props(default = Vec::new())]
	pub extra_attributes: Vec<Attribute>,
}

#[component]
pub fn Radio(props: RadioProps) -> Element {
	let RadioProps {
		class,
		indicator_variant,
		size,
		indicator_class,
		disabled,
		value,
		checked,
		default_checked,
		attributes,
		extra_attributes,
		on_change,
		required,
	} = props;

	let class = RadioClass { size: size() }.with_class(class.clone());
	let indicator_class = RadioIndicatorClass { variant: indicator_variant() }.with_class(indicator_class.clone());

	let mut attrs = attributes.clone();
	attrs.extend(extra_attributes);

	rsx! {
		maestro_headless::radio::Radio {
			disabled,
			value,
			checked,
			default_checked,
			on_change,
			required,
			extra_attributes: attrs.clone(),
			class,
			maestro_headless::radio::RadioIndicator {
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
