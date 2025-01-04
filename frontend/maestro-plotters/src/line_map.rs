use {crate::chart_options::ChartOptions, plotters::prelude::*, plotters_canvas::CanvasBackend};

pub fn render_line_chart(canvas_id: &str, data: Vec<Vec<(f32, f32)>>, options: ChartOptions) -> Result<(), Box<dyn std::error::Error>> {
	let root = CanvasBackend::new(canvas_id).expect("failed to create CanvasBackend").into_drawing_area();
	root.fill(&WHITE)?;
	let (min_x, max_x) =
		data.iter().flat_map(|series| series.iter()).map(|&(x, _)| x).fold((f32::INFINITY, f32::NEG_INFINITY), |(min, max), x| (min.min(x), max.max(x)));
	let (min_y, max_y) =
		data.iter().flat_map(|series| series.iter()).map(|&(_, y)| y).fold((f32::INFINITY, f32::NEG_INFINITY), |(min, max), y| (min.min(y), max.max(y)));
	let mut chart = ChartBuilder::on(&root)
		.caption(&options.title, (options.font.0.as_str(), options.font.1).into_font())
		.margin(options.margin)
		.x_label_area_size(options.x_label_area_size)
		.y_label_area_size(options.y_label_area_size)
		.build_cartesian_2d(min_x..max_x, min_y..max_y)?;
	chart.configure_mesh().x_desc(&options.x_label).y_desc(&options.y_label).draw()?;
	for (i, series) in data.iter().enumerate() {
		let color = options.colors[i % options.colors.len()];
		let series_name = options.series_labels.get(i).cloned().unwrap_or_else(|| format!("Series {}", i + 1));
		chart
			.draw_series(LineSeries::new(series.iter().copied(), &color))?
			.label(&series_name)
			.legend(move |(x, y)| PathElement::new(vec![(x, y), ((x + 20), y)], color));
	}
	chart.configure_series_labels().background_style(WHITE.mix(0.8)).border_style(BLACK).draw()?;
	root.present()?;
	Ok(())
}

use {
	dioxus::prelude::*,
	dioxus_logger::tracing::info,
	maestro_toast::{toast_info::ToastInfo, toast_manager::ToastManager},
};

pub fn use_line_map(canvas_id: String, data: Memo<Option<Vec<Vec<(f32, f32)>>>>, options: ChartOptions) {
	let mut toast = use_context::<Signal<ToastManager>>();
	use_effect(move || {
		if let Some(data) = data() {
			if let Err(e) = render_line_chart(canvas_id.as_str(), data.clone(), options.clone()) {
				info!("{e}");
				toast.write().popup(ToastInfo::builder().context(e.to_string()).build());
			}
		}
	});
}
