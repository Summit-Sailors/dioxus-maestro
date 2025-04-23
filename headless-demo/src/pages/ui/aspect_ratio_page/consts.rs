pub const EXAMPLE: &str = "use dioxus::prelude::*;
use maestro_ui::aspect_ratio::AspectRatio;

rsx! {
  div { 
    class: 'w-64',
    AspectRatio {
      ratio: 16.0 / 9.0,
      img {
        class: 'size-full object-cover',
        src: 'https://ychef.files.bbci.co.uk/1280x720/p01x8qtv.jpg',
        alt: 'Ocean',
      }
    }
  }
}";
