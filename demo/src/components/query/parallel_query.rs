use {
  dioxus::prelude::*,
  maestro_query::prelude::*,
  maestro_ui::button::Button,
  std::fmt::Error
};

#[component]
pub fn ParallelQueriesDemo() -> Element {
  let query_client: UseQueryClient<Vec<String>, Error, String> = use_init_query_client();
  
  // first parallel query
  let departments_query = use_get_query([String::from("departments")], |_| async move {
    async_std::task::sleep(std::time::Duration::from_millis(500)).await;
    QueryResult::<Vec<std::string::String>, Error>::Ok(vec!["Engineering".to_string(), "Marketing".to_string()])
  });

  // second parallel query depending on departments
  let department_names = match departments_query.result().value() {
    QueryResult::Ok(deps) => deps.clone(),
    _ => Vec::new(),
  };

  let employees_query = use_get_query(
    [String::from("employees"), department_names.join(",")],
    |_| async move {
      async_std::task::sleep(std::time::Duration::from_millis(500)).await;
      QueryResult::<Vec<std::string::String>, Error>::Ok(vec!["John".to_string(), "Jane".to_string()])
    }
  );

  rsx! {
    div { class: "grid justify-center bg-white p-6 border rounded-lg shadow-md mt-6",
      div { class: "w-full max-w-4xl",
        h3 { 
          class: "text-xl font-bold text-gray-800 text-center mb-6", 
          "Parallel Queries with Dependencies" 
        }
        
        div { 
          class: "grid flex grid-cols-2 justify-center gap-6",
            // departments Section
            div { 
              class: "p-5 bg-gray-200 border text-center border-gray-100 rounded-lg shadow-md",
              h4 { class: "text-lg font-semibold text-gray-700 mb-3", "Departments" }
              
              match departments_query.result().value() {
                QueryResult::Loading(_) => rsx!{ 
                  p { class: "text-yellow-500 italic", "Loading departments..." } 
                },
                QueryResult::Ok(deps) => rsx!{
                  ul { 
                    class: "list-disc list-inside text-gray-600",
                    {deps.iter().map(|dep| rsx!(
                      li { key: "{dep}", class: "py-1", "{dep}" }
                    ))}
                  }
                },
                QueryResult::Err(_) => rsx!{ 
                  p { class: "text-red-500 font-semibold", "Error loading departments" } 
                }
              }
            }

            // employees Section
            div { class: "p-5 bg-gray-200 border text-center border-gray-100 rounded-lg shadow-md",
              h4 { class: "text-lg font-semibold text-gray-700 mb-3", "Employees" }
              
              match employees_query.result().value() {
                QueryResult::Loading(_) => rsx!{ 
                  p { class: "text-yellow-500 italic", "Loading employees..." } 
                },
                QueryResult::Ok(emps) => rsx!{
                  ul { class: "list-disc list-inside text-gray-600",
                    {emps.iter().map(|emp| rsx!(
                      li { key: "{emp}", class: "py-1", "{emp}" }
                    ))}
                  }
                },
                QueryResult::Err(_) => rsx!{ 
                  p { class: "text-red-500 font-semibold text-center", "Error loading employees" } 
                }
              }
            }
        }

        // refresh Button
        div { class: "flex justify-center mt-6",
          Button {
            class: "bg-blue-600 hover:bg-blue-700 text-white font-semibold p-4 px-5 py-2 rounded-lg transition duration-200",
            on_click: move |_| query_client.invalidate_query(String::from("departments")),
            "Refresh Departments"
          }
        }
      }
    }
  }
}
