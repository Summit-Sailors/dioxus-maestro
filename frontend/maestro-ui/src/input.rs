use {dioxus::prelude::*, tailwind_fuse::*};

#[derive(TwClass)]
#[tw(
	class = "flex h-10 w-full bg-transparent py-2 text-sm file:border-0 file:bg-transparent file:text-sm file:font-medium placeholder:text-gray-500 focus-visible:outline-none focus-visible:ring-0  disabled:cursor-not-allowed disabled:opacity-50"
)]
pub struct InputClass {
	pub variant: InputVariant,
}

#[derive(PartialEq, TwVariant)]
pub enum InputVariant {
	#[tw(default, class = "border rounded-md px-3")]
	Default,
	#[tw(class = "border-b pt-2 pb-1")]
	Underlined,
}

#[derive(Clone, PartialEq, Props)]
pub struct InputProps {
	#[props(default = InputVariant::Default)]
	pub variant: InputVariant,
	pub onchange: Option<EventHandler<Event<FormData>>>,
	pub onenter: Option<EventHandler<Event<KeyboardData>>>,
	pub onfocus: Option<EventHandler<Event<FocusData>>>,
	pub onblur: Option<EventHandler<Event<FocusData>>>,
	pub class: Option<String>,
	pub error_class: Option<String>,
	pub style: Option<String>,
	#[props(default = "".to_string())]
	pub error: String,
	#[props(default = None)]
	pub children: Element,
	#[props(extends = GlobalAttributes, extends = input)]
	pub attributes: Vec<Attribute>,
}

#[component]
pub fn Input(props: InputProps) -> Element {
	let class = InputClass { variant: props.variant }.with_class(props.class.unwrap_or_default());

	let has_error = !props.error.is_empty();

	rsx! {
		div { class: "flex flex-col gap-2 w-full relative",
			div { class: "relative",
				input {
					class: tw_join!(class, (has_error).then_some("border-danger")),
					style: props.style.unwrap_or_default(),
					oninput: move |event| {
							props.onchange.unwrap_or_default().call(event);
					},
					onfocus: move |event| {
							props.onfocus.unwrap_or_default().call(event);
					},
					onblur: move |event| {
							props.onblur.unwrap_or_default().call(event);
					},
					onkeypress: move |event| {
							if event.data().code() == Code::Enter {
									props.onenter.unwrap_or_default().call(event);
							}
					},
					..props.attributes,
				}
				{props.children}
			}
			span {
				class: tw_merge!(
						"text-xs min-h-4 text-left", props.error_class.clone().unwrap_or_default()
				),
				{props.error}
			}
		}
	}
}
