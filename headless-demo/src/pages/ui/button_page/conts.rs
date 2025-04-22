pub const EXAMPLE: &str = "use dioxus::prelude::*;
maestro_ui::button::{Button, ButtonRound, ESize, ButtonVariant};

let mut is_pending = use_signal(|| false);

rsx! {
  Button {
    pending: is_pending,
    variant: ButtonVariant::Primary,
    round: ButtonRound::Md,
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
