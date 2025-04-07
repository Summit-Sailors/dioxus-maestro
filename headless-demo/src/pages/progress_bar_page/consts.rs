pub const EXAMPLE:&str = "use dioxus::prelude::*;
use maestro_headless::{
  progewss::{Progress, ProgressIndicator},
  button::Button,
};
use dioxus_free_icons::{
  Icon,
  icons::io_icons::BsArrowCounterclockwise,
};
use async_std::task::sleep;
use std::time::Duration;

let mut progress = use_signal(|| 0.0_f32);
let max_value = 80.0;

use_future(move || async move {
  loop {
		let current_progress = *progress.peek();
		if current_progress < max_value {
			progress.set(current_progress + 2.0);
		}
		sleep(Duration::from_millis(200)).await;
	}
	});

rsx! {
  div {
    class: 'p-6 flex flex-col grow items-center justify-center gap-4 w-full',
		Progress {
			class: 'w-56 h-2 rounded-md overflow-hidden relative bg-neutral-300',
			value: progress(),
			ProgressIndicator {{ class: 'size-full bg-orange-600 rounded-md transition-transform duration-500' }}
		}
		Button {
			class: 'w-8 h-8 flex justify-center items-center bg-neutral-900 hover:bg-neutral-700 border border-neutral-300 text-neutral-300 hover:text-neutral-100 rounded-full focus-visible:ring-2 focus-visible:ring-offset-2 outline-none transition-colors focus-visible:neutral-300 focus-visible:ring-offset-neutral-900',
			onclick: move |_| progress.set(0.0),
			Icon { icon: BsArrowCounterclockwise }
		}
	}
}";

pub const EXAMPLE_ANATOMY: &str = "Progress { 
  ProgressIndicator { }
}";
