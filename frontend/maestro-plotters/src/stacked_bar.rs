use dioxus::prelude::*;
use dioxus_logger::tracing::info;
use maestro_toast::{ctx::use_toast, toast_info::ToastInfo};

use crate::chart_options::ChartOptions;

pub fn use_stacked_bar_hook(canvas_id: String, data: Memo<Option<Vec<(String, Vec<f32>)>>>, options: ChartOptions) {
	let mut toast = use_toast();
	use_effect(move || {
		if let Some(data) = data() {
			if let Err(e) = render_stacked_bar_chart(canvas_id.as_str(), data.clone(), options.clone()) {
				info!("{e}");
				toast.write().popup(ToastInfo::builder().context(e.to_string()).build());
			}
		}
	});
}

use dioxus::hooks::to_owned;
use plotters::prelude::*;
use plotters_canvas::CanvasBackend;

pub fn render_stacked_bar_chart(canvas_id: &str, data: Vec<(String, Vec<f32>)>, options: ChartOptions) -> Result<(), Box<dyn std::error::Error>> {
	let root = CanvasBackend::new(canvas_id).expect("failed to create CanvasBackend").into_drawing_area();
	root.fill(&WHITE)?;
	let categories: Vec<String> = data.iter().map(|(category, _)| category.clone()).collect();
	let n_categories = categories.len();
	let n_series = data.first().ok_or("no data")?.1.len();
	let max_value = data.iter().map(|(_, values)| values.iter().sum::<f32>()).fold(0.0, f32::max);
	let mut chart = ChartBuilder::on(&root)
		.caption(&options.title, (options.font.0.as_str(), options.font.1).into_font())
		.margin(options.margin)
		.x_label_area_size(options.x_label_area_size)
		.y_label_area_size(options.y_label_area_size)
		.build_cartesian_2d(0f32..n_categories as f32, 0.0f32..max_value)?;
	chart.configure_mesh().x_desc(&options.x_label).y_desc(&options.y_label).draw()?;
	for series in 0..n_series {
		to_owned![options];
		let series_data: Vec<(f32, f32)> = data.iter().enumerate().map(|(i, (_, values))| (i as f32, values[series])).collect();
		chart
			.draw_series(AreaSeries::new(series_data.iter().map(|&(x, y)| (x, y)), 0.0, options.clone().colors[series % options.clone().colors.len()].mix(0.7)))?
			.label(&options.clone().series_labels[series])
			.legend(move |(x, y)| Rectangle::new([(x, y - 5), (x + 10, y + 5)], options.clone().colors[series % options.colors.len()].filled()));
	}
	chart.configure_series_labels().background_style(WHITE.mix(0.8)).border_style(BLACK).draw()?;
	chart.configure_mesh().x_labels(n_categories).x_label_formatter(&|x| categories[*x as usize].clone());
	root.present()?;
	Ok(())
}
