pub const EXAMPLE: &str = "use dioxus::prelude::*;
use maestro_ui::{
  accordion::{Accordion, AccordionContent, AccordionItem, AccordionTrigger, AccordionVariant},
  shared::EOrientation,
};

rsx! {
  Accordion {
    class: 'data-[orientation=horizontal]:h-24 h-full data-[orientation=vertical]:max-w-56',
    variant: AccordionVariant::Single,
    orientation: EOrientation::Vertical,
    for (key , title , text) in [
      ('1', 'Lorem ipsum', '...dolor sit amet'),
      ('2', 'consectetur adipiscing elit', '...sed do eiusmod tempor'),
      ('3', 'incididunt ut labore', '...et dolore magna aliqua'),
      ]
    {
      AccordionItem { key, value: key,
        AccordionTrigger { '{title}' }
        AccordionContent { class: '[&>div]:flex [&>div]:justify-center [&>div]:items-center',
          '{text}'
        }
      }
    }
  }
}";

pub const EXAMPLE_ANATOMY: &str = "Accordion {
  AccordionItem {
    AccordionTrigger {  }
    AccordionContent {  }
  }
}";
