use {crate::chart_options::ChartOptions, maestro_toast::ctx::use_toast, plotters::prelude::*, plotters_canvas::CanvasBackend};

pub fn render_candlestick_chart(canvas_id: &str, data: Vec<(f32, f32, f32, f32, f32)>, options: ChartOptions) -> Result<(), Box<dyn std::error::Error>> {
	let root = CanvasBackend::new(canvas_id).expect("failed to create CanvasBackend").into_drawing_area();
	root.fill(&WHITE)?;
	let (min_x, max_x) = data.iter().map(|(x, _, _, _, _)| *x).fold((f32::INFINITY, f32::NEG_INFINITY), |(min, max), x| (min.min(x), max.max(x)));
	let (min_y, max_y) = data.iter().fold((f32::INFINITY, f32::NEG_INFINITY), |(min, max), &(_, open, high, low, close)| {
		(min.min(open).min(high).min(low).min(close), max.max(open).max(high).max(low).max(close))
	});
	let mut chart = ChartBuilder::on(&root)
		.caption(&options.title, (options.font.0.as_str(), options.font.1).into_font())
		.margin(options.margin)
		.x_label_area_size(options.x_label_area_size)
		.y_label_area_size(options.y_label_area_size)
		.build_cartesian_2d(min_x..max_x, min_y..max_y)?;
	chart.configure_mesh().x_desc(&options.x_label).y_desc(&options.y_label).draw()?;
	chart.draw_series(data.iter().map(|&(x, open, high, low, close)| CandleStick::new(x, open, high, low, close, GREEN, RED, 15)))?;
	root.present()?;
	Ok(())
}

use {dioxus::prelude::*, dioxus_logger::tracing::info, maestro_toast::toast_info::ToastInfo};

pub fn use_candle_stick_hook(canvas_id: String, data: Memo<Option<Vec<(f32, f32, f32, f32, f32)>>>, options: ChartOptions) {
	let mut toast = use_toast();
	use_effect(move || {
		if let Some(data) = data() {
			if let Err(e) = render_candlestick_chart(canvas_id.as_str(), data.clone(), options.clone()) {
				info!("{e}");
				toast.write().popup(ToastInfo::builder().context(e.to_string()).build());
			}
		}
	});
}
