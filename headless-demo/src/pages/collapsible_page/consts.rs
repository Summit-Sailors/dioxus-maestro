pub const EXAMPLE:&str = "use dioxus::prelude::*;
use maestro_headless::collapsible::{{Collapsible, CollapsibleContent, CollapsibleTrigger}};

let mut is_open = use_signal(|| false);

rsx! {{
  div {{ class: 'p-6 flex gap-4 items-start max-w-96 w-full',
    Collapsible {{
      open: is_open(),
      on_open_change: move |value: bool| is_open.set(value),
      class: 'flex flex-col space-y-3 w-full',
      div {{
        class: 'flex justify-between items-center gap-3',
        span {{ class: 'text-neutral-100', 'Collapsible Component' }}
        CollapsibleTrigger {{ 
          class: 'h-8 rounded-md flex items-center justify-center px-3 py-2 border border-neutral-300 text-neutral-300 hover:text-neutral-100 hover:border-neutral-100 focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-neutral-300 focus-visible:ring-offset-2 focus-visible:ring-offset-neutral-900',
          if is_open() {{ 'Collapse' }} else {{ 'Expand' }}
        }}
        CollapsibleContent {{ 
          class: 'data-[state=closed]:animate-slide-out data-[state=open]:animate-slide-in',
          'Lorem ipsum dolor sit amet, consectetur adipiscing elit, sed do eiusmod tempor incididunt ut labore et dolore magna aliqua. Ut enim ad minim veniam, quis nostrud exercitation ullamco laboris nisi ut aliquip ex ea commodo consequat. Duis aute irure dolor in reprehenderit in voluptate velit esse cillum dolore eu fugiat nulla pariatur. Excepteur sint occaecat cupidatat non proident, sunt in culpa qui officia deserunt mollit anim id est laborum.'
        }}
      }} 
    }}
  }}
}}";

pub const EXAMPLE_ANATOMY: &str = "Collapsible {{
		CollapsibleTrigger {{ }}
		CollapsibleContent {{ }}
}}";
