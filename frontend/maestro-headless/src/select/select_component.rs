use {
	dioxus::prelude::*,
	dioxus_free_icons::{
		icons::bs_icons::{BsCheck, BsChevronDown},
		Icon,
	},
	dioxus_logger::tracing::info,
	std::{
		cmp::Ordering,
		fmt::{Debug, Display},
	},
};

#[derive(Clone, PartialEq, Debug, Copy)]
pub struct SelectContext<T>
where
	T: Clone + PartialEq + Display + Debug + 'static,
{
	pub options: Signal<Vec<SelectOption<T>>>,
	pub value: Signal<Option<T>>,
	pub onchange: Option<Callback<T>>,
	pub open: Signal<bool>,
	pub disabled: Signal<bool>,
	pub onopenchange: Option<Callback<bool>>,
	pub search_input: Signal<String>,
	pub is_searchable: bool,
}

impl<T> SelectContext<T>
where
	T: Clone + PartialEq + Display + Debug + 'static,
{
	pub fn new(
		value: Signal<Option<T>>,
		options: Signal<Vec<SelectOption<T>>>,
		onchange: Option<Callback<T>>,
		open: Signal<bool>,
		onopenchange: Option<Callback<bool>>,
		is_searchable: bool,
		disabled: Signal<bool>,
	) -> Self {
		info!("RERUN");
		Self { value, options, onchange, open, onopenchange, is_searchable, disabled, search_input: Signal::new(String::default()) }
	}

	fn set_options(&mut self, options: Vec<SelectOption<T>>) {
		self.options.set(options);
	}

	pub fn on_search_input_change(&mut self) {
		let search = self.search_input.peek().clone();
		let current_value = self.value.peek().clone();
		let options = self.options.peek().clone();
		let mut filtered = options.into_iter().filter(|option| option.label.to_lowercase().contains(&search.to_lowercase())).collect::<Vec<SelectOption<T>>>();
		filtered.sort_by(|a, b| {
			if let Some(current_value) = &current_value {
				if &a.value == current_value {
					Ordering::Less
				} else if &b.value == current_value {
					Ordering::Greater
				} else {
					Ordering::Equal
				}
			} else {
				Ordering::Equal
			}
		});
		self.set_options(filtered);
	}
}

#[derive(Clone, PartialEq, Debug)]
pub struct SelectOption<T> {
	pub label: String,
	pub value: T,
}

#[derive(Clone, PartialEq, Props)]
pub struct SelectProps<T>
where
	T: Clone + PartialEq + Display + Debug + 'static,
{
	pub options: Vec<SelectOption<T>>,
	#[props(default = Signal::new(None))]
	pub value: Signal<Option<T>>,
	pub onchange: Option<Callback<T>>,
	pub onopenchange: Option<Callback<bool>>,
	#[props(default = false)]
	pub is_searchable: bool,
	#[props(default = false)]
	pub default_open: bool,
	#[props(default = false)]
	pub disabled: bool,
	pub children: Element,
	#[props(extends = select, extends = GlobalAttributes)]
	attributes: Vec<Attribute>,
}

#[component]
pub fn Select<T: Clone + PartialEq + Display + Debug + 'static>(props: SelectProps<T>) -> Element {
	let SelectProps { options, default_open, disabled, onchange, onopenchange, value, is_searchable, children, attributes } = props;

	let options = use_signal::<Vec<SelectOption<T>>>(|| options.clone());
	let is_disabled = use_signal(|| disabled);
	let is_default_open = use_signal(|| default_open);

	let mut select_context = use_context_provider(|| SelectContext::new(value, options, onchange, is_default_open, onopenchange, is_searchable, is_disabled));

	use_effect(use_reactive!(|disabled| {
		if disabled != *select_context.disabled.peek() {
			select_context.disabled.set(disabled);
		}
	}));

	rsx! {
		div {
			aria_haspopup: "listbox",
			aria_expanded: *select_context.open.read(),
			aria_disabled: "{*select_context.disabled.read()}",
			tabindex: if *select_context.disabled.read() { "-1" } else { "0" },
			..attributes,
			{children}
		}
	}
}

#[derive(Props, PartialEq, Clone)]
pub struct SelectTriggerProps {
	pub children: Element,
	#[props(default = None)]
	pub onclick: Option<EventHandler<MouseEvent>>,
	#[props(default = None)]
	pub onkeydown: Option<EventHandler<KeyboardEvent>>,
	#[props(extends = button, extends = GlobalAttributes)]
	attributes: Vec<Attribute>,
	#[props(optional, default=None)]
	icon: Option<Element>,
}

#[component]
pub fn SelectTrigger<T: Clone + PartialEq + Debug + Display + 'static>(props: SelectTriggerProps) -> Element {
	let mut select_context = use_context::<SelectContext<T>>();

	let icon_down = props.icon.clone().unwrap_or_else(|| {
		rsx! {
			Icon {
				width: 16,
				height: 16,
				icon: BsChevronDown,
				style: if *select_context.open.read() { "transform: rotateX(180deg);" } else { "" },
			}
		}
	});
	rsx! {
		button {
			onclick: move |event| {
					props.onclick.unwrap_or_default().call(event);
					select_context.open.toggle();
					select_context
							.onopenchange
							.unwrap_or_default()
							.call(*select_context.open.peek());
			},
			onkeydown: move |event| {
					props.onkeydown.unwrap_or_default().call(event);
					select_context.open.toggle();
					select_context
							.onopenchange
							.unwrap_or_default()
							.call(*select_context.open.peek());
			},
			{props.children}
			{icon_down}
		}
	}
}

#[derive(Props, PartialEq, Clone)]
pub struct SelectValueProps {
	#[props(extends = span, extends = GlobalAttributes)]
	attributes: Vec<Attribute>,
	#[props(default = String::default())]
	pub placeholder: String,
}

#[component]
pub fn SelectValue<T: Clone + PartialEq + Display + Debug + 'static>(props: SelectValueProps) -> Element {
	let select_context = use_context::<SelectContext<T>>();

	let value = select_context.value.read();

	rsx! {
		span {
			"data-state": if value.is_some() { "selected" } else { "placeholder" },
			..props.attributes.clone(),
			if value.as_ref().is_some() {
				"{value.as_ref().unwrap()}"
			} else {
				"{props.placeholder}"
			}
		}
	}
}

#[derive(Props, PartialEq, Clone)]
pub struct OptionProps<T>
where
	T: Clone + PartialEq + Display + Debug + 'static,
{
	#[props(extends = li, extends = GlobalAttributes)]
	attributes: Vec<Attribute>,
	#[props(optional, default=None)]
	onclick: Option<EventHandler<MountedData>>,
	option: SelectOption<T>,
	children: Element,
}

#[component]
fn Option<T: Clone + PartialEq + Display + Debug + 'static>(props: OptionProps<T>) -> Element {
	let select_context = use_context::<SelectContext<T>>();

	rsx! {
		li {..props.attributes,

			{props.children}
			Icon { width: 16, height: 16, icon: BsCheck }
		}
	}
}

#[derive(Props, PartialEq, Clone)]
pub struct SelectDropdownProps<T>
where
	T: Clone + PartialEq + Display + Debug + 'static,
{
	#[props(extends = span, extends = GlobalAttributes)]
	attributes: Vec<Attribute>,
	#[props(optional, default=None)]
	value_renderer: Option<Component<OptionProps<T>>>,
}

#[component]
pub fn SelectDropdown<T: Clone + PartialEq + Display + Debug + 'static>(props: SelectDropdownProps<T>) -> Element {
	let select_context = use_context::<SelectContext<T>>();

	let ValueRender = props.value_renderer.unwrap_or(Option::<T>);
	rsx! {
		div {..props.attributes,
			ul {
				for option in select_context.options.read().iter() {
					ValueRender { option: option.clone() }
				}
			}
		}
	}
}

// #[component]
// pub fn Select<T: Clone + PartialEq + std::fmt::Display + 'static>(props: SelectProps<T>) -> Element {
// 	let SelectProps { options, default_open, onchange, onopenchange, value, is_searchable, children } = props;

// 	use_context_provider(|| SelectContext::new(value, Signal::new(options), onchange, Signal::new(default_open), onopenchange, is_searchable));
// 	rsx! {
// 		div { {children} }
// 	}
// }

// use {
// 	crate::{
// 		button::{Button, ButtonSize, ButtonVariant},
// 		input::{Input, InputVariant},
// 	},
// 	dioxus::prelude::*,
// 	dioxus_free_icons::{
// 		icons::{
// 			bs_icons::BsSearch,
// 			io_icons::{IoCheckmarkOutline, IoChevronDownOutline},
// 			ld_icons::LdX,
// 		},
// 		Icon,
// 	},
// 	std::cmp::Ordering,
// 	tailwind_fuse::*,
// };

// #[derive(Clone, PartialEq)]
// pub struct SelectOption<T> {
// 	pub label: String,
// 	pub value: T,
// }

// #[derive(Clone, PartialEq, Props)]
// pub struct SelectProps<T>
// where
// 	T: Clone + PartialEq + std::fmt::Display + 'static,
// {
// 	pub options: Vec<SelectOption<T>>,
// 	#[props(default = None)]
// 	pub current_value: Option<T>,
// 	pub onchange: Option<EventHandler<T>>,
// 	#[props(default = String::new())]
// 	pub label: String,
// 	#[props(default = String::new())]
// 	pub placeholder: String,
// 	#[props(default = String::new())]
// 	pub placeholder_class: String,
// 	#[props(default = String::new())]
// 	pub option_class: String,
// 	#[props(default = String::new())]
// 	pub dropdown_class: String,
// 	#[props(default = String::new())]
// 	pub container_class: String,
// 	#[props(default = String::new())]
// 	pub button_class: String,
// 	#[props(default = String::new())]
// 	pub button_wrapper_class: String,
// 	#[props(default = String::new())]
// 	pub label_class: String,
// 	#[props(default = String::new())]
// 	pub search_input_container_class: String,
// 	#[props(default = String::new())]
// 	pub search_clear_class: String,
// 	#[props(default = String::new())]
// 	pub search_input_class: String,
// 	pub icon_down: Option<Element>,
// 	pub icon_check: Option<Element>,
// 	pub option_renderer: Option<fn(&SelectOption<T>) -> Element>,
// 	pub onopen: Option<EventHandler<Option<Event<MouseData>>>>,
// 	pub onclose: Option<EventHandler<Option<Event<FocusData>>>>,
// 	#[props(default = false)]
// 	pub is_searchable: bool,
// }

// #[component]
// pub fn Select<T: Clone + PartialEq + std::fmt::Display + 'static>(props: SelectProps<T>) -> Element {
// 	let mut is_opened = use_signal(|| false);
// 	let mut search_input = use_signal(String::new);
// 	let SelectProps { options, current_value, placeholder, ref placeholder_class, .. } = props;

// 	let display_value =
// 		current_value.as_ref().and_then(|value| options.iter().find(|opt| &opt.value == value).map(|opt| opt.label.clone())).unwrap_or_else(||
// placeholder.clone());

// 	let is_placeholder = current_value.is_none();
// 	let display_options = {
// 		let search = search_input();

// 		let mut filtered =
// 			options.clone().into_iter().filter(|option| option.label.to_lowercase().contains(&search.to_lowercase())).collect::<Vec<SelectOption<T>>>();

// 		filtered.sort_by(|a, b| {
// 			if let Some(current_value) = &current_value {
// 				if &a.value == current_value {
// 					Ordering::Less
// 				} else if &b.value == current_value {
// 					Ordering::Greater
// 				} else {
// 					Ordering::Equal
// 				}
// 			} else {
// 				Ordering::Equal
// 			}
// 		});
// 		filtered
// 	};

// 	let icon_down = props.icon_down.clone().unwrap_or_else(|| {
// 		rsx! {
// 			Icon {
// 				width: 16,
// 				height: 16,
// 				icon: IoChevronDownOutline,
// 				class: tw_merge!(
// 						"absolute top-0 bottom-0 my-auto right-3 transition-all ease-linear fill-none",
// 						(is_opened()).then_some("rotate-180")
// 				),
// 			}
// 		}
// 	});

// 	rsx! {
// 		div { class: tw_merge!("flex flex-col gap-2 w-full relative min-h-12", & props.container_class),
// 			if !props.label.is_empty() {
// 				span { class: tw_merge!("text-gray-400", & props.label_class), {props.label} }
// 			}
// 			div {
// 				class: tw_merge!(
// 						"relative w-full cursor-pointer maestro-select-button__wrapper", & props
// 						.button_wrapper_class
// 				),
// 				button {
// 					class: tw_merge!(
// 							"min-h-12 flex items-center border border-gray-500 rounded-md transition-colors ease-linear relative flex py-2 px-3 w-full h-full focus:outline-none
// focus-visible:ring-2 focus-visible:ring-offset-2 focus-visible:ring-gray-700 focus-visible:ring-offset-white maestro-select-button", 							& props.
// button_class 					),
// 					onfocusout: move |ev| {
// 							is_opened.set(false);
// 							search_input.set(String::new());
// 							props.onclose.unwrap_or_default().call(Some(ev));
// 					},
// 					onmousedown: move |ev| {
// 							ev.prevent_default();
// 							ev.stop_propagation();
// 							is_opened.toggle();
// 							props.onopen.unwrap_or_default().call(Some(ev))
// 					},
// 					r#type: "button",
// 					span {
// 						class: tw_merge!(
// 								"line-clamp-1 pr-2 text-left", if is_placeholder { placeholder_class } else { ""
// 								}
// 						),
// 						"{display_value}"
// 					}
// 					{icon_down}
// 				}
// 				div {
// 					class: tw_merge!(
// 							"absolute flex flex-col gap-1 p-4 w-full left-0 right-0 top-[100%] mt-3 rounded-md max-h-48 overflow-y-auto bg-gray-100 border
// maestro-select-dropdown", 							if is_opened() { "flex z-40" } else { "hidden -z-40" }, & props.dropdown_class
// 					),
// 					onclick: move |ev| {
// 							ev.stop_propagation();
// 					},
// 					if props.is_searchable {
// 						div {
// 							class: tw_merge!(
// 									"relative px-3 text-gray-500 maestro-select-search_container", & props
// 									.search_input_container_class
// 							),
// 							Icon {
// 								width: 16,
// 								icon: BsSearch,
// 								class: "absolute top-2.5 left-3",
// 							}
// 							Input {
// 								r#type: "text",
// 								variant: InputVariant::Underlined,
// 								value: search_input(),
// 								class: tw_merge!(
// 										"px-7 focus-visible:ring-0 focus-visible:ring-transparent focus-visible:ring-none focus-visible:ring-offset-0 maestro-select-search_input",
// 										& props.search_input_class
// 								),
// 								onchange: move |event: Event<FormData>| search_input.set(event.value()),
// 							}
// 							Button {
// 								variant: ButtonVariant::Icon,
// 								r#type: "button",
// 								size: ButtonSize::IconSm,
// 								class: tw_merge!(
// 										"h-fit w-fit absolute top-2.5 right-3 text-gray-500 hover:text-gray-700 focus-visible:text-gray-700 focus-visible:ring-0
// focus-visible:ring-transparent focus-visible:ring-none focus-visible:ring-offset-0 maestro-select-search_clear", 										& props.search_clear_class
// 								),
// 								onclick: move |event: Event<MouseData>| {
// 										event.stop_propagation();
// 										search_input.set(String::new());
// 								},
// 								Icon { width: 16, icon: LdX }
// 							}
// 						}
// 					}
// 					{
// 							display_options
// 									.iter()
// 									.map(|option| {
// 											let option_clone = option.clone();
// 											rsx! {
// 												div {
// 													key: "{option.value}",
// 													id: "{option.value}",
// 													class: tw_merge!(
// 															"flex w-full items-center py-2 hover:bg-gray-300 rounded px-3 cursor-pointer", &
// 															props.option_class
// 													),
// 													onclick: move |ev| {
// 															ev.stop_propagation();
// 															is_opened.set(false);
// 															props.onclose.unwrap_or_default().call(None);
// 															search_input.set(String::new());
// 															props.onchange.unwrap_or_default().call(option_clone.value.clone());
// 													},
// 													{
// 															if let Some(renderer) = props.option_renderer {
// 																	renderer(option)
// 															} else {
// 																	rsx! {
// 																	"{option.label}"
// 																	}
// 															}
// 													}
// 													if props.icon_check.is_some() {
// 														{props.icon_check.clone().unwrap()}
// 													} else {
// 														Icon {
// 															icon: IoCheckmarkOutline,
// 															class: tw_merge!(
// 																	"fill-none ml-auto", if current_value.as_ref() == Some(& option.value) {
// 																	"opacity-100" } else { "opacity-0" }
// 															),
// 														}
// 													}
// 												}
// 											}
// 									})
// 					}
// 				}
// 			}
// 		}
// 	}
// }
