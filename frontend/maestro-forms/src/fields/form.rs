use {
	crate::use_formik::{use_init_form_ctx, Formik},
	dioxus::prelude::*,
	serde::{Deserialize, Serialize},
	validator::Validate,
};

pub type FormResult<T> = (T, bool);

#[derive(Clone, PartialEq, Props)]
pub struct FormProps<T>
where
  T: Validate + Clone + Serialize + PartialEq + 'static + for<'de> Deserialize<'de>,
{
  inner: Component<InnerComponentProps<T>>,
  pub initial_values: T,
  #[props(optional)]
  pub onsubmit: Option<EventHandler<(FormEvent, FormResult<T>)>>,
  #[props(optional)]
  pub auto_reset: Option<bool>,
  #[props(extends = GlobalAttributes, extends = form)]
  pub attributes: Vec<Attribute>,
}

#[derive(Clone, PartialEq, Props)]
pub struct InnerComponentProps<T>
where
  T: Validate + Clone + Serialize + PartialEq + 'static + for<'de> Deserialize<'de>,
{
  pub form: Formik<T>,
}

pub fn Form<T>(props: FormProps<T>) -> Element
where
  T: Validate + Clone + Serialize + PartialEq + 'static + for<'de> Deserialize<'de>,
{
  let mut form = use_init_form_ctx(props.initial_values.clone());
  let InnerComponent = props.inner;

  form.should_auto_reset = props.auto_reset.unwrap_or(false);

  let onsubmit = move |e: FormEvent| {
    e.stop_propagation();
    if let Some(submit_handler) = &props.onsubmit {
      form.submit(e, submit_handler);
    }
  };

  rsx! {
    form {
      onsubmit: onsubmit,
      ..props.attributes,
      InnerComponent { form }
    }
  }
}
