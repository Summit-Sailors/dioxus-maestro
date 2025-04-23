use {dioxus::prelude::*, tailwind_fuse::*};

#[derive(Props, Clone, PartialEq)]
pub struct TextareaProps {
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

	#[props(extends = textarea, extends = GlobalAttributes)]
	pub attributes: Vec<Attribute>,
	pub children: Element,
}

#[component]
pub fn Textarea(props: TextareaProps) -> Element {
	let TextareaProps { class, value, default_value, on_value_change, debounce_ms, disabled, invalid, attributes, children, onchange, oninput } = props;

	rsx! {
		maestro_headless::textarea::Textarea {
			class: tw_merge!(
					"border-input placeholder:text-muted-foreground focus-visible:ring-ring aria-invalid:ring-destructive/20 aria-invalid:border-destructive flex field-sizing-content min-h-16 w-full rounded-md border bg-transparent px-3 py-2 text-base shadow-xs transition-[color,box-shadow] outline-none focus-visible:ring-2 focus-visible:ring-offset-2 focus-visible:ring-offset-background disabled:cursor-not-allowed disabled:opacity-50 md:text-sm",
					class.clone()
			),
			value,
			default_value,
			on_value_change,
			debounce_ms,
			disabled,
			invalid,
			onchange,
			oninput,
			extra_attributes: attributes.clone(),
			{children}
		}
	}
}
