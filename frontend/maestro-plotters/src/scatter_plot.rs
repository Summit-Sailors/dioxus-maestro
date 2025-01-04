use {crate::chart_options::ChartOptions, maestro_toast::ctx::use_toast, plotters::prelude::*, plotters_canvas::CanvasBackend};

pub fn render_scatter_plot(
	canvas_id: &str,
	data: Vec<(f32, f32)>,
	highlight: Vec<(f32, f32)>,
	options: ChartOptions,
) -> Result<(), Box<dyn std::error::Error>> {
	let root = CanvasBackend::new(canvas_id).expect("failed to create CanvasBackend").into_drawing_area();
	root.fill(&WHITE)?;
	let x_range = data.iter().chain(highlight.iter()).map(|&(x, _)| x).fold((f32::INFINITY, f32::NEG_INFINITY), |acc, x| (acc.0.min(x), acc.1.max(x)));
	let y_range = data.iter().chain(highlight.iter()).map(|&(_, y)| y).fold((f32::INFINITY, f32::NEG_INFINITY), |acc, y| (acc.0.min(y), acc.1.max(y)));
	let mut chart = ChartBuilder::on(&root)
		.caption(&options.title, (options.font.0.as_str(), options.font.1).into_font())
		.margin(options.margin)
		.x_label_area_size(options.x_label_area_size)
		.y_label_area_size(options.y_label_area_size)
		.build_cartesian_2d(x_range.0..x_range.1, y_range.0..y_range.1)?;
	chart.configure_mesh().x_desc(&options.x_label).y_desc(&options.y_label).draw()?;
	chart
		.draw_series(data.iter().map(|&(x, y)| Circle::new((x, y), 3, options.colors[0].filled())))?
		.label("Data")
		.legend(|(x, y)| PathElement::new(vec![(x, y), (x + 20, y)], options.colors[0]));
	chart
		.draw_series(highlight.iter().map(|&(x, y)| TriangleMarker::new((x, y), 5, options.colors[1].filled())))?
		.label("Highlight")
		.legend(|(x, y)| PathElement::new(vec![(x, y), (x + 20, y)], options.colors[1]));
	chart.configure_series_labels().background_style(WHITE.mix(0.8)).border_style(BLACK).draw()?;
	root.present()?;
	Ok(())
}

use {dioxus::prelude::*, dioxus_logger::tracing::info, maestro_toast::toast_info::ToastInfo};

pub fn use_scatter_plot_hook(canvas_id: String, data: Memo<Option<Vec<(f32, f32)>>>, highlight: Memo<Option<Vec<(f32, f32)>>>, options: ChartOptions) {
	let mut toast = use_toast();
	use_effect(move || {
		if let Some(data) = data() {
			if let Err(e) = render_scatter_plot(canvas_id.as_str(), data.clone(), highlight().unwrap_or_default(), options.clone()) {
				info!("{e}");
				toast.write().popup(ToastInfo::builder().context(e.to_string()).build());
			}
		}
	});
}
