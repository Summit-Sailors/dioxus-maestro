pub const EXAMPLE_GROUP:&str = "use dioxus::prelude::*;
use maestro_headless::checkbox::CheckboxIndicator;
use maestro_headless::checkbox_group::{{CheckboxGroup, CheckboxGroupItem, CheckboxGroupItemIndicator}}

rsx! {{
  div {{ 
  class: 'p-6 flex flex-col gap-4 items-start',
    CheckboxGroup {{
      class: 'flex items-center gap-4 md:flex-row flex-col',
      value: values(),
      on_value_change: move |v: Vec<String>| {{
        values.set(v);
      }},
      name: 'favorites',
      div {{ 
        class: 'flex justify-center items-center gap-3',
        CheckboxGroupItem {{
          id: 'chocolate',
          class: 'w-6 h-6 rounded flex items-center justify-center border border-neutral-100 transition-colors hover:border-orange-600 focus-visible:ring-2 ring-orange-600 ring-offset-neutral-950 focus-visible:ring-offset-2 focus-visible:outline-none disabled:opacity-50',
          value: 'chocolate',
          CheckboxGroupItemIndicator {{}}
        }}
        label {{
          class: 'text-slate-100',
          r#for: 'chocolate',
          'Chocolate'
        }}
      }}
      div {{ 
        class: 'flex justify-center items-center gap-3',
        CheckboxGroupItem {{
          id: 'banana',
          disabled: true,
          class: 'w-6 h-6 rounded flex items-center justify-center border border-neutral-100 transition-colors hover:border-orange-600 focus-visible:ring-2 ring-orange-600 ring-offset-neutral-950 focus-visible:ring-offset-2 focus-visible:outline-none disabled:opacity-50',
          value: 'banana',
          CheckboxGroupItemIndicator {{}}
        }}
        label {{
          class: 'text-slate-100',
          r#for: 'banana',
          'Banana'
        }}
      }}
      div {{ 
        class: 'flex justify-center items-center gap-3',
        CheckboxGroupItem {{
          id: 'coffee',
          disabled: true,
          class: 'w-6 h-6 rounded flex items-center justify-center border border-neutral-100 transition-colors hover:border-orange-600 focus-visible:ring-2 ring-orange-600 ring-offset-neutral-950 focus-visible:ring-offset-2 focus-visible:outline-none disabled:opacity-50',
          value: 'coffee',
          CheckboxGroupItemIndicator {{}}
        }}
        label {{
          class: 'text-slate-100',
          r#for: 'coffee',
          'Coffee'
        }}
      }}
      div {{ 
        class: 'flex justify-center items-center gap-3',
        CheckboxGroupItem {{
          id: 'ice-cream',
          disabled: true,
          class: 'w-6 h-6 rounded flex items-center justify-center border border-neutral-100 transition-colors hover:border-orange-600 focus-visible:ring-2 ring-orange-600 ring-offset-neutral-950 focus-visible:ring-offset-2 focus-visible:outline-none disabled:opacity-50',
          value: 'ice-cream',
          CheckboxGroupItemIndicator {{}}
        }}
        label {{
          class: 'text-slate-100',
          r#for: 'ice-cream',
          'Ice-cream'
        }}
      }}
    }}
    ul {{ 
      class: 'space-y-2 list-disc list-inside',
      for value in values().iter() {{
        li {{ '{{value}}' }}
      }}
    }}
  }}		
}}";

pub const EXAMPLE_GROUP_ANATOMY: &str = "CheckboxGroup {{
	CheckboxGroupItem {{ 
    CheckboxGroupItemIndicator {{}}
  }}
}}";

pub const EXAMPLE:&str = "use dioxus::prelude::*;
use maestro_headless::checkbox::{{Checkbox, CheckboxIndicator}};

rsx! {{
  div {{ 
    class: 'flex justify-center items-center gap-3',
    Checkbox {{
      id: 'maestro-box',
      class: 'w-6 h-6 rounded flex items-center justify-center border border-neutral-100 transition-colors hover:border-orange-600 focus-visible:ring-2 ring-orange-600 ring-offset-neutral-950 focus-visible:ring-offset-2 focus-visible:outline-none',
      value: 'some',
      name: 'box',
      CheckboxIndicator {{ class: 'text-neutral-100' }}
    }}
    label {{ 
      class: 'text-slate-100', r#for: 'maestro-box', 'Check Me' 
    }}
  }}
}}";

pub const EXAMPLE_ANATOMY: &str = "Checkbox {{
	CheckboxIndicator {{ }}
}}";
