pub const EXAMPLE:&str = "use dioxus::prelude::*;
use maestro_headless::tabs::{TabsRoot, TabsContent, TabsList, TabsTrigger};

rsx! {
  TabsRoot { default_value: '1', class: 'flex flex-col gap-4',
    TabsList { 
    class: 'w-full flex items-center gap-6',
      TabsTrigger {
        value: '1',
        class: 'data-[disabled=true]:opacity-50 data-[disabled=true]:pointer-events-none py-2 font-medium text-neutral-300 border-b border-b-transparent hover:border-b-orange-300 data-[state=active]:border-b-orange-600 data-[state=active]:text-neutral-100 transition-all ease-linear focus-visible:outline-none data-[state=active]:focus-visible:border-b-neutral-100 focus-visible:text-neutral-100',
        'One'
      }
      TabsTrigger {
        value: '2',
        class: 'data-[disabled=true]:opacity-50 data-[disabled=true]:pointer-events-none py-2 font-medium text-neutral-300 border-b border-b-transparent hover:border-b-orange-300 data-[state=active]:border-b-orange-600 data-[state=active]:text-neutral-100 transition-all ease-linear focus-visible:outline-none data-[state=active]:focus-visible:border-b-neutral-100 focus-visible:text-neutral-100',
        'Two'
      }
      TabsTrigger {
        value: '3',
        class: 'data-[disabled=true]:opacity-50 data-[disabled=true]:pointer-events-none py-2 font-medium text-neutral-300 border-b border-b-transparent hover:border-b-orange-300 data-[state=active]:border-b-orange-600 data-[state=active]:text-neutral-100 transition-all ease-linear focus-visible:outline-none data-[state=active]:focus-visible:border-b-neutral-100 focus-visible:text-neutral-100',
        disabled: true,
        'Three'
      }
      TabsTrigger {
        value: '4',
        class: 'data-[disabled=true]:opacity-50 data-[disabled=true]:pointer-events-none py-2 font-medium text-neutral-300 border-b border-b-transparent hover:border-b-orange-300 data-[state=active]:border-b-orange-600 data-[state=active]:text-neutral-100 transition-all ease-linear focus-visible:outline-none data-[state=active]:focus-visible:border-b-neutral-100 focus-visible:text-neutral-100',
        'Four'
      }
		}
    TabsContent { value: '1',
      'Lorem ipsum dolor sit amet, consectetur adipiscing elit, sed do eiusmod tempor incididunt ut labore et dolore magna aliqua.'
    }
    TabsContent { value: '2',
      'Ut enim ad minim veniam, quis nostrud exercitation ullamco laboris nisi ut aliquip ex ea commodo consequat.'
    }
    TabsContent { value: '3',
      'Duis aute irure dolor in reprehenderit in voluptate velit esse cillum dolore eu fugiat nulla pariatur.'
    }
    TabsContent { value: '4',
      'Excepteur sint occaecat cupidatat non proident, sunt in culpa qui officia deserunt mollit anim id est laborum.'
    }
  }	
}";

pub const EXAMPLE_ANATOMY: &str = "TabsRoot {
  TabsList {
    TabsTrigger {}
  }
  TabsContent {}
}";
