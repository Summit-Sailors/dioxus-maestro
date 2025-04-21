use {
	crate::checkbox::{CheckboxClass, CheckboxIndicatorClass, CheckboxIndicatorVariant, CheckboxRound, CheckboxSize},
	dioxus::prelude::*,
	tailwind_fuse::*,
};

#[derive(Clone, PartialEq, Props)]
pub struct CheckboxProps {
	#[props(default = ReadOnlySignal::new(Signal::new(CheckboxIndicatorVariant::Tick)))]
	pub indicator_variant: ReadOnlySignal<CheckboxIndicatorVariant>,
	#[props(default = ReadOnlySignal::new(Signal::new(CheckboxRound::Md)))]
	pub round: ReadOnlySignal<CheckboxRound>,
	#[props(default = ReadOnlySignal::new(Signal::new(CheckboxSize::Md)))]
	pub size: ReadOnlySignal<CheckboxSize>,
	#[props(default = ReadOnlySignal::new(Signal::new(String::new())))]
	pub class: ReadOnlySignal<String>,
	#[props(default = ReadOnlySignal::new(Signal::new(String::new())))]
	pub indicator_class: ReadOnlySignal<String>,

	#[props(optional, default = ReadOnlySignal::new(Signal::new(None)))]
	pub checked: ReadOnlySignal<Option<bool>>,
	#[props(optional, default = false)]
	pub default_checked: bool,
	#[props(default = None)]
	pub on_change: Option<Callback<bool>>,

	pub value: ReadOnlySignal<String>,
	#[props(optional, default = ReadOnlySignal::new(Signal::new(false)))]
	pub disabled: ReadOnlySignal<bool>,
	#[props(default = ReadOnlySignal::new(Signal::new(false)))]
	pub required: ReadOnlySignal<bool>,

	#[props(extends = GlobalAttributes, extends = button)]
	pub attributes: Vec<Attribute>,
	#[props(default = Vec::new())]
	pub extra_attributes: Vec<Attribute>,
}

#[component]
pub fn Checkbox(props: CheckboxProps) -> Element {
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
	let class = CheckboxClass { size: size(), round: round() }.with_class(class().clone());
	let indicator_class = CheckboxIndicatorClass { variant: indicator_variant() }.with_class(indicator_class().clone());

	let mut attrs = attributes.clone();
	attrs.extend(extra_attributes);

	rsx! {
		maestro_headless::checkbox::CheckboxRoot {
			class,
			disabled: disabled(),
			on_change,
			value,
			default_checked,
			checked,
			required,
			extra_attributes: attrs,
			maestro_headless::checkbox::CheckboxIndicator {
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
