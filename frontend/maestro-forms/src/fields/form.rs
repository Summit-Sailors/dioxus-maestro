use {
	crate::use_formik::{use_formik, Formik},
	dioxus::prelude::*,
	serde::{Deserialize, Serialize},
	validator::Validate,
};

#[derive(Clone, PartialEq, Props)]
pub struct FormProps<T>
where
	T: Validate + Clone + Serialize + PartialEq + 'static + for<'de> Deserialize<'de>,
{
	pub initial_value: T,
	#[props(optional)]
	pub onsubmit: Option<EventHandler<FormEvent>>,
	#[props(extends = GlobalAttributes, extends = form)]
	pub attributes: Vec<Attribute>,
	children_maker: Component<InnerComponentProps<T>>,
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
	let form = use_formik::<T>(props.initial_value);
	let InnerComponent = props.children_maker;
	rsx! {
		form {..props.attributes,
			InnerComponent { form }
		}
	}
}
