pub const EXAMPLE:&str = "use dioxus::prelude::*;
use maestro_headless::tabs::{TabsRoot, TabsContent, TabsList, TabsTrigger};

rsx! {
  TabsRoot { default_value: '1', class: 'flex data-[orientation=vertical]:flex-row flex-col gap-4',
    TabsList { 
    class: 'w-full flex items-center data-[orientation=horizontal]:gap-6 data-[orientation=vertical]:flex-col',
      TabsTrigger {
        value: '1',
        class: 'data-[disabled=true]:opacity-50 data-[disabled=true]:pointer-events-none py-2 data-[orientation=vertical]:pr-3 font-medium text-neutral-300 data-[orientation=horizontal]:border-b data-[orientation=vertical]:border-r data-[orientation=horizontal]:border-b-transparent data-[orientation=vertical]:border-r-transparent data-[orientation=horizontal]:hover:border-b-orange-300 data-[orientation=vertical]:hover:border-r-orange-300 data-[orientation=horizontal]:data-[state=active]:border-b-orange-600 data-[orientation=vertical]:data-[state=active]:border-r-orange-600 data-[state=active]:text-neutral-100 transition-all ease-linear focus-visible:outline-none data-[orientation=horizontal]:data-[state=active]:focus-visible:border-b-neutral-100 data-[orientation=vertical]:data-[state=active]:focus-visible:border-r-neutral-100 focus-visible:text-neutral-100',
        'One'
      }
      TabsTrigger {
        value: '2',
        class: 'data-[disabled=true]:opacity-50 data-[disabled=true]:pointer-events-none py-2 data-[orientation=vertical]:pr-3 font-medium text-neutral-300 data-[orientation=horizontal]:border-b data-[orientation=vertical]:border-r data-[orientation=horizontal]:border-b-transparent data-[orientation=vertical]:border-r-transparent data-[orientation=horizontal]:hover:border-b-orange-300 data-[orientation=vertical]:hover:border-r-orange-300 data-[orientation=horizontal]:data-[state=active]:border-b-orange-600 data-[orientation=vertical]:data-[state=active]:border-r-orange-600 data-[state=active]:text-neutral-100 transition-all ease-linear focus-visible:outline-none data-[orientation=horizontal]:data-[state=active]:focus-visible:border-b-neutral-100 data-[orientation=vertical]:data-[state=active]:focus-visible:border-r-neutral-100 focus-visible:text-neutral-100',
        'Two'
      }
      TabsTrigger {
        value: '3',
        class: 'data-[disabled=true]:opacity-50 data-[disabled=true]:pointer-events-none py-2 data-[orientation=vertical]:pr-3 font-medium text-neutral-300 data-[orientation=horizontal]:border-b data-[orientation=vertical]:border-r data-[orientation=horizontal]:border-b-transparent data-[orientation=vertical]:border-r-transparent data-[orientation=horizontal]:hover:border-b-orange-300 data-[orientation=vertical]:hover:border-r-orange-300 data-[orientation=horizontal]:data-[state=active]:border-b-orange-600 data-[orientation=vertical]:data-[state=active]:border-r-orange-600 data-[state=active]:text-neutral-100 transition-all ease-linear focus-visible:outline-none data-[orientation=horizontal]:data-[state=active]:focus-visible:border-b-neutral-100 data-[orientation=vertical]:data-[state=active]:focus-visible:border-r-neutral-100 focus-visible:text-neutral-100',
        disabled: true,
        'Three'
      }
      TabsTrigger {
        value: '4',
        class: 'data-[disabled=true]:opacity-50 data-[disabled=true]:pointer-events-none py-2 data-[orientation=vertical]:pr-3 font-medium text-neutral-300 data-[orientation=horizontal]:border-b data-[orientation=vertical]:border-r data-[orientation=horizontal]:border-b-transparent data-[orientation=vertical]:border-r-transparent data-[orientation=horizontal]:hover:border-b-orange-300 data-[orientation=vertical]:hover:border-r-orange-300 data-[orientation=horizontal]:data-[state=active]:border-b-orange-600 data-[orientation=vertical]:data-[state=active]:border-r-orange-600 data-[state=active]:text-neutral-100 transition-all ease-linear focus-visible:outline-none data-[orientation=horizontal]:data-[state=active]:focus-visible:border-b-neutral-100 data-[orientation=vertical]:data-[state=active]:focus-visible:border-r-neutral-100 focus-visible:text-neutral-100',
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
