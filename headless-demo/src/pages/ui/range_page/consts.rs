pub const EXAMPLE:&str = "use dioxus::prelude::*;
use maestro_headless::range::{RangeRoot, RangeThumb, Range, RangeTrack};

let mut value = use_signal(|| Vec::from([0.0_f32]));

rsx! {
  RangeRoot {
    class: 'w-52 flex items-center',
    value: value(),
    on_value_change: move |v| { value.set(v) },
    RangeTrack { 
      class: 'flex-1 bg-neutral-600 rounded-full h-1',
      Range { class: 'flex-1 bg-orange-600 rounded-full h-1' }
    }
    RangeThumb {
      class: 'w-6 h-6 rounded-full bg-orange-600 flex items-center justify-center text-neutral-300 text-xs cursor-pointer transition-colors hover:text-neutral-100 focus-visible:outline-none focus-visible:ring-1 focus-visible:ring-orange-600 focus-visible:ring-offset-2 focus-visible:ring-offset-neutral-900'
      '{value().get(0).unwrap_or(&0.0):.0}'
    }
  }
}";

pub const EXAMPLE_ANATOMY: &str = "RangeRoot {
  RangeTrack {
    Range { }
  }
  RangeThumb { }
}";
