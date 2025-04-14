use maestro_toast::ctx::use_toast;
use plotters::prelude::*;
use plotters_canvas::CanvasBackend;

use crate::chart_options::ChartOptions;

pub fn render_radar_chart(canvas_id: &str, data: Vec<(String, f32)>, options: ChartOptions) -> Result<(), Box<dyn std::error::Error>> {
	let root = CanvasBackend::new(canvas_id).expect("failed to create CanvasBackend").into_drawing_area();
	root.fill(&WHITE)?;
	let max_value = data.iter().map(|(_, value)| *value).fold(f32::NEG_INFINITY, f32::max);
	let n = data.len();
	let angle_step = 2.0 * std::f32::consts::PI / n as f32;
	let mut chart = ChartBuilder::on(&root)
		.caption(&options.title, (options.font.0.as_str(), options.font.1).into_font())
		.margin(options.margin)
		.build_cartesian_2d(-1.0f32..1.0f32, -1.0f32..1.0f32)?;
	chart.configure_mesh().disable_mesh().draw()?;
	// Draw axes and labels
	for (i, (label, _)) in data.iter().enumerate() {
		let angle = i as f32 * angle_step;
		let (sin, cos) = angle.sin_cos();
		chart.draw_series(std::iter::once(PathElement::new(vec![(0.0, 0.0), (cos, sin)], BLACK.mix(0.3))))?;
		chart.draw_series(std::iter::once(Text::new(label.clone(), (cos * 1.1, sin * 1.1), ("sans-serif", 15).into_font())))?;
	}
	// Draw data
	let points: Vec<(f32, f32)> = data
		.iter()
		.enumerate()
		.map(|(i, (_, value))| {
			let angle = i as f32 * angle_step;
			let (sin, cos) = angle.sin_cos();
			(cos * value / max_value, sin * value / max_value)
		})
		.collect();
	chart.draw_series(std::iter::once(Polygon::new(points.clone(), options.colors[0].mix(0.3))))?;
	chart.draw_series(PointSeries::of_element(points, 5, &options.colors[0], &|c, s, st| Circle::new(c, s, st.filled())))?;
	root.present()?;
	Ok(())
}

use dioxus::prelude::*;
use dioxus_logger::tracing::info;
use maestro_toast::toast_info::ToastInfo;

pub fn use_radar(canvas_id: String, data: Memo<Option<Vec<(String, f32)>>>, options: ChartOptions) {
	let mut toast = use_toast();
	use_effect(move || {
		if let Some(data) = data() {
			if let Err(e) = render_radar_chart(canvas_id.as_str(), data.clone(), options.clone()) {
				info!("{e}");
				toast.write().popup(ToastInfo::builder().context(e.to_string()).build());
			}
		}
	});
}
