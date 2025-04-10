pub const EXAMPLE:&str = "use dioxus::prelude::*;
use maestro_headless::{
  popover::{PopoverRoot, PopoverArrow, PopoverClose, PopoverContent, PopoverTrigger},
  shared::{EAlign, EOrientation, ESide}
};
use dioxus_free_icons::{
  Icon,
  icons::io_icons::BsInfo,
};

rsx! {
  PopoverRoot {
    class: 'w-fit',
    PopoverTrigger {
      class: 'mx-auto w-10 h-10 flex justify-center items-center bg-neutral-900 hover:bg-neutral-700 border border-orange-600 text-neutral-300 hover:text-neutral-100 rounded-full focus-visible:ring-2 focus-visible:ring-offset-2 outline-none transition-colors focus-visible:ring-orange-600 focus-visible:ring-offset-neutral-900',
      Icon { icon: BsInfo {} }
    }
    PopoverContent {
      side_offset: 4.0,
      align: EAlign::Center,
      class: 'z-10 data-[state=open]:opacity-100 data-[state=closed]:opacity-0 bg-neutral-700 text-neutral-100 text-xs text-center rounded-sm p-2 transition-opacity ease-linear',
      div {
        class: 'flex flex-col gap-3',
        h3 { class: 'font-medium text-lg', 'Here maybe any content you want' }
        div {
          class: 'overflow-hidden rounded-lg border border-neutral-500 w-full h-40',
          img {
            class: 'object-cover size-full',
            src: 'https://www.blueplanetaquarium.com/wp-content/uploads/2023/09/iStock-1405520633-1024x682.jpg',
            alt: 'whales',
          }
        }
        PopoverClose {
          class: 'h-8 px-3 w-2/3 mx-auto rounded-md flex items-center justify-center bg-orange-600 text-neutral-100 border border-transparent transition-colors hover:bg-neutral-700 hover:border-orange-600 focus-visible:outline-none focus-visible:hover:bg-neutral-700 focus-visible:ring-2 focus-visible:ring-orange-600 focus-visible:ring-offset-2 focus-visible:ring-offset-neutral-700',
          'Close me'
        }
      }
      PopoverArrow {
        width: 16.0,
        height: 8.0,
        class: 'text-neutral-700',
      }
    }
  }
}";

pub const EXAMPLE_ANATOMY: &str = "PopoverRoot {
  PopoverTrigger { } 
  PopoverContent {
    PopoverClose { }
    PopoverArrow { }
  }
}";
