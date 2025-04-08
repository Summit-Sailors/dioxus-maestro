pub const EXAMPLE:&str = "use dioxus::prelude::*;
use maestro_headless::{
  hover_card::{HoverCardRoot, HoverCardArrow, HoverCardContent, HoverCardTrigger},
  shared::{EAlign, EOrientation, ESide}
};
use dioxus_free_icons::{
  Icon,
  icons::io_icons::IoLogoGithub,
};

rsx! {
  HoverCardRoot {
    class: 'w-fit',
    HoverCardTrigger {
      class: 'bg-neutral-700 text-neutral-100 rounded-sm w-56 p-4 data-[state-open]:animate-fade-in data-[state=closed]:animate-fade-out z-50',
      href: 'https://github.com/Summit-Sailors/dioxus-maestro/tree/dev/frontend/maestro-headless',
      Icon { icon: IoLogoGithub }
    }
    HoverCardContent {
      side: ESide::Top,
      side_offset: 6.0,
      align: EAlign::Center,
      class: 'w-full h-96 lg:max-w-lg md:max-w-md max-w-2xs max-h-[95vh] fixed z-[110] left-1/2 top-1/2 -translate-x-1/2 -translate-y-1/2 rounded bg-neutral-900 shadow border border-neutral-600 flex flex-col gap-6 px-6 py-8 overflow-y-auto data-[state=open]:animate-fade-in data-[state=closed]:animate-fade-out',
      div {
        class: 'flex flex-col',
        h3 { class: 'font-medium text-lg mb-1', 'Maestro-Headless' }
        p { class: 'text-neutral-300 mb-3', 'The part of Dioxus-Maestro project' }
        ul { 
          class: 'list-disc list-inside',
          li { 'Customizable' }
          li { 'Flexible' }
          li { 'Accessible' }
          }
        }
      HoverCardArrow {
        width: 16.0,
        height: 8.0,
        class: 'text-neutral-700',
      }
    }
  }
}";

pub const EXAMPLE_ANATOMY: &str = "HoverCardRoot {
  HoverCardTrigger { } 
  HoverCardContent {
    HoverCardArrow { }
  }
}";
