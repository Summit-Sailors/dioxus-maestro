use std::fmt::Display;

use maestro_toast::ctx::use_toast;
use plotters::{prelude::*, style::colors::colormaps};
use plotters_canvas::CanvasBackend;

use crate::chart_options::ChartOptions;

pub fn render_heatmap<Label: Display + Clone>(
	canvas_id: &str,
	data: Vec<Vec<f32>>,
	labels: Vec<Label>,
	options: ChartOptions,
) -> Result<(), Box<dyn std::error::Error>> {
	let root = CanvasBackend::new(canvas_id).expect("failed to create CanvasBackend").into_drawing_area();
	root.fill(&WHITE)?;

	let data_rows = data.len() as u32;
	let data_cols = data.first().map_or(0, |row| row.len() as u32);

	let mut chart = ChartBuilder::on(&root)
		.caption(&options.title, (options.font.0.as_str(), options.font.1).into_font())
		.margin(options.margin)
		.x_label_area_size(options.x_label_area_size)
		.y_label_area_size(options.y_label_area_size)
		.build_cartesian_2d(0..data_cols, 0..data_rows)?;

	chart
		.configure_mesh()
		.disable_x_mesh()
		.disable_y_mesh()
		.x_labels(data_cols as usize)
		.y_labels(data_rows as usize)
		.x_label_formatter(&|x| labels.get(*x as usize).map(|l| l.to_string()).unwrap_or_default())
		.y_label_formatter(&|y| labels.get(*y as usize).map(|l| l.to_string()).unwrap_or_default())
		.draw()?;

	for (y, row) in data.iter().enumerate() {
		for (x, &value) in row.iter().enumerate() {
			chart.draw_series(std::iter::once(Rectangle::new(
				[(x as u32, y as u32), ((x + 1) as u32, (y + 1) as u32)],
				colormaps::ViridisRGB::get_color(value).filled(),
			)))?;
		}
	}
	root.present()?;
	Ok(())
}

use dioxus::prelude::*;
use dioxus_logger::tracing::info;
use maestro_toast::toast_info::ToastInfo;

pub fn use_heat_map<Label: Display + Clone + PartialEq>(
	canvas_id: String,
	data: Memo<Option<Vec<Vec<f32>>>>,
	labels: Memo<Option<Vec<Label>>>,
	options: ChartOptions,
) {
	let mut toast = use_toast();
	use_effect(move || {
		if let (Some(data), Some(labels)) = (data(), labels()) {
			if let Err(e) = render_heatmap(canvas_id.as_str(), data.clone(), labels.clone(), options.clone()) {
				info!("{e}");
				toast.write().popup(ToastInfo::builder().context(e.to_string()).build());
			}
		}
	});
}
