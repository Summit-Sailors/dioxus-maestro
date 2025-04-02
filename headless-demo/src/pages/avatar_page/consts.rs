pub const EXAMPLE: &str = "use dioxus::prelude::*;
use maestro_headless::avatar::{{Avatar, AvatarFallback, AvatarImage}};

rsx! {{
  Avatar {{
    class: 'rounded-full flex justify-center items-center w-10 h-10 border border-neutral-50 overflow-hidden',
    AvatarImage {{
      src: 'https://mis.dp.ua/wp-content/uploads/2023/07/pqma3f-c17x11x50px50p-c17x11x50px50p-15f939eddf2b09f7e6c097aad232da37.jpg',
      class: 'object-cover size-full',
    }}
    AvatarFallback {{
      class: 'text-sm font-medium text-neutral-50',
      'NA'
    }}
  }}
    
  Avatar {{
    class: 'rounded-full flex justify-center items-center w-10 h-10 border border-neutral-50 overflow-hidden',
    AvatarFallback {{
      class: 'text-sm font-medium text-neutral-50',
      delay_ms: 300,
      'NA'
    }}
  }}
}}";

pub const EXAMPLE_ANATOMY: &str = "Avatar {{
  AvatarImage {{  }}
  AvatarFallback {{  }}
}}";
