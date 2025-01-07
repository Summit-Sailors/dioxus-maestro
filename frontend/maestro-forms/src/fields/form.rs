use {
	crate::use_formik::{use_init_form_ctx, Formik},
	dioxus::prelude::*,
	serde::{Deserialize, Serialize},
	validator::Validate,
};

#[derive(Clone, PartialEq, Props)]
pub struct FormProps<T>
where
	T: Validate + Clone + Serialize + PartialEq + 'static + for<'de> Deserialize<'de>,
{
	inner: Component<InnerComponentProps<T>>,
	pub initial_value: T,
	#[props(optional)]
	pub onsubmit: Option<EventHandler<FormEvent>>,
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
	let form = use_init_form_ctx::<T>(props.initial_value);
	let InnerComponent = props.inner;
	rsx! {
		form {..props.attributes,
			InnerComponent { form }
		}
	}
}
