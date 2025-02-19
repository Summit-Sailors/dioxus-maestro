use {
	dioxus::prelude::*,
	dioxus_free_icons::{
		icons::io_icons::{IoCheckmarkOutline, IoChevronDownOutline},
		Icon,
	},
	tailwind_fuse::tw_join,
};

#[derive(Clone, PartialEq)]
pub struct SelectOption<T> {
	pub label: String,
	pub value: T,
}

#[derive(Props, Clone, PartialEq)]
pub struct SelectProps<T>
where
	T: Clone + PartialEq + std::fmt::Display + 'static,
{
	pub options: ReadOnlySignal<Vec<SelectOption<T>>>,
	pub current_value: ReadOnlySignal<Option<T>>,
	pub multi: bool,
	pub callback: EventHandler<T>,
	pub multi_callback: EventHandler<Vec<T>>,
	pub label: Option<String>,
	pub placeholder: Option<String>,
	pub option_class: Option<String>,
	pub dropdown_class: Option<String>,
	pub container_class: Option<String>,
	pub button_class: Option<String>,
	pub label_class: Option<String>,
	pub icon_down: Option<Element>,
	pub icon_check: Option<Element>,
	pub option_renderer: Option<fn(&SelectOption<T>) -> Element>,
}

// classes may be extended also by using "maestro-select__*" classname

#[component]
pub fn Select<T: Clone + PartialEq + std::fmt::Display + 'static>(props: SelectProps<T>) -> Element {
	let mut is_opened = use_signal(|| false);
	let mut selected_options = use_signal(|| Vec::<T>::new());

	let display_value = if props.multi {
		if selected_options().is_empty() {
			props.placeholder.clone().unwrap_or_default()
		} else {
			props.options.read().iter().filter(|option| selected_options().contains(&option.value)).map(|option| option.label.clone()).collect::<Vec<_>>().join(", ")
		}
	} else if props.current_value.read().is_some() {
		props
			.options
			.read()
			.iter()
			.find(|option| props.current_value.read().as_ref().unwrap() == &option.value)
			.map(|x| x.label.clone())
			.unwrap_or(props.placeholder.clone().unwrap_or_default())
	} else {
		props.placeholder.clone().unwrap_or_default()
	};

	let icon_down = props.icon_down.clone().unwrap_or_else(|| {
		rsx! {
			Icon {
				width: 16,
				height: 16,
				icon: IoChevronDownOutline,
				class: "fill-none",
			}
		}
	});

	let icon_check = props.icon_check.clone().unwrap_or_else(|| {
		rsx! {
			Icon {
				width: 16,
				height: 16,
				icon: IoCheckmarkOutline,
				class: "fill-none",
			}
		}
	});

	rsx! {
		div {
			class: tw_join!(
					"flex flex-col gap-2 w-full relative maestro-select__container", props
					.container_class.clone().unwrap_or_default(),
			),
			if let Some(label) = props.label.clone() {
				span {
					class: tw_join!(
							"text-gray-400 maestro-select__label", props.label_class.clone()
							.unwrap_or_default()
					),
					{label}
				}
			}
			div {
				class: tw_join!(
						"relative w-full bg-gray-800 text-gray-100 border border-gray-500 rounded-md hover:border-indigo-300 transition-colors ease-linear cursor-pointer maestro-select__select",
						is_opened().then_some("ring-1 ring-indigo-500"), props.button_class.clone()
						.unwrap_or_default()
				),
				onclick: move |ev| {
						ev.prevent_default();
						is_opened.toggle();
				},
				div { class: "flex items-center justify-between py-2 px-3 w-full rounded-md focus:outline-none focus:ring-1 focus:ring-indigo-500 overflow-hidden whitespace-nowrap text-ellipsis",
					span { "{display_value}" }
					div { class: "ml-2", {icon_down} }
				}
				div {
					class: tw_join!(
							"absolute flex flex-col gap-1 bg-gray-800 text-gray-200 p-4 w-full left-0 right-0 top-[100%] mt-3 rounded-md border border-gray-700 max-h-48 overflow-y-auto maestro-select__options",
							if is_opened() { "flex z-40" } else { "hidden -z-40" }, props.dropdown_class
							.clone().unwrap_or_default()
					),
					onclick: move |ev| {
							ev.stop_propagation();
					},
					for value in props.options.read().clone() {
						div {
							key: "{value.value}",
							class: tw_join!(
									"flex w-full items-center justify-between py-2 hover:bg-gray-700 rounded px-3 cursor-pointer maestro-select__option",
									props.option_class.clone().unwrap_or_default()
							),
							onclick: move |ev| {
									ev.stop_propagation();
									if props.multi {
											let mut current = selected_options().clone();
											if current.contains(&value.value) {
													current.retain(|x| x != &value.value);
											} else {
													current.push(value.value.clone());
											}
											selected_options.set(current);
											props.multi_callback.call(selected_options().clone());
									} else {
											is_opened.set(false);
											props.callback.call(value.value.clone());
									}
							},
							{
									if let Some(renderer) = props.option_renderer {
											renderer(&value)
									} else {
											rsx! {
											"{value.value}"
											}
									}
							}
							if props.multi && selected_options().contains(&value.value)
									|| !props.multi && props.current_value.read().as_ref() == Some(&value.value)
							{
								div { class: "ml-2", {icon_check.clone()} }
							}
						}
					}
				}
			}
		}
	}
}
