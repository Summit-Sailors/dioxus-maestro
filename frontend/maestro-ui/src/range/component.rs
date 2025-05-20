use {crate::shared::EOrientation, dioxus::prelude::*, tailwind_fuse::*};

#[derive(Clone, PartialEq, Props)]
pub struct RangeProps {
	#[props(optional, default = ReadOnlySignal::new(Signal::new(None)))]
	pub value: ReadOnlySignal<Option<Vec<f32>>>,
	#[props(optional, default = Vec::new())]
	pub default_value: Vec<f32>,
	#[props(default = None)]
	pub on_value_change: Option<Callback<Vec<f32>>>,

	#[props(optional, default = ReadOnlySignal::new(Signal::new(false)))]
	pub disabled: ReadOnlySignal<bool>,
	#[props(default = false)]
	pub required: bool,
	#[props(optional, default = ReadOnlySignal::new(Signal::new(EOrientation::Horizontal)))]
	pub orientation: ReadOnlySignal<EOrientation>,
	#[props(optional, default = 0.0)]
	pub min: f32,
	#[props(optional, default = 100.0)]
	pub max: f32,
	#[props(optional, default = 1.0)]
	pub step: f32,
	#[props(optional, default = 0.0)]
	pub min_steps_between_thumbs: f32,
	#[props(default = String::new())]
	pub class: String,

	#[props(extends = GlobalAttributes, extends = div)]
	pub attributes: Vec<Attribute>,
	pub children: Element,
}

#[component]
pub fn Range(props: RangeProps) -> Element {
	let RangeProps {
		value,
		default_value,
		on_value_change,
		disabled,
		required,
		orientation,
		min,
		max,
		step,
		min_steps_between_thumbs,
		class,
		attributes,
		children,
	} = props;

	rsx! {
		maestro_headless::range::RangeRoot {
			value,
			default_value,
			on_value_change,
			disabled,
			required,
			orientation,
			min,
			max,
			step,
			min_steps_between_thumbs,
			class: tw_merge!(
					"relative flex w-full items-center select-none data-[disabled=true]:opacity-50 data-[disabled=true]:cursor-auto data-[disabled=true]:pointer-events-none data-[orientation=vertical]:h-full data-[orientation=vertical]:min-h-52 data-[orientation=vertical]:w-auto data-[orientation=vertical]:flex-col",
					class
			),
			extra_attributes: attributes,
			maestro_headless::range::RangeTrack { class: "relative flex-grow bg-muted overflow-hidden rounded-full data-[orientation=horizontal]:h-1 data-[orientation=horizontal]:w-full data-[orientation=vertical]:w-1 data-[orientation=vertical]:h-full data-[disabled=true]:cursor-auto data-[disabled=true]:pointer-events-none",
				maestro_headless::range::Range { class: "flex-grow bg-primary rounded-full data-[orientation=horizontal]:h-full data-[orientation=vertical]:w-full data-[disabled=true]:cursor-auto data-[disabled=true]:pointer-events-none" }
			}
			{children}
		}
	}
}

#[derive(Clone, PartialEq, Props)]
pub struct RangeThumbProps {
	#[props(default = String::new())]
	pub class: String,
	#[props(extends = div, extends = GlobalAttributes)]
	pub attributes: Vec<Attribute>,
	pub children: Element,
}

#[component]
pub fn RangeThumb(props: RangeThumbProps) -> Element {
	rsx! {
		maestro_headless::range::RangeThumb {
			class: tw_merge!(
					"shrink-0 transition-all w-6 h-6 border-primary border rounded-full bg-background flex items-center justify-center text-foreground-300 text-xs cursor-pointer transition-colors focus-visible:outline-none focus-visible:ring-1 hover:ring-1 ring-ring focus-visible:ring-offset-2 focus-visible:ring-offset-background hover:ring-offset-2 hover:ring-offset-background data-[disabled=true]:cursor-auto data-[disabled=true]:pointer-events-none",
					props.class.clone()
			),
			extra_attributes: props.attributes.clone(),
			{props.children}
		}
	}
}
