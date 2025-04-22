use std::fmt::Error;

use dioxus::prelude::*;
use maestro_query::prelude::{MutationResult, use_mutation};
use maestro_ui::button::Button;
use tailwind_fuse::tw_join;

#[component]
pub fn SilentMutationDemo() -> Element {
	let mut counter = use_signal(|| 0);

	let silent_mutation = use_mutation(|value: i32| async move {
		async_std::task::sleep(std::time::Duration::from_secs(1)).await;
		MutationResult::Ok::<i32, Error>(value)
	});

	// triggers component re-render
	let handle_normal_mutation = move |_| {
		let new_value = counter() + 1;
		counter.set(new_value);
		silent_mutation.mutate(new_value);
	};

	// does not trigger component re-render
	let handle_silent_mutation = move |_| {
		let new_value = counter() + 1;
		counter.set(new_value);
		let silent_mutation = silent_mutation;
		spawn(async move {
			silent_mutation.mutate_silent(new_value).await;
		});
	};

	rsx! {
		div { class: "grid justify-center grid-cols-1 text-center p-4 border border-slate-700 bg-[color:var(--bg-color)] rounded-lg shadow-lg mt-4",
			h3 { class: "text-xl text-slate-100 font-bold mb-4", "Silent vs. Normal Mutations" }
			p { class: "mb-4 text-slate-200 font-bold",
				"Counter: "
				span { class: "text-yellow-500 font-bold", " {counter}" }
			}
			div { class: "space-x-2",
				Button {
					class: "bg-blue-500 text-white px-4 py-2 rounded",
					onclick: handle_normal_mutation,
					"Normal"
				}
				Button {
					class: "bg-green-500 text-white px-4 py-2 rounded",
					onclick: handle_silent_mutation,
					"Silent"
				}
			}

			div { class: "mt-4 text-slate-200 font-semibold",
				"Mutation Status: "
				match *silent_mutation.result() {
						MutationResult::Loading(_) => rsx! {
							span { class: "text-yellow-500", "Loading..." }
						},
						MutationResult::Ok(_) => rsx! {
							span { class: "text-green-500", "Success" }
						},
						MutationResult::Err(_) => rsx! {
							span { class: "text-red-500", "Error" }
						},
						MutationResult::Pending => rsx! {
							span { class: "text-slate-500", "Pending" }
						},
				}
			}
		}
	}
}

#[component]
pub fn ManualMutationDemo() -> Element {
	let mut status = use_signal(|| "Idle");

	let manual_mutation = use_mutation(|value: String| async move {
		async_std::task::sleep(std::time::Duration::from_secs(1)).await;
		MutationResult::<std::string::String, Error>::Ok(value)
	});

	let handle_manual_mutation = move |_| {
		let mutation = manual_mutation;
		status.set("Starting...");

		spawn(async move {
			status.set("Processing...");
			mutation.manual_mutate("Test".to_string()).await;
			status.set("Completed!");
		});
	};

	let status_class = move || match status() {
		"Idle" => "text-slate-500",
		"Starting..." => "text-blue-500",
		"Processing..." => "text-yellow-500",
		"Completed!" => "text-green-500",
		_ => "text-slate-500",
	};

	rsx! {
		div { class: "flex flex-col items-center bg-[color:var(--bg-color)] p-4 border border-slate-700 shadow-lg rounded mt-4",
			h3 { class: "text-xl font-bold text-slate-200 text-center mb-4", "Manual Mutation Control" }

			p { class: tw_join!("mb-4 text-center font-semibold", status_class()),
				"Status: {status}"
			}

			Button {
				class: "bg-blue-500 text-white px-4 py-2 rounded",
				onclick: handle_manual_mutation,
				"Trigger Manual Mutation"
			}
		}
	}
}
