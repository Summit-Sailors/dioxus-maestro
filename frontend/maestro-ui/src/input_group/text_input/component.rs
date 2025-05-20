use {dioxus::prelude::*, tailwind_fuse::*};

#[derive(Clone, PartialEq, Props)]
pub struct TextInputProps {
	#[props(default = String::new())]
	pub class: String,
	#[props(optional, default = ReadOnlySignal::new(Signal::new(None)))]
	pub value: ReadOnlySignal<Option<String>>,
	#[props(optional, default = String::new())]
	pub default_value: String,
	#[props(optional)]
	pub on_value_change: Option<Callback<String>>,

	#[props(default = 0)]
	pub debounce_ms: u32,
	#[props(default = ReadOnlySignal::new(Signal::new(false)))]
	pub disabled: ReadOnlySignal<bool>,
	#[props(default = ReadOnlySignal::new(Signal::new(false)))]
	pub invalid: ReadOnlySignal<bool>,

	#[props(default = None)]
	pub oninput: Option<EventHandler<Event<FormData>>>,
	#[props(default = None)]
	pub onchange: Option<EventHandler<Event<FormData>>>,
	#[props(default = None)]
	pub onblur: Option<EventHandler<Event<FocusData>>>,
	#[props(default = None)]
	pub onfocus: Option<EventHandler<Event<FocusData>>>,

	#[props(extends = input, extends = GlobalAttributes)]
	pub attributes: Vec<Attribute>,
}

#[component]
pub fn TextInput(props: TextInputProps) -> Element {
	let TextInputProps { class, value, default_value, on_value_change, debounce_ms, disabled, invalid, attributes, oninput, onchange, onblur, onfocus } = props;

	rsx! {
		maestro_headless::text_input::TextInput {
			class: tw_merge!(
					"file:text-foreground placeholder:text-muted-foreground selection:bg-primary selection:text-primary-foreground border-input flex h-9 w-full min-w-0 rounded-md border bg-transparent px-3 py-1 text-base shadow-xs transition-[color,box-shadow] outline-none file:inline-flex file:h-7 file:border-0 file:bg-transparent file:text-sm file:font-medium disabled:pointer-events-none disabled:cursor-not-allowed disabled:opacity-50 md:text-sm focus-visible:ring-ring focus-visible:ring-2 focus-visible:ring-offset-2 focus-visible:ring-offset-background aria-invalid:ring-destructive/20 aria-invalid:border-destructive",
					class.clone()
			),
			value,
			default_value,
			on_value_change,
			debounce_ms,
			disabled,
			invalid,
			extra_attributes: attributes.clone(),
			oninput,
			onchange,
			onfocus,
			onblur,
		}
	}
}
