use dioxus::prelude::*;
use maestro_forms::use_formik::Formik;
use maestro_ui::button::Button;
use serde::{Deserialize, Serialize};
use validator::Validate;

#[derive(Clone, PartialEq, Props)]
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

	rsx! {
		div { class: "flex flex-col justify-center bg-[color:var(--bg-color)] lg:py-16 sm:py-8 py-6 gap-8 text-[color:var(--text-color)]",
			div { class: "flex justify-center transition hover:text-[color:var(--highlight-color)]",
				Button {
					class: "bg-[color:var(--primary-bg)] text-[color:var(--primary-text)] hover:bg-[color:oklch(0.52_0.19_263.83)]",
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
				div { class: "flex flex-col rounded-[var(--radius-lg)] gap-5 bg-[color:var(--card-bg)] sm:px-6 px-2 py-8",
					div { class: "flex gap-6 items-center flex-wrap",
						{
								let is_valid = *form.is_valid.read();
								let is_dirty = *form.is_dirty.read();
								let is_submitting = *form.is_submitting.read();
								let has_custom_errors = !form.custom_errors.read().is_empty();
								let _ = [
										("Is Valid", is_valid),
										("Is Dirty", is_dirty),
										("Is Submitting", is_submitting),
										("Custom Errors", has_custom_errors),
								]
										.iter()
										.map(|(label, value)| rsx! {
											div { class: "px-3 py-2 bg-[color:var(--input-bg)] border border-[color:var(--border-color)] rounded-md flex items-center justify-center font-medium text-[color:var(--card-text)]",
												"{label}: {value}"
											}
										});
						}
					}

					div { class: "flex flex-col gap-3",
						h3 { class: "font-medium text-[color:var(--card-text)]", "Field Values:" }
						div { class: "overflow-hidden rounded-md border border-[color:var(--border-color)] grid grid-cols-[max-content_1fr]",
							{
									let field_names: Vec<String> = form
											.name_to_id_map
											.read()
											.keys()
											.cloned()
											.collect();
									field_names
											.into_iter()
											.map(|name| {
													let value = form.get_field_json_value(name.clone());
													rsx! {
														span { class: "flex px-3 py-2 items-center justify-center font-medium text-[color:var(--card-text)] bg-[color:var(--input-bg)] border-r border-[color:var(--border-color)] border-b last:border-b-0 [&:nth-last-child(-n+2)]:border-b-0",
															"{name}: "
														}
														span { class: "flex px-3 py-2 break-words text-[color:var(--card-text)] border-b border-[color:var(--border-color)] last:border-b-0 [&:nth-last-child(-n+2)]:border-b-0",
															"{value}"
														}
													}
											})
							}
						}
					}

					{
							let custom_errors = form.custom_errors.read();
							let has_errors = !custom_errors.is_empty();
							let errors_vec: Vec<String> = custom_errors.iter().cloned().collect();
							has_errors.then(|| rsx! {
								div { class: "flex flex-col gap-3",
									h3 { class: "font-medium text-[color:var(--card-text)]", "Form Errors:" }
									ul { class: "list-disc list-inside space-y-1 text-[color:var(--destructive)]",
										{errors_vec.iter().map(|error| rsx! {
											li { "{error}" }
										})}
									}
								}
							})
					}

					div { class: "flex flex-col gap-3",
						h3 { class: "font-medium text-[color:var(--card-text)]",
							"Complete Form State:"
						}
						pre { class: "px-3 py-2 bg-[color:var(--bg-color)] text-[color:var(--text-color)] rounded-md overflow-auto max-h-96",
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
