use {
	crate::chart_options::ChartOptions,
	plotters::{prelude::*, style::colors::colormaps},
	plotters_canvas::CanvasBackend,
	std::fmt::Display,
};

pub fn render_heatmap<Label: Display + Clone>(
	canvas_id: &str,
	data: Vec<Vec<f32>>,
	labels: Vec<Label>,
	options: ChartOptions,
) -> Result<(), Box<dyn std::error::Error>> {
	let root = CanvasBackend::new(canvas_id).expect("failed to create CanvasBackend").into_drawing_area();
	root.fill(&WHITE)?;
	let n = labels.len() as u32;
	let mut chart = ChartBuilder::on(&root)
		.caption(&options.title, (options.font.0.as_str(), options.font.1).into_font())
		.margin(options.margin)
		.x_label_area_size(options.x_label_area_size)
		.y_label_area_size(options.y_label_area_size)
		.build_cartesian_2d(0..n, 0..n)?;
	chart
		.configure_mesh()
		.disable_x_mesh()
		.disable_y_mesh()
		.x_labels(labels.len())
		.y_labels(labels.len())
		.x_label_formatter(&|x| labels[*x as usize].to_string())
		.y_label_formatter(&|y| labels[*y as usize].to_string())
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

use {
	dioxus::prelude::*,
	dioxus_logger::tracing::info,
	maestro_toast::{toast_info::ToastInfo, toast_manager::ToastManager},
};

pub fn use_heat_map<Label: Display + Clone + PartialEq>(
	canvas_id: String,
	data: Memo<Option<Vec<Vec<f32>>>>,
	labels: Memo<Option<Vec<Label>>>,
	options: ChartOptions,
) {
	let mut toast = use_context::<Signal<ToastManager>>();
	use_effect(move || {
		if let (Some(data), Some(labels)) = (data(), labels()) {
			if let Err(e) = render_heatmap(canvas_id.as_str(), data.clone(), labels.clone(), options.clone()) {
				info!("{e}");
				toast.write().popup(ToastInfo::builder().context(e.to_string()).build());
			}
		}
	});
}
