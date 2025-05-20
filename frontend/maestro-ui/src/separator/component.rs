use {crate::shared::EOrientation, dioxus::prelude::*, tailwind_fuse::*};

#[derive(Clone, PartialEq, Props)]
pub struct SeparatorProps {
	#[props(default = String::new())]
	pub class: String,
	#[props(default = ReadOnlySignal::new(Signal::new(EOrientation::Horizontal)))]
	orientation: ReadOnlySignal<EOrientation>,
	#[props(extends = div, extends = GlobalAttributes)]
	pub attributes: Vec<Attribute>,
}

#[component]
pub fn Separator(props: SeparatorProps) -> Element {
	let SeparatorProps { orientation, attributes, class } = props;

	rsx! {
		maestro_headless::separator::Separator {
			extra_attributes: attributes,
			orientation,
			class: tw_merge!(
					"bg-border shrink-0 data-[orientation=horizontal]:h-px data-[orientation=horizontal]:w-full data-[orientation=vertical]:h-full data-[orientation=vertical]:w-px",
					class
			),
		}
	}
}
