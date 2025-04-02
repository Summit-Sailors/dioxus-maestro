pub const EXAMPLE:&str = "use dioxus::prelude::*;
use maestro_headless::accordion::{{Accordion, AccordionContent, AccordionHeader, AccordionItem, AccordionTrigger, AccordionVariant}};

rsx! {{
  Accordion {{
    default_value: Vec::from(['1'.into()]),
    class: 'relative w-full max-w-96 flex flex-col rounded-sm bg-neutral-900 text-neutral-100 p-0.5 transition-all ease-linear overflow-hidden',
    variant: AccordionVariant::Single,
    AccordionItem {{
      value: '1',
      class: 'flex flex-col data-[state=open]:gap-3 data-[disabled=true]:opacity-50 data-[disabled=true]:pointer-events-none border-b border-b-neutral-500',
      AccordionHeader {{
        AccordionTrigger {{ 
          class: 'px-4 py-2 h-full w-full hover:bg-neutral-800 data-[state=open]:border-b border-b-neutral-700 data-[state=open]:text-orange-600 transition-all ease-linear focus-visible:ring-2 focus-visible:ring-orange-600 focus-visible:outline-none line-clamp-1',
          'Default opened'
        }}
      }}
      AccordionContent {{ 
        class: 'flex overflow-hidden data-[state=open]:h-fit data-[state=closed]:h-0 transition-all ease-linear px-4 data-[state=open]:py-2',
        'Lorem ipsum dolor sit amet, consectetur adipiscing elit, sed do eiusmod tempor incididunt ut labore et dolore magna aliqua. Ut enim ad minim veniam, quis nostrud exercitation ullamco laboris nisi ut aliquip ex ea commodo consequat. Duis aute irure dolor in reprehenderit in voluptate velit esse cillum dolore eu fugiat nulla pariatur. Excepteur sint occaecat cupidatat non proident, sunt in culpa qui officia deserunt mollit anim id est laborum.'
      }}
    }}
    AccordionItem {{
      value: '2',
      class: 'flex flex-col data-[state=open]:gap-3 data-[disabled=true]:opacity-50 data-[disabled=true]:pointer-events-none border-b border-b-neutral-500',
      AccordionHeader {{ 
        AccordionTrigger {{
          class:'px-4 py-2 h-full w-full hover:bg-neutral-800 data-[state=open]:border-b border-b-neutral-700 data-[state=open]:text-orange-600 transition-all ease-linear focus-visible:ring-2 focus-visible:ring-orange-600 focus-visible:outline-none line-clamp-1',
          'Sed ut perspiciatis unde...'
        }}
      }}
      AccordionContent {{
        class: 'flex overflow-hidden data-[state=open]:h-fit data-[state=closed]:h-0 transition-all ease-linear px-4 data-[state=open]:py-2',
        'Sed ut perspiciatis unde omnis iste natus error sit voluptatem accusantium doloremque laudantium, totam rem aperiam, eaque ipsa quae ab illo inventore
        veritatis et quasi architecto beatae vitae dicta sunt explicabo. Nemo enim ipsam voluptatem quia voluptas sit aspernatur aut odit aut fugit, sed quia
        consequuntur magni dolores eos qui ratione voluptatem sequi nesciunt.'
      }}
    }}
    AccordionItem {{
      value: '3',
      disabled: true,
      class: 'flex flex-col data-[state=open]:gap-3 data-[disabled=true]:opacity-50 data-[disabled=true]:pointer-events-none border-b border-b-neutral-500',
      AccordionHeader {{ 
        AccordionTrigger {{
          class: 'px-4 py-2 h-full w-full hover:bg-neutral-800 data-[state=open]:border-b border-b-neutral-700 data-[state=open]:text-orange-600 transition-all ease-linear focus-visible:ring-2 focus-visible:ring-orange-600 focus-visible:outline-none line-clamp-1',
          'I\'m disabled :('
        }}
      }}
      AccordionContent {{
        class: 'flex overflow-hidden data-[state=open]:h-fit data-[state=closed]:h-0 transition-all ease-linear px-4 data-[state=open]:py-2',
        'Neque porro quisquam est, qui dolorem ipsum quia dolor sit amet, consectetur, adipisci velit, sed quia non numquam eius modi tempora incidunt ut labore et dolore magnam aliquam quaerat voluptatem.'
      }}
    }}
    AccordionItem {{
      value: '4',
      class: 'flex flex-col data-[state=open]:gap-3 data-[disabled=true]:opacity-50 data-[disabled=true]:pointer-events-none border-b border-b-neutral-500',
      AccordionHeader {{ 
        AccordionTrigger {{
          class: 'px-4 py-2 h-full w-full hover:bg-neutral-800 data-[state=open]:border-b border-b-neutral-700 data-[state=open]:text-orange-600 transition-all ease-linear focus-visible:ring-2 focus-visible:ring-orange-600 focus-visible:outline-none line-clamp-1',
          'Ut enim ad minima veniam'
        }}
      }}
      AccordionContent {{
        class: 'flex overflow-hidden data-[state=open]:h-fit data-[state=closed]:h-0 transition-all ease-linear px-4 data-[state=open]:py-2',
        'Ut enim ad minima veniam, quis nostrum exercitationem ullam corporis suscipit laboriosam, nisi ut aliquid ex ea commodi consequatur?'
      }}
    }}  
  }}
}}";

pub const EXAMPLE_ANATOMY: &str = "Accordion {{
  AccordionItem {{
    AccordionTrigger {{
      AccordionHeader {{  }}
    }}
    AccordionContent {{  }}
  }}
}}";
