pub const EXAMPLE: &str = "use dioxus::prelude::*;
use maestro_headless::separator::Separator;
use maestro_headless::shared::EOrientation;

rsx! {
  div { class: 'flex items-center flex-col text-neutral-100 gap-5',
    h3 { 'Hello, this is Maestro Headless lib!' }
    Separator { class: 'mx-4 bg-neutral-700 data-[orientation=horizontal]:h-px data-[orientation=vertical]:h-full data-[orientation=horizontal]:w-full data-[orientation=vertical]:w-px' }
    div { class: 'flex gap-4 h-5',
      span { 'Customizable' }
      Separator {
        orientation: EOrientation::Vertical,
        class: 'bg-neutral-700 data-[orientation=horizontal]:h-px data-[orientation=vertical]:h-full data-[orientation=horizontal]:w-full data-[orientation=vertical]:w-px',
      }
      span { 'Easy to use' }
      Separator {
        orientation: EOrientation::Vertical,
        class: 'bg-neutral-700 data-[orientation=horizontal]:h-px data-[orientation=vertical]:h-full data-[orientation=horizontal]:w-full data-[orientation=vertical]:w-px',
      }
      span { 'Flexible' }
    }
  }
}";
