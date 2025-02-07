use {
  crate::router::Route, 
  dioxus::prelude::*, 
  dioxus_free_icons::{icons::fa_solid_icons::{FaBars, FaX}, Icon}, 
  maestro_toast::{init::use_init_toast_ctx, toast_frame_component::ToastFrame}, 
  strum::IntoEnumIterator, 
  tailwind_fuse::tw_join
};

#[component]
pub fn Layout(children: Element) -> Element {
  let toast = use_init_toast_ctx();
  let mut menu_open = use_signal(|| false);

  rsx! {
    head {
      link { rel: "stylesheet", href: asset!("/assets/main.css") }
      link { rel: "icon", href: asset!("/assets/favicon.ico") }
    }

    ToastFrame { manager: toast }

    button {
      class: tw_join!(
        "lg:hidden flex top-0 left-0 bg-gray-800 text-white p-2 rounded-md z-50 items-center"
      ),
      onclick: move |_| menu_open.set(!menu_open()),
      Icon {
        width: 30,
        height: 30,
        fill: "white",
        icon: FaBars
      }
    }

    div {
      class: tw_join!(
        "grid lg:grid-cols-7 md:grid-cols-5 sm:grid-cols-1 min-h-screen bg-gray-50 text-gray-800"
      ),

      div {
        class: if menu_open() {
          tw_join!(
            "fixed top-0 left-0 w-full z-40 bg-gray-800 text-white h-full overflow-y-auto sm:block"
          )
        } else {
          tw_join!(
            "hidden lg:block lg:relative"
          )
        },
        NavigationMenu { close_menu: menu_open.clone() }
      }

      div {
        class: tw_join!(
          "lg:col-span-6 md:col-span-4 sm:col-span-1 p-4 sm:p-6 bg-white shadow-lg overflow-y-auto w-full absolute"
        ),
        Outlet::<Route> {}
      }      
    }
  }
}

#[component]
fn NavigationMenu(close_menu: Signal<bool>) -> Element {
  let current_route = use_route::<Route>();

  rsx! {
    div {
      class: tw_join!(
        "w-full lg:w-64 p-6 bg-gray-800 text-white flex flex-col space-y-4 fixed min-h-screen"
      ),

      button {
        class: "lg:hidden top-0 right-0 bg-red-500 text-white p-2 rounded-md",
        onclick: move |_| close_menu.set(false),
        Icon::<FaX> {
          width: 30,
          height: 30,
          fill: "white",
          icon: FaX,
        }
      }

      {Route::iter()
        .filter(|route| *route != current_route)
        .map(|route| {
          let route_name = route.name();
          rsx! {
            Link {
              to: route.clone(),
              class: tw_join!(
                "py-2 px-4 rounded-md hover:bg-gray-700 transition w-full text-left text-white"
              ),
              aria_label: route_name,
              "{route_name}"
            }
          }
        })
      }
    }
  }
}
