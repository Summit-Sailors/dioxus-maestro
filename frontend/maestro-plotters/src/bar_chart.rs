use {
	crate::chart_options::ChartOptions,
	dioxus::prelude::*,
	dioxus_logger::tracing::info,
	maestro_toast::{ctx::use_toast, toast_info::ToastInfo},
	plotters::prelude::*,
	plotters_canvas::CanvasBackend,
};

pub fn use_bar_chart(canvas_id: String, data: Memo<Option<Vec<(String, f32)>>>, options: ChartOptions) {
	let mut toast = use_toast();
	use_effect(move || {
		if let Some(data) = data() {
			if let Err(e) = render_bar_chart(canvas_id.as_str(), data.clone(), options.clone()) {
				info!("{e}");
				toast.write().popup(ToastInfo::builder().context(e.to_string()).build());
			}
		}
	});
}

pub fn render_bar_chart(canvas_id: &str, data: Vec<(String, f32)>, options: ChartOptions) -> Result<(), Box<dyn std::error::Error>> {
	let root = CanvasBackend::new(canvas_id).expect("failed to create CanvasBackend").into_drawing_area();
	root.fill(&WHITE)?;
	let max_value = data.iter().map(|(_, value)| *value).fold(f32::NEG_INFINITY, f32::max);
	let mut chart = ChartBuilder::on(&root)
		.caption(&options.title, (options.font.0.as_str(), options.font.1).into_font())
		.margin(options.margin)
		.x_label_area_size(options.x_label_area_size)
		.y_label_area_size(options.y_label_area_size)
		.build_cartesian_2d(0..data.len(), 0.0f32..max_value)?;
	chart.configure_mesh().x_desc(&options.x_label).y_desc(&options.y_label).draw()?;
	chart.draw_series(
		Histogram::vertical(&chart).style(options.colors[0].filled()).margin(3).data(data.iter().enumerate().map(|(i, (_label, value))| (i, *value))),
	)?;
	chart.configure_series_labels().background_style(WHITE.mix(0.8)).border_style(BLACK).draw()?;
	chart.configure_mesh().x_labels(data.len()).x_label_formatter(&|x| data[*x].0.clone());
	root.present()?;
	Ok(())
}
