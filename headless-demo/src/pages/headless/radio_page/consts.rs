pub const EXAMPLE:&str = "use dioxus::prelude::*;
use maestro_headless::radio_group::{RadioGroupIndicator, RadioGroupItem, RadioGroupRoot};

rsx! {
  div { 
    class: 'p-6 flex grow items-center justify-center w-full',
    RadioGroupRoot {
      class: flex items-center gap-4,
      div { 
        class: 'flex items-center gap-2',
        RadioGroupItem {
          value: 'coffee',
          id: 'maestro-radio-1',
          class: 'w-6 h-6 rounded-full flex items-center justify-center transition-colors border border-orange-400 hover:border-orange-600 focus-visible:outline-none focus-visible:ring-1 focus-visible:ring-orange-400 focus-visible:ring-offset-2 focus-visible:ring-offset-neutral-900 disabled:opacity-50',
          RadioGroupIndicator { class: 'w-3 h-3 rounded-full bg-orange-600' }
        }
        label { r#for: 'maestro-radio-1', 'Coffee' }
      }
      div { 
        class: flex items-center gap-2,
        RadioGroupItem {
          value: water,
          id: maestro-radio-2,
          class: w-6 h-6 rounded-full flex items-center justify-center transition-colors border border-orange-400 hover:border-orange-600 focus-visible:outline-none focus-visible:ring-1 focus-visible:ring-orange-400 focus-visible:ring-offset-2 focus-visible:ring-offset-neutral-900 disabled:opacity-50,
          disabled: true,
          RadioGroupIndicator { class: w-3 h-3 rounded-full bg-orange-600 }
        }
        label { r#for: 'maestro-radio-2', 'Water' }
      }
      div { 
        class: 'flex items-center gap-2',
        RadioGroupItem {
          value: 'juice',
          id: 'maestro-radio-3',
          class: 'w-6 h-6 rounded-full flex items-center justify-center transition-colors border border-orange-400 hover:border-orange-600 focus-visible:outline-none focus-visible:ring-1 focus-visible:ring-orange-400 focus-visible:ring-offset-2 focus-visible:ring-offset-neutral-900 disabled:opacity-50',
          RadioGroupIndicator { class: 'w-3 h-3 rounded-full bg-orange-600' }
        }
      label { r#for: 'maestro-radio-3', 'Juice' }
      }
    }
  }
}";

pub const EXAMPLE_ANATOMY: &str = "RadioGroupRoot {
  RadioGroupItem {
    RadioGroupIndicator { }
  }
}";
