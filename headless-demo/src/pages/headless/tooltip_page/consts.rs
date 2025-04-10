pub const EXAMPLE:&str = "use dioxus::prelude::*;
use maestro_headless::{
  popover::{PopoverRoot, PopoverArrow, PopoverClose, PopoverContent, PopoverTrigger},
  shared::{EAlign, EOrientation, ESide}
};

rsx! {
  TooltipRoot { 
    class: 'w-fit mx-auto'
    Tooltip { class: 'w-fit group',
      TooltipTrigger { 
        class: 'mx-auto w-8 h-8 bg-neutral-100 text-neutral-800 border border-transparent rounded-full transition-colors hover:bg-neutral-900 hover:border-orange-600 hover:text-orange-600 focus-visible::bg-neutral-900 focus-visible:border-orange-600 focus-visible:text-orange-600 focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-orange-600 focus-visible:ring-offset-2 focus-visible:ring-offset-neutral-900',
        '+'
      }
      TooltipContent {
        side: ESide::Top,
        side_offset: 8.0,
        align: EAlign::Center,
        class: 'data-[state=open]:opacity-100 data-[state=closed]:opacity-0 bg-neutral-700 text-slate-neutral-100 rounded-sm w-56 p-4 transition-opacity ease-linear',
        'Some help text'
        TooltipArrow {
          width: 16.0,
          height: 8.0,
          class: 'text-neutral-700',
        }
      }
    }
	}
}";

pub const EXAMPLE_ANATOMY: &str = "TooltipRoot {
  Tooltip {
    TooltipTrigger { } 
    TooltipContent {
      TooltipArrow { }
    }
  }
}";
