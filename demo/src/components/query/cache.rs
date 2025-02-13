use {
  dioxus::prelude::*, 
  maestro_query::prelude::*, 
  maestro_ui::button::Button, 
  std::fmt::Error
};


#[component]
pub fn CacheDemo() -> Element {
  let query_client: UseQueryClient<String, Error, String> = use_init_query_client();
  let mut stale_time = use_signal(|| 100u64);

  let cached_query = use_get_query([String::from("cached-data")], |_| async move {
    async_std::task::sleep(std::time::Duration::from_secs(2)).await;
    QueryResult::Ok::<String, Error>("This data is cached!".to_string())
  });

  let force_refetch = move |_| { 
    query_client.invalidate_query(String::from("cached-data"));  
  };

  rsx! {
    div { class: "grid flex justify-center p-4 border bg-white shadow-md rounded mt-4",
      h3 { class: "text-xl text-gray-800 text-center font-bold mb-4", "Cache Demonstration" }

      div {  
        class: "mb-4",
        label {  
          class: "block text-sm text-center font-medium text-gray-700", 
          "Stale Time (ms)"
        }
        input {  
          r#type: "range",
          min: "100",
          max: "10000",
          value: "{stale_time}",
          onchange: move |e| stale_time.set(e.value().parse().unwrap_or(100))
        }
        span { class: "ml-2 text-gray-700", "{stale_time}ms" }
      }
      
      div { 
        class: "mb-4 text-center font-bold",
        p {
          class: "text-gray-700",
          "Cache Status: ",
          if cached_query.result().is_fresh() {
            span { class: "text-green-500", "Fresh" }
          } else {
            span { class: "text-yellow-500", "Stale" }
          }
        }
        p {
          class: "text-gray-700",
          "Query Status: ",
          if cached_query.result().is_loading() {
            span { class: "text-blue-500", "Loading..." }
          } else {
            span { class: "text-green-500", "Ready" }
          }
        }
      }

      div { class: "mb-4 text-gray-700 font-bold text-center",
        match cached_query.result().value() {
          QueryResult::Loading(_) => rsx!{ span { class: "text-yellow-500", "Fetching data..." } },
          QueryResult::Ok(data) => rsx!{ "Data:", span{ class: "text-green-500","{data}" }},
          QueryResult::Err(e) => rsx!{ "Error:", span{ class: "text-red-500"," {e}" }}
        }
      }

      Button {
        class: "bg-blue-500 text-white px-4 py-2 rounded",
        on_click: force_refetch,
        "Force Refetch"
      }
    }
  }
}
