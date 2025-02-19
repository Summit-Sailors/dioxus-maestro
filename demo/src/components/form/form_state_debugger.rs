use {
	dioxus::prelude::*,
	maestro_forms::use_formik::Formik,
	maestro_ui::button::Button,
	serde::{Deserialize, Serialize},
	tailwind_fuse::tw_join,
	validator::Validate,
};

#[derive(Props, PartialEq, Clone)]
pub struct FormStateDebuggerProps<T>
where
	T: Validate + Clone + Serialize + PartialEq + 'static + for<'de> Deserialize<'de>,
{
	pub form: Formik<T>,
	#[props(default = false)]
	pub initial_expanded: bool,
}

#[component]
pub fn FormStateDebugger<T>(props: FormStateDebuggerProps<T>) -> Element
where
	T: Validate + Clone + Serialize + PartialEq + 'static + for<'de> Deserialize<'de>,
{
	let form = props.form;
	let mut show_debug = use_signal(|| props.initial_expanded);

	type ClassFn = fn(bool) -> &'static str;

	rsx! {
		div { class: "flex flex-col justify-center p-2 bg-gray-900 text-white rounded-lg shadow-md",
			div { class: "flex justify-center text-gray-300 hover:text-gray-100 transition",
				Button {
					class: "bg-blue-500 text-white hover:bg-blue-600",
					r#type: "button",
					onclick: move |_| show_debug.toggle(),
					if show_debug() {
						"Hide Form State"
					} else {
						"Show Form State"
					}
				}
			}

			if show_debug() {
				div { class: "mt-6 space-y-4",

					// form status grid
					div { class: "grid grid-cols-1 md:grid-cols-3 gap-4",
						{
								[
										(
												"Is Valid",
												*form.is_valid.read(),
												(|v: bool| v.then_some("text-green-500").unwrap_or("text-red-500"))
														as ClassFn,
										),
										(
												"Is Dirty",
												*form.is_dirty.read(),
												(|_: bool| "text-yellow-500") as ClassFn,
										),
										(
												"Is Submitting",
												*form.is_submitting.read(),
												(|_: bool| "text-blue-500") as ClassFn,
										),
										(
												"Custom Errors",
												!form.custom_errors.read().is_empty(),
												(|_: bool| "text-red-500") as ClassFn,
										),
								]
										.iter()
										.map(|(label, value, class_fn)| rsx! {
											div { class: "p-4 bg-gray-700 rounded-lg",
												span { class: "font-semibold", "{label}: " }
												span { class: tw_join!(class_fn(* value)), "{value}" }
											}
										})
						}
					}

					// field values
					div { class: "mt-4",
						h3 { class: "font-semibold mb-3 text-gray-500", "Field Values:" }
						div { class: "space-y-2",
							{
									form.name_to_id_map
											.read()
											.keys()
											.map(|name| {
													let value = form.get_field_json_value(name.clone());
													rsx! {
														div { class: "grid grid-cols-2 p-2 bg-gray-800 rounded-lg",
															span { class: "font-medium text-gray-400", "{name}: " }
															span { class: "break-words text-gray-300", "{value}" }
														}
													}
											})
							}
						}
					}

					// custom form errors section
					{
							(!form.custom_errors.read().is_empty())
									.then(|| rsx! {
										div { class: "mt-4 p-4 bg-red-900/20 rounded-lg",
											h3 { class: "font-semibold mb-2 text-red-400", "Form Errors:" }
											ul { class: "list-disc list-inside space-y-1 text-red-300",
												{form.custom_errors.read().iter().map(|error| rsx! {
													li { "{error}" }
												})}
											}
										}
									})
					}

					// complete form state
					div { class: "mt-4",
						h3 { class: "font-semibold mb-3 text-gray-500", "Complete Form State:" }
						pre { class: "p-4 bg-gray-800 text-gray-200 rounded-lg overflow-auto max-h-96",
							code { class: "font-mono text-sm",
								"{serde_json::to_string_pretty(&form.as_struct()).unwrap_or_else(|_| \"Serialization error\".to_string())}"
							}
						}
					}
				}
			}
		}
	}
}
