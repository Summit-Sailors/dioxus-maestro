use maestro_toast::ctx::use_toast;
use plotters::{data::Quartiles, prelude::*};
use plotters_canvas::CanvasBackend;

use crate::chart_options::ChartOptions;

pub fn render_box_plot(canvas_id: &str, data: Vec<Vec<f32>>, options: ChartOptions) -> Result<(), Box<dyn std::error::Error>> {
	let root = CanvasBackend::new(canvas_id).expect("failed to create CanvasBackend").into_drawing_area();
	root.fill(&WHITE)?;
	let max_value = data.iter().flat_map(|v| v.iter()).fold(f32::NEG_INFINITY, |a, &b| a.max(b));
	let min_value = data.iter().flat_map(|v| v.iter()).fold(f32::INFINITY, |a, &b| a.min(b));
	let mut chart = ChartBuilder::on(&root)
		.caption(&options.title, (options.font.0.as_str(), options.font.1).into_font())
		.margin(options.margin)
		.x_label_area_size(options.x_label_area_size)
		.y_label_area_size(options.y_label_area_size)
		.build_cartesian_2d(0..data.len(), min_value..max_value)?;
	chart.configure_mesh().x_desc(&options.x_label).y_desc(&options.y_label).draw()?;
	for (i, series) in data.iter().enumerate() {
		let quartiles = Quartiles::new(series);
		chart.draw_series(std::iter::once(Boxplot::new_vertical(i, &quartiles).style(options.colors[i % options.colors.len()])))?;
	}
	root.present()?;
	Ok(())
}

use dioxus::prelude::*;
use dioxus_logger::tracing::info;
use maestro_toast::toast_info::ToastInfo;

pub fn use_box_plot(canvas_id: String, data: Memo<Option<Vec<Vec<f32>>>>, options: ChartOptions) {
	let mut toast = use_toast();
	use_effect(move || {
		if let Some(data) = data() {
			if let Err(e) = render_box_plot(canvas_id.as_str(), data.clone(), options.clone()) {
				info!("{e}");
				toast.write().popup(ToastInfo::builder().context(e.to_string()).build());
			}
		}
	});
}
