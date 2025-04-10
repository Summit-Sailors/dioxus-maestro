pub const EXAMPLE:&str = "use dioxus::prelude::*;
use maestro_headless::collapsible::{DialogRoot, DialogBody, DialogClose, DialogContent, DialogDescription, DialogFooter, DialogHeader, DialogOverlay, DialogTitle, DialogTrigger};
use dioxus_free_icons::{
  Icon,
  icons::{
    bs_icons::BsThreeDots,
    ld_icons::LdX,
  },
};

rsx! {
  DialogRoot {
    DialogTrigger {
      class: 'rounded-full w-10 h-10 flex items-center justify-center focus-visible:ring-2 focus-visible:ring-offset-2 outline-none transition-colors bg-neutral-900 border border-neutral-300 text-orange-600 hover:border-neutral-100 focus-visible:ring-neutral-300 focus-visible:ring-offset-neutral-900',
      Icon { icon: BsThreeDots }
    }
    DialogOverlay { class: 'w-full h-full fixed top-0 left-0 bottom-0 right-0 bg-neutral-900/20 backdrop-blur-sm z-[100] data-[state=open]:animate-fade-in data-[state=closed]:animate-fade-out' }
    DialogContent {
      class: 'w-full h-96 lg:max-w-lg md:max-w-md max-w-2xs max-h-[95vh] fixed z-[110] left-1/2 top-1/2 -translate-x-1/2 -translate-y-1/2 rounded bg-neutral-900 shadow border border-neutral-600 flex flex-col gap-6 px-6 py-8 overflow-y-auto data-[state=open]:animate-fade-in data-[state=closed]:animate-fade-out',
      DialogHeader {
        class: 'flex justify-between gap-4',
        div {
          class: 'flex flex-col gap-2',
          DialogTitle {
            class: 'font-medium text-2xl text-neutral-100',
            'Uncontrolled dialog'
          }
          DialogDescription { class: 'text-neutral-300', 'But it may be controlled too' }
        }
        DialogClose {
          title: 'Close my popup',
          class: 'w-8 h-8 flex items-center justify-center text-neutral-300 hover:text-neutral-100 focus-visible:outline-none focus-visible:ring-1 focus-visible:ring-neutral-300 transition-colors',
          Icon { width: 16, height: 16, icon: LdX }
        }
      }
      DialogBody {
        class: 'flex flex-col gap-3',
        p { 'Here may be any type of content, for example, an image' }
        div {
          class: 'overflow-hidden rounded-lg border border-neutral-500 w-full h-40',
          img {
            class: 'object-cover size-full',
            src: 'https://www.blueplanetaquarium.com/wp-content/uploads/2023/09/iStock-1405520633-1024x682.jpg',
            alt: 'whales',
          }
        }
      }
      DialogFooter {
        DialogClose {
          title: 'Close my popup',
          class: 'mx-auto h-8 flex items-center justify-center text-neutral-100 bg-orange-600 px-3 py-2 rounded-md hover:bg-neutral-900 border border-transparent hover:border-orange-600 focus-visible:outline-none focus-visible:ring-1 focus-visible:ring-orange-600 transition-colors',
          'Close'
        }
      }
    }
  }
}";

pub const EXAMPLE_ANATOMY: &str = "DialogRoot {
  DialogTrigger { } 
  DialogOverlay { }
    DialogContent {
      DialogHeader {
        DialogTitle { }
        DialogDescription { }
      }
    }
    DialogBody { }
    DialogFooter {
      DialogClose { }
    }
  }
}";
