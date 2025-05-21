use {
	super::{SwitchRound, SwitchSize},
	crate::switch::SwitchClass,
	dioxus::prelude::*,
	tailwind_fuse::*,
};

#[derive(Debug, Clone, PartialEq, Props)]
pub struct SwitchProps {
	#[props(default = String::new())]
	pub class: String,
	#[props(default = String::new())]
	pub indicator_class: String,
	#[props(default = ReadOnlySignal::new(Signal::new(SwitchSize::default())))]
	pub size: ReadOnlySignal<SwitchSize>,
	#[props(default = ReadOnlySignal::new(Signal::new(SwitchRound::default())))]
	pub round: ReadOnlySignal<SwitchRound>,
	#[props(default = ReadOnlySignal::new(Signal::new(None)))]
	pub value: ReadOnlySignal<Option<String>>,

	#[props(default = ReadOnlySignal::new(Signal::new(None)))]
	pub checked: ReadOnlySignal<Option<bool>>,
	#[props(default = false)]
	pub default_checked: bool,
	#[props(default = None)]
	pub on_toggle_change: Option<Callback<bool>>,
	#[props(optional, default = ReadOnlySignal::new(Signal::new(false)))]
	pub disabled: ReadOnlySignal<bool>,
	#[props(optional, default = false)]
	pub required: bool,

	#[props(extends = GlobalAttributes, extends = button)]
	pub attributes: Vec<Attribute>,
}

#[component]
pub fn Switch(props: SwitchProps) -> Element {
	let SwitchProps { class, indicator_class, size, round, value, disabled, required, attributes, on_toggle_change, checked, default_checked } = props;

	let class = SwitchClass { size: size(), round: round() }.with_class(class.clone());
	let indicator_class = tw_merge!(
		"bg-background block ring-0 transition-transform data-[state=checked]:translate-x-[calc(100%-2px)] data-[state=unchecked]:translate-x-0",
		match size() {
			SwitchSize::Sm => "size-4",
			SwitchSize::Md => "size-4.5",
			SwitchSize::Lg => "size-5",
		},
		match round() {
			SwitchRound::Sm => "rounded-xs",
			SwitchRound::Md => "rounded-sm",
			SwitchRound::Lg => "rounded-md",
			SwitchRound::Full => "rounded-full",
		},
		indicator_class.clone()
	);

	rsx! {
		maestro_headless::switch::SwitchRoot {
			value,
			disabled,
			required,
			extra_attributes: attributes.clone(),
			on_toggle_change,
			checked,
			default_checked,
			class,
			maestro_headless::switch::SwitchIndicator { class: indicator_class }
		}
	}
}
