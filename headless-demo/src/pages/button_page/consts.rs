pub const EXAMPLE:&str = "use dioxus::prelude::*;
use maestro_headless::button::Button;

let mut is_pending = use_signal(|| false);

rsx! {
  Button {
    pending: is_pending,,
    class: 'h-10 px-3 py-2 rounded-md flex items-center justify-center gap-3 transition-all ease-linear bg-neutral-900 text-neutral-100 border border-neutral-100 hover:border-orange-600 focus-visible:border-orange-600 focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-orange-600 focus-visible:ring-offset-2 focus-visible:ring-offset-neutral-900 disabled:opacity-50 data-[pending=true]:opacity-50',
    onclick: move |_| {
      is_pending.set(true);
      spawn(async move {
        sleep(Duration::from_millis(1000)).await;
        is_pending.set(false);
      });
    },
    if is_pending() {
      {
        rsx! {
          'Pending'
          Icon { icon: LdLoader, class: 'animate-spin ease-linear' }			
        }
      }
    } else {'Active'}
  }
}";
