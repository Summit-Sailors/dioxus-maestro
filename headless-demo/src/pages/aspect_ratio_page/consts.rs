pub const EXAMPLE: &str = "use dioxus::prelude::*;
use maestro_headless::aspect_ratio::AspectRatioRoot;

rsx! {
  div { 
    class: 'w-64 overflow-hidden rounded-md',
    AspectRatioRoot {
      ratio: 16.0 / 9.0,
      img {
        class: 'size-full object-cover',
        src: 'https://ychef.files.bbci.co.uk/1280x720/p01x8qtv.jpg',
        alt: 'Ocean',
      }
    }
  }
}";
