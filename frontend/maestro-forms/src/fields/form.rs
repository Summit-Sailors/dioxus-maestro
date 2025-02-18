use {
	crate::use_formik::Formik,
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
  pub form: Formik<T>,
  inner: Component<InnerComponentProps<T>>,
  #[props(optional)]
  pub onsubmit: Option<EventHandler<(FormEvent, FormResult<T>, Box<dyn FnOnce()>)>>,
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

pub fn Form<T>(mut props: FormProps<T>) -> Element
where
  T: Validate + Clone + Serialize + PartialEq + 'static + for<'de> Deserialize<'de>,
{
  let InnerComponent = props.inner;

  props.form.should_auto_reset = props.auto_reset.unwrap_or(false);

  let onsubmit = move |e: FormEvent| {
    e.stop_propagation();
    if let Some(submit_handler) = props.onsubmit {
      props.form.submit(e, submit_handler);
    }
  };

  rsx! {
    form {
      onsubmit: onsubmit,
      ..props.attributes,
      InnerComponent { form: props.form }
  }
  }
}
