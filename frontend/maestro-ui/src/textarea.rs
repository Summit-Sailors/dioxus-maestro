use {crate::input::InputVariant, dioxus::prelude::*, tailwind_fuse::*};

#[derive(TwClass)]
#[tw(
	class = "text-foreground ease-linear w-full bg-transparent border-input placeholder:text-muted-foreground focus-visible:outline-none focus-visible:ring-0 focus-visible:border-b-primary disabled:cursor-not-allowed disabled:opacity-50 disabled:pointer-events-none transition-colors resize-none max-h-48 min-h-11 n-scrollbar"
)]
pub struct TextareaClass {
	pub variant: InputVariant,
}

#[derive(PartialEq, Clone, Props)]
pub struct TextareaProps {
	#[props(default = false)]
	pub disabled: bool,
	#[props(default = "".to_string())]
	pub value: String,
	pub on_change: Option<EventHandler<String>>,
	pub on_enter: Option<EventHandler<String>>,
	pub id: Option<String>,
	pub class: Option<String>,
	pub style: Option<String>,
	pub placeholder: Option<String>,
	#[props(default = None)]
	pub children: Element,
	#[props(default = InputVariant::Default)]
	pub variant: InputVariant,
}

#[component]
pub fn Textarea(props: TextareaProps) -> Element {
	let class = TextareaClass { variant: props.variant }.with_class(props.class.unwrap_or_default());
	let mut local_value = use_signal(|| props.value.to_owned());

	rsx! {
		div { class: "w-full relative",
			textarea {
				id: props.id.unwrap_or_default(),
				class,
				style: props.style.unwrap_or_default(),
				disabled: props.disabled,
				value: props.value,
				placeholder: props.placeholder.unwrap_or_default(),
				oninput: move |event| {
						local_value.set(event.value());
						props.on_change.unwrap_or_default().call(local_value.read().to_string());
				},
				onkeypress: move |event| {
						if event.data().code() == Code::Enter && event.data().modifiers().shift() {
								props.on_enter.unwrap_or_default().call(local_value.read().to_string());
						}
				},
			}
			{props.children}
		}
	}
}
