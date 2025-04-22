use dioxus::prelude::*;
use tailwind_fuse::*;

#[derive(Debug, Clone, PartialEq, Props)]
pub struct RangeProps {
	pub oninput: Option<EventHandler<Event<FormData>>>,
	pub onfocus: Option<EventHandler<Event<FocusData>>>,
	pub onblur: Option<EventHandler<Event<FocusData>>>,
	#[props(default = String::new())]
	pub thumb_class: String,
	#[props(default = String::new())]
	pub track_class: String,
	#[props(default = String::new())]
	pub wrapper_class: String,
	#[props(default = String::new())]
	pub label_class: String,
	#[props(default = String::new())]
	pub value_class: String,
	pub style: Option<String>,
	pub value: i32,
	#[props(default = String::new())]
	pub label: String,
	#[props(extends = GlobalAttributes, extends = input)]
	pub attributes: Vec<Attribute>,
}

// TO DO: add also multi range (min-max inputs)

#[component]
pub fn Range(props: RangeProps) -> Element {
	rsx! {
		div {
			class: tw_merge!(
					"flex flex-col gap-2 w-full relative maestro-range__wrapper", & props
					.wrapper_class
			),
			if !props.label.is_empty() {
				span { class: tw_merge!("text-[color:var(--text-color)]400 maestro-range__label", & props.label_class),
					{props.label}
				}
			}
			input {
				r#type: "range",
				value: props.value,
				oninput: move |e| props.oninput.unwrap_or_default().call(e),
				onfocus: move |e| props.onfocus.unwrap_or_default().call(e),
				onblur: move |e| props.onblur.unwrap_or_default().call(e),
				class: tw_merge!(
						"maestro-range__input",
						"appearance-none bg-transparent focus-visible:outline-none [&::-ms-track]:bg-transparent [&::-ms-track]:border-transparent [&::-ms-track]:text-transparent [&::-webkit-slider-thumb]:appearance-none",
						"[&::-webkit-slider-thumb]:cursor-pointer [&::-webkit-slider-thumb]:ring-2 [&::-webkit-slider-thumb]:ring-offset-1 [&::-webkit-slider-thumb]:ring-gray-900 [&::-webkit-slider-thumb]:h-4 [&::-webkit-slider-thumb]:w-4",
						"[&::-webkit-slider-thumb]:rounded-full [&::-webkit-slider-thumb]:bg-gray-800 [&::-webkit-slider-thumb]:-mt-2 [&::-webkit-slider-thumb]:z-10 [&::-webkit-slider-thumb]:transition-all [&::-webkit-slider-thumb]:ease-linear [&::-webkit-slider-thumb]:hover:ring-gray-900",
						"[&::-webkit-slider-thumb]:hover:bg-gray-900 [&::-webkit-slider-thumb]:focus-visible:ring-gray-600 [&::-webkit-slider-thumb]:disabled:pointer-events-none [&::-webkit-slider-thumb]:disabled:opacity-50",
						"[&::-moz-range-thumb]:cursor-pointer [&::-moz-range-thumb]:ring-2 [&::-moz-range-thumb]:ring-offset-1 [&::-moz-range-thumb]:ring-gray-900 [&::-moz-range-thumb]:h-4 [&::-moz-range-thumb]:w-4 [&::-moz-range-thumb]:rounded-full",
						"[&::-moz-range-thumb]:bg-gray-800 [&::-moz-range-thumb]:-mt-2 [&::-moz-range-thumb]:z-10 [&::-moz-range-thumb]:transition-all [&::-moz-range-thumb]:ease-linear",
						"[&::-moz-range-thumb]:hover:ring-gray-900 [&::-moz-range-thumb]:hover:bg-gray-900 [&::-moz-range-thumb]:focus-visible:ring-gray-600 [&::-moz-range-thumb]:disabled:pointer-events-none [&::-moz-range-thumb]:disabled:opacity-50",
						"[&::-ms-thumb]:cursor-pointer [&::-ms-thumb]:ring-2 [&::-ms-thumb]:ring-offset-1 [&::-ms-thumb]:ring-gray-900 [&::-ms-thumb]:h-4 [&::-ms-thumb]:w-4 [&::-ms-thumb]:rounded-full [&::-ms-thumb]:bg-gray-800",
						"[&::-ms-thumb]:-mt-2 [&::-ms-thumb]:z-10 [&::-ms-thumb]:transition-all [&::-ms-thumb]:ease-linear",
						"[&::-ms-thumb]:hover:ring-gray-900 [&::-ms-thumb]:hover:bg-gray-900 [&::-ms-thumb]:focus-visible:ring-gray-600 [&::-ms-thumb]:disabled:pointer-events-none [&::-ms-thumb]:disabled:opacity-50",
						"[&::-webkit-slider-runnable-track]:w-full [&::-webkit-slider-runnable-track]:h-0.5 [&::-webkit-slider-runnable-track]:cursor-pointer [&::-webkit-slider-runnable-track]:bg-gray-500 [&::-webkit-slider-runnable-track]:rounded [&::-webkit-slider-runnable-track]:hover:bg-gray-700",
						"[&::-webkit-slider-runnable-track]:focus-visible:bg-gray-700 [&::-webkit-slider-runnable-track]:disabled:opacity-50 [&::-webkit-slider-runnable-track]:disabled:pointer-events-none",
						"[&::-moz-range-track]:w-full [&::-moz-range-track]:h-0.5 [&::-moz-range-track]:cursor-pointer [&::-moz-range-track]:bg-gray-500 [&::-moz-range-track]:rounded [&::-moz-range-track]:hover:bg-gray-700",
						"[&::-moz-range-track]:focus-visible:bg-gray-700 [&::-moz-range-track]:disabled:opacity-50 [&::-moz-range-track]:disabled:pointer-events-none",
						"[&::-ms-track]:w-full [&::-ms-track]:h-0.5 [&::-ms-track]:cursor-pointer [&::-ms-track]:bg-gray-500 [&::-ms-track]:rounded [&::-ms-track]:hover:bg-gray-700 [&::-ms-track]:focus-visible:bg-gray-700 [&::-ms-track]:disabled:opacity-50 [&::-ms-track]:disabled:pointer-events-none",
						& props.thumb_class, & props.track_class
				),
				..props.attributes,
			}
			span {
				class: tw_merge!(
						"text-xs text-left text-[color:var(--text-color)]400 maestro-range-value", & props.value_class
				),
				"{props.value}"
			}
		}
	}
}
