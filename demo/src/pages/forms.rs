use {
  crate::{components::forms::form_content::FormContent, 
    models::user::{Role, User}}
  ,
  dioxus::prelude::*,
  maestro_forms::fields::
    form::{Form, FormResult}
  ,
  maestro_toast::{
    ctx::use_toast, toast_info::ToastInfo
  },
  tailwind_fuse::tw_join
};

async fn simulate_submission(delay_ms: u64) -> Result<(), String> {
  async_std::task::sleep(std::time::Duration::from_millis(delay_ms)).await;

  let success_rate = 95;
  let success = js_sys::Math::random() * 100.0 < success_rate as f64;

  if success {
    Ok(())
  } else {
    Err("server error".to_string())
  }
}

#[component]
pub fn FormsDemo() -> Element {
  let toast = use_toast();
  let mut is_async = use_signal(|| true);

  let on_submit = move |result: FormResult<User>| {
    to_owned![toast, is_async];
    async move {
      let (submitted_user, is_valid) = result;
      
      if !is_valid {
        toast.write().popup(
          ToastInfo::builder()
            .context("Form validation failed. Please check your inputs.".to_owned())
            .build(),
        );
        return;
      }

      let delay = if is_async() { 1000 } else { 500 };
      let result = simulate_submission(delay).await;

      match result {
        Ok(_) => {
          toast.write().popup(
            ToastInfo::builder()
              .context(format!(
                "Form submitted successfully for user: {:?}",
                submitted_user.username
              ))
              .build()
          );
        }
        Err(err) => {
          toast.write().popup(
            ToastInfo::builder()
              .context(format!("Submission failed: {}", err))
              .build(),
          );
        }
      }
    }
  };

  let mode_button_base = tw_join!(
    "px-4 py-2 rounded-md font-medium transition-all duration-200"
  );

  let async_class = tw_join!(
    mode_button_base.clone(),
    if is_async() { "bg-blue-500 text-white" } else { "bg-gray-200" }
  );

  let sync_class = tw_join!(
    mode_button_base,
    if !is_async() { "bg-blue-500 text-white" } else { "bg-gray-200" }
  );

  rsx! {
    div {
      class: "max-w-4xl mx-auto p-6",
      div {
        class: "mb-8",
        h1 {
          class: "text-3xl font-bold mb-2",
          "Maestro Forms Demo"
        }
        p {
          class: "text-gray-600",
          "A comprehensive demonstration of form handling with simulation mode."
        }

        div {
          class: "mt-4 space-x-2",
          button {
            class: "{async_class}",
            onclick: move |_| is_async.set(true),
            "Async Mode"
          }
          button {
            class: "{sync_class}",
            onclick: move |_| is_async.set(false),
            "Sync Mode"
          }
        }
      }

      Form {
        initial_values: User {
          role: Role::Admin,
          ..User::default()
        },
        onsubmit: on_submit,
        auto_reset: true,
        inner: FormContent,
      }
    }
  }
}
