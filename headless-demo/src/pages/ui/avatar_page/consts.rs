pub const EXAMPLE: &str = "use dioxus::prelude::*;
use maestro_ui::avatar::{Avatar, AvatarFallback, AvatarImage, AvatarSize};

rsx! {
  Avatar {
    size: AvatarSize::Md,
    AvatarImage {
      src: 'https://mis.dp.ua/wp-content/uploads/2023/07/pqma3f-c17x11x50px50p-c17x11x50px50p-15f939eddf2b09f7e6c097aad232da37.jpg',
    }
    AvatarFallback {
      'NA'
    }
  }
    
  AvatarRoot {
    AvatarFallback {
      delay_ms: 300,
      'NA'
    }
  }
}";

pub const EXAMPLE_ANATOMY: &str = "Avatar {
  AvatarImage {  }
  AvatarFallback {  }
}";
