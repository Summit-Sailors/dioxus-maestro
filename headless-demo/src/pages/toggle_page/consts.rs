pub const EXAMPLE_GROUP:&str = "use dioxus::prelude::*;
use maestro_headless::toggle_group::{ToggleGroupRoot, ToggleGroupItem}

let mut value = use_signal(|| '1'.to_string());

rsx! {
  ToggleGroupRoot {
    class: 'flex justify-center items-center rounded overflow-hidden border border-slate-700',
    value: value(),
    on_value_chenge: move |v: String| value.set(v),
    ToggleGroupItem {
      class: 'data-[state=on]:bg-orange-600  data-[state=on]:text-neutral-100 border-r border-r-neutral-700 bg-neutral-500 text-neutral-300 flex justify-center items-center w-12 h-12 hover:text-neutral-100 focus-visible:outline-none data-[state=on]focus-visible:bg-orange-700 data-[state=off]:focus-visible:bg-neutral-700',
      value: '1',
      Icon { icon: LdAlignRight }
    }
    ToggleGroupItem {
      class: 'data-[state=on]:bg-orange-600  data-[state=on]:text-neutral-100 border-r border-r-neutral-700 bg-neutral-500 text-neutral-300 flex justify-center items-center w-12 h-12 hover:text-neutral-100 focus-visible:outline-none data-[state=on]focus-visible:bg-orange-700 data-[state=off]:focus-visible:bg-neutral-700',
      value: '2',
      Icon { icon: LdAlignCenter }
    }
    ToggleGroupItem {
      class: 'data-[state=on]:bg-orange-600  data-[state=on]:text-neutral-100 border-r border-r-neutral-700 bg-neutral-500 text-neutral-300 flex justify-center items-center w-12 h-12 hover:text-neutral-100 focus-visible:outline-none data-[state=on]focus-visible:bg-orange-700 data-[state=off]:focus-visible:bg-neutral-700',
      value: '3',
      Icon { icon: LdAlignLeft }
    }
  }	
}";

pub const EXAMPLE_GROUP_ANATOMY: &str = "ToggleGroupRoot {
  ToggleGroupItem { }
}";

pub const EXAMPLE:&str = "use dioxus::prelude::*;
use maestro_headless::toggle::Toggle;

rsx! {
  Toggle {
    class: 'aria-[pressed=true]:bg-orange-600 bg-neutral-500 text-neutral-200 flex justify-center items-center p-3 w-10 h-10 rounded transition-colors hover:text-neutral-100 focus-visible:outline-none focus-visible:ring-1 focus-visible:ring-offset-2 focus-visible:ring-offset-neutral-900 aria-[pressed=true]:hover:bg-orange-700 aria-[pressed=true]:focus-visible:ring-orange-600 aria-[pressed=false]:hover:bg-neutral-700 aria-[pressed=false]:focus-visible:ring-neutral-500',
    value: 'on',
    default_pressed: false
  }
}";

pub const EXAMPLE_ANATOMY: &str = "Toggle {}";
