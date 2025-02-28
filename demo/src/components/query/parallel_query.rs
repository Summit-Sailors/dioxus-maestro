use {
	dioxus::prelude::*,
	maestro_query::prelude::*,
	maestro_ui::button::Button,
	serde::{Deserialize, Serialize},
	std::fmt::Error,
	tailwind_fuse::tw_join,
};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
struct Department {
	id: String,
	name: String,
	budget: f64,
	location: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
struct Employee {
	id: String,
	name: String,
	title: String,
	department_id: String,
	salary: f64,
}

#[component]
fn DepartmentContextProvider(children: Element) -> Element {
	let _: UseQueryClient<Vec<Department>, Error, String> = use_init_query_client();

	rsx! {
    EmployeeContextProvider { children }
  }
}

#[component]
fn EmployeeContextProvider(children: Element) -> Element {
	let _: UseQueryClient<Vec<Employee>, Error, String> = use_init_query_client();

	rsx! {
    {children}
  }
}

// main wrapper component
#[component]
pub fn ParallelQueriesWrapper() -> Element {
	rsx! {
    div { class: "w-full",
      DepartmentContextProvider { ParallelQueriesDemo {} }
    }
  }
}

#[component]
pub fn ParallelQueriesDemo() -> Element {
	let query_client: UseQueryClient<Vec<Department>, Error, String> = use_query_client();
	let mut selected_dept = use_signal(|| None::<String>);

	// Simulated departments data
	let departments_query = use_get_query([String::from("departments")], |_| async move {
		async_std::task::sleep(std::time::Duration::from_millis(1000)).await;
		QueryResult::<Vec<Department>, Error>::Ok(vec![
			Department { id: "eng".to_string(), name: "Engineering".to_string(), budget: 1_000_000.0, location: "Location 1".to_string() },
			Department { id: "mkt".to_string(), name: "Marketing".to_string(), budget: 500_000.0, location: "Location 2".to_string() },
			Department { id: "sal".to_string(), name: "Sales".to_string(), budget: 750_000.0, location: "Location 3".to_string() },
		])
	});

	// employees query with department dependency
	let employees_query = use_get_query([String::from("employees"), selected_dept().unwrap_or_default()], move |_| async move {
		async_std::task::sleep(std::time::Duration::from_millis(500)).await;
		let dept_id = selected_dept().unwrap_or_default();
		let employees = match dept_id.as_str() {
			"eng" => vec![
				Employee {
					id: "1".to_string(),
					name: "Stephen O.".to_string(),
					title: "Senior Engineer".to_string(),
					department_id: "eng".to_string(),
					salary: 150000.0,
				},
				Employee { id: "2".to_string(), name: "Jane M.".to_string(), title: "Tech Lead".to_string(), department_id: "eng".to_string(), salary: 180000.0 },
			],
			"mkt" => vec![Employee {
				id: "3".to_string(),
				name: "Alice J.".to_string(),
				title: "Marketing Manager".to_string(),
				department_id: "mkt".to_string(),
				salary: 120000.0,
			}],
			"sal" => vec![Employee {
				id: "4".to_string(),
				name: "Bob W".to_string(),
				title: "Sales Director".to_string(),
				department_id: "sal".to_string(),
				salary: 160000.0,
			}],
			_ => vec![],
		};
		QueryResult::<Vec<Employee>, Error>::Ok(employees)
	});

	rsx! {
    div { class: "flex flex-col items-center justify-center rounded-lg shadow-lg p-6 bg-gray-900",
      div { class: "w-full max-w-4xl bg-gray-900 rounded-lg shadow-lg p-6",
        h3 { class: "text-2xl text-gray-100 text-center font-bold mb-6", "Parallel Queries" }

        // department overview section
        div { class: "space-y-6",
          h2 { class: "text-xl font-bold text-center text-gray-200", "Department Overview" }
          div { class: "grid grid-cols-1 md:grid-cols-3 gap-4 text-center",
            match departments_query.result().value() {
                QueryResult::Loading(_) => rsx! {
                  div { class: "col-span-3 text-gray-500", "Loading departments..." }
                },
                QueryResult::Ok(deps) => rsx! {
                  {
                      deps.iter()
                          .map(|dept| {
                              let dept = dept.clone();
                              let is_selected = selected_dept() == Some(dept.id.clone());
                              rsx! {
                                div {
                                  key: dept.id.clone(),
                                  class: tw_join!(
                                      "p-4 rounded-lg border border-gray-800 transition-colors cursor-pointer", if
                                      is_selected { "border-blue-500" } else { "bg-gray-800" }
                                  ),
                                  onclick: move |_| selected_dept.set(Some(dept.id.clone())),
                                
                                  h3 { class: "font-semibold text-lg", "{dept.name}" }
                                  p { class: "text-sm text-gray-300", "Location: {dept.location}" }
                                  p { class: "text-sm text-gray-300",
                                    "Budget: "
                                    span { class: "font-medium", {format!("${:.2}", dept.budget)} }
                                  }
                                }
                              }
                          })
                  }
                },
                QueryResult::Err(_) => rsx! {
                  div { class: "col-span-3 text-center text-red-500", "Error loading departments" }
                },
            }
          }
        }

        // department employees section
        div { class: "space-y-6",
          h2 { class: "text-xl font-bold text-center text-gray-200", "Department Employees" }

          if selected_dept().is_some() {
            div { class: "grid grid-cols-1 md:grid-cols-2 gap-4",

              match employees_query.result().value() {
                  QueryResult::Loading(_) => rsx! {
                    div { class: "col-span-2 text-center text-gray-500", "Loading employees..." }
                  },
                  QueryResult::Ok(emps) => rsx! {
                    {
                        emps.iter()
                            .map(|emp| {
                                let emp = emp.clone();
                                rsx! {
                                  div {
                                    key: emp.id.clone(),
                                    class: "p-4 bg-gray-800 rounded-lg border border-gray-800 shadow-sm",
                                    div { class: "space-y-2 text-center",
                                      h4 { class: "font-semibold text-lg", "{emp.name}" }
                                      p { class: "text-sm text-gray-300", "{emp.title}" }
                                      p { class: "text-sm text-gray-300",
                                        "Salary: "
                                        span { class: "font-medium", {format!("${:.2}", emp.salary)} }
                                      }
                                    }
                                  }
                                }
                            })
                    }
                  },
                  QueryResult::Err(_) => rsx! {
                    div { class: "col-span-2 text-center text-red-500", "Error loading employees" }
                  },
              }
            }
          } else {
            div { class: "text-center text-gray-300",
              "Select a department to view employees"
            }
          }
        }

        div { class: "flex justify-center gap-4 mt-6",
          Button {
            class: "bg-blue-500 text-white px-4 py-2 rounded-lg hover:bg-blue-600",
            onclick: move |_| {
                query_client
                    .invalidate_queries(
                        &[String::from("departments"), String::from("employees")],
                    );
            },
            "Refresh All"
          }
          if let Some(dept) = selected_dept() {
            Button {
              class: "bg-green-500 text-white px-4 py-2 rounded-lg hover:bg-green-600",
              onclick: move |_| query_client.invalidate_queries(&[String::from("employees"), dept.clone()]),
              "Refresh Employees"
            }
          }
        }
      }
    }
  }
}
