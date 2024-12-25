use {dioxus::prelude::*, tailwind_fuse::*};

#[derive(TwClass)]
#[tw(
	class = "text-foreground flex h-10 w-full border-input bg-transparent py-2 text-sm file:border-0 file:bg-transparent file:text-sm file:font-medium placeholder:text-muted-foreground focus-visible:outline-none focus-visible:ring-0  disabled:cursor-not-allowed disabled:opacity-50"
)]
pub struct InputClass {
	pub variant: InputVariant,
}

#[derive(PartialEq, TwVariant)]
pub enum InputVariant {
	#[tw(default, class = "border rounded-md px-3 focus-visible:border-primary")]
	Default,
	#[tw(class = "border-b focus-visible:border-b-primary")]
	Underlined,
}

#[derive(PartialEq, Clone, Copy)]
pub enum InputType {
	Text,
	Search,
	Password,
	Email,
}

impl InputType {
	fn as_str(&self) -> &'static str {
		match self {
			InputType::Text => "text",
			InputType::Search => "search",
			InputType::Password => "password",
			InputType::Email => "email",
		}
	}
}

#[derive(PartialEq, Clone, Props)]
pub struct InputProps {
	#[props(default = false)]
	pub disabled: bool,
	#[props(default = InputType::Text)]
	pub input_type: InputType,
	#[props(default = "".to_string())]
	pub value: String,
	#[props(default = InputVariant::Default)]
	pub variant: InputVariant,
	pub on_change: Option<EventHandler<String>>,
	pub on_enter: Option<EventHandler<String>>,
	pub id: Option<String>,
	pub class: Option<String>,
	pub style: Option<String>,
	pub placeholder: Option<String>,
	#[props(default = "".to_string())]
	pub error: String,
	#[props(default = None)]
	pub children: Element,
}

#[component]
pub fn Input(props: InputProps) -> Element {
	let class = InputClass { variant: props.variant }.with_class(props.class.unwrap_or_default());

	let has_error = !props.error.is_empty();

	let mut local_value = use_signal(|| props.value.to_owned());

	rsx! {
		div { class: "flex flex-col gap-2 w-full relative",
			div { class: "relative",
				input {
					id: props.id.unwrap_or_default(),
					class: tw_join!(class, (has_error).then_some("border-danger")),
					r#type: props.input_type.as_str(),
					style: props.style.unwrap_or_default(),
					disabled: props.disabled,
					value: props.value,
					placeholder: props.placeholder.unwrap_or_default(),
					oninput: move |event| {
							local_value.set(event.value());
							props.on_change.unwrap_or_default().call(local_value.read().to_string());
					},
					onkeypress: move |event| {
							if event.data().code() == Code::Enter {
									props.on_enter.unwrap_or_default().call(local_value.read().to_string());
							}
					},
				}
				{props.children}
			}
			if has_error {
				span { class: "text-xs text-destructive", {props.error} }
			}
		}
	}
}
