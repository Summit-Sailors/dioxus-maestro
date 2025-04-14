pub const EXAMPLE:&str = "use dioxus::prelude::*;
use maestro_headless::select::{OptionSelectedIndicator, SelectRoot, SelectDropdown, SelectIcon, SelectOption, SelectTrigger, SelectValue};
use maestro_headless::shared::{EAlign, EOrientation, ESide};

let mut selected = use_signal(Vec::new);

rsx! {
  SelectRoot {
    value: selected(),
    on_value_change: move |value: Vec<String>| { selected.set(value) },
    class: 'relative w-fit',
    SelectTrigger {
      class: 'rounded-sm border border-orange-400 bg-neutral-900 text-neutral-100 w-52 flex justify-between items-center gap-4 px-3 py-2 min-h-12 hover:text-neutral-100 focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-orange-600 focus-visible:ring-offset-2 focus-visible:ring-offset-neutral-900'
      SelectValue {
        placeholder: 'Chose something...',
        class: 'data-[state=selected]:text-neutral-100 data-[state=placeholder]:text-neutral-500 overflow-ellipsis',
      }
      SelectIcon {}
    }
    SelectDropdown {
      side: ESide::Bottom,
      side_offset: 10.0,
      class: 'rounded bg-neutral-900 text-neutral-200 border border-neutral-700 z-10 px-2 py-4 [&_*]:transition-all w-60',
      SelectOption {
        key: 1,
        value: 'apple',
        class: 'flex items-center justify-between gap-4 px-2 py-3 hover:bg-neutral-700 focus-visible:outline-none focus-visible:bg-neutral-700 data-[disabled=true]:opacity-50 data-[disabled=true]:pointer-events-none data-[disabled=true]:cursor-auto',
        'Apple'
        OptionSelectedIndicator { class: 'w-4 h-4' }
      }
      SelectOption {
        key: 2,
        value: 'banana',
        class: 'flex items-center justify-between gap-4 px-2 py-3 hover:bg-neutral-700 focus-visible:outline-none focus-visible:bg-neutral-700 data-[disabled=true]:opacity-50 data-[disabled=true]:pointer-events-none data-[disabled=true]:cursor-auto',
        'Banana'
        OptionSelectedIndicator { class: 'w-4 h-4' }
			}
      SelectOption {
        key: 3,
        value: 'ice-cream',
        class: 'flex items-center justify-between gap-4 px-2 py-3 hover:bg-neutral-700 focus-visible:outline-none focus-visible:bg-neutral-700 data-[disabled=true]:opacity-50 data-[disabled=true]:pointer-events-none data-[disabled=true]:cursor-auto',
        'Ice-Cream'
        OptionSelectedIndicator { class: 'w-4 h-4' }
      }
      SelectOption {
        key: 4,
        value: 'coffee',
        class: 'flex items-center justify-between gap-4 px-2 py-3 hover:bg-neutral-700 focus-visible:outline-none focus-visible:bg-neutral-700 data-[disabled=true]:opacity-50 data-[disabled=true]:pointer-events-none data-[disabled=true]:cursor-auto',
        'Coffee'
        OptionSelectedIndicator { class: 'w-4 h-4' }
      }
      SelectOption {
        key: 5,
        value: 'salt',
        disabled: true,
        class: 'flex items-center justify-between gap-4 px-2 py-3 hover:bg-neutral-700 focus-visible:outline-none focus-visible:bg-neutral-700 data-[disabled=true]:opacity-50 data-[disabled=true]:pointer-events-none data-[disabled=true]:cursor-auto',
        'Salt'
        OptionSelectedIndicator { class: 'w-4 h-4' }
      }
      SelectOption {
        key: 6,
        value: 'chocolatte',
        class: 'flex items-center justify-between gap-4 px-2 py-3 hover:bg-neutral-700 focus-visible:outline-none focus-visible:bg-neutral-700 data-[disabled=true]:opacity-50 data-[disabled=true]:pointer-events-none data-[disabled=true]:cursor-auto',
        'Chocolatte'
        OptionSelectedIndicator { class: 'w-4 h-4' }
      }
    }
  }		
}";

pub const EXAMPLE_ANATOMY: &str = "SelectRoot {
  SelectTrigger {
    SelectValue {}
    SelectIcon {}
  }
  SelectDropdown {
    SelectOption {
      OptionSelectedIndicator {}
    }
  }
}";
