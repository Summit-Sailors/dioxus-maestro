pub const EXAMPLE: &str = "use dioxus::prelude::*;
use maestro_headless::switch::{SwitchRoot, SwitchIndicator};

rsx! {
  SwitchRoot {
    class: 'flex items-center px-1 py-1 rounded-full h-6 w-12 bg-neutral-500 data-[state=checked]:bg-neutral-100 border border-neutral-700',
    SwitchIndicator { class: 'relative data-[state=checked]:translate-x-5 transition ease-linear rounded-full w-5 h-5 bg-neutral-900' }
  }
}";

pub const EXAMPLE_ANATOMY: &str = "SwitchRoot {
  SwitchIndicator { }
}";
