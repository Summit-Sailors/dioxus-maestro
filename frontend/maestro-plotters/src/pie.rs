use std::{error::Error, f32::consts::PI};

use maestro_toast::ctx::use_toast;
use plotters::{prelude::*, style::RGBColor};
use plotters_canvas::CanvasBackend;

#[derive(Debug, Clone, PartialEq, bon::Builder)]
pub struct PieChartOptions {
	#[builder(default = (0,0))]
	pub center: (i32, i32),
	#[builder(default = 2.0)]
	pub radius: f32,
	#[builder(default = 0.0)]
	pub start_angle: f32,
	#[builder(default = ("sans-serif".to_string(), 1))]
	pub label_style: (String, i32),
	#[builder(default = 1.0)]
	pub label_offset: f32,
	pub percentage_style: Option<(String, i32, RGBColor)>,
	#[builder(default = 1.0)]
	pub donut_hole: f32,
}

#[derive(Debug)]
pub enum PieError {
	LengthMismatch,
}

impl std::fmt::Display for PieError {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		match self {
			PieError::LengthMismatch => write!(f, "Length Mismatch"),
		}
	}
}

impl std::error::Error for PieError {}

pub struct PieChart {
	options: PieChartOptions,
	sizes: Vec<i32>,
	colors: Vec<RGBColor>,
	labels: Vec<String>,
	total: i32,
}

impl PieChart {
	pub fn new(sizes: Vec<i32>, colors: Vec<RGBColor>, labels: Vec<String>, options: PieChartOptions) -> Self {
		let total = sizes.iter().sum();
		Self { options, sizes, colors, labels, total }
	}

	pub fn render(&self, canvas_id: &str) -> Result<(), Box<dyn Error>> {
		let backend = CanvasBackend::new(canvas_id).unwrap();
		let root = backend.into_drawing_area();
		root.fill(&BLACK)?;
		let mut chart = ChartBuilder::on(&root).build_cartesian_2d(-500i32..500i32, -500i32..500i32)?;
		let mut offset_theta = self.options.start_angle.to_radians();
		let radian_increment = PI / 180.0 / self.options.radius.sqrt() * 2.0;

		let mut perc_labels = Vec::new();
		for (index, slice) in self.sizes.iter().enumerate() {
			let slice_style = self.colors.get(index).ok_or(PieError::LengthMismatch)?;
			let label = self.labels.get(index).ok_or(PieError::LengthMismatch)?;
			let mut points = if self.options.donut_hole == 0.0 { vec![self.options.center] } else { vec![] };
			let ratio = *slice as f32 / self.total as f32;
			let theta_final = ratio * 2.0 * PI + offset_theta;
			let middle_theta = ratio * PI + offset_theta;
			let slice_start = offset_theta;
			while offset_theta <= theta_final {
				let coord = self.theta_to_ordinal_coord(self.options.radius, offset_theta);
				points.push(coord);
				offset_theta += radian_increment;
			}
			let final_coord = self.theta_to_ordinal_coord(self.options.radius, theta_final);
			points.push(final_coord);
			if self.options.donut_hole > 0.0 {
				while offset_theta >= slice_start {
					let coord = self.theta_to_ordinal_coord(self.options.donut_hole, offset_theta);
					points.push(coord);
					offset_theta -= radian_increment;
				}
				let final_coord_inner = self.theta_to_ordinal_coord(self.options.donut_hole, slice_start);
				points.push(final_coord_inner);
			}
			offset_theta = theta_final;
			chart.draw_series(std::iter::once(Polygon::new(points, slice_style.filled())))?;
			let mut mid_coord = self.theta_to_ordinal_coord(self.options.radius + self.options.label_offset, middle_theta);
			let label_size = root.estimate_text_size(label, &(self.options.label_style.0.as_str(), self.options.label_style.1).into())?;
			if mid_coord.0 <= self.options.center.0 {
				mid_coord.0 -= label_size.0 as i32;
			}
			chart.draw_series(std::iter::once(Text::new(label.to_string(), mid_coord, (self.options.label_style.0.as_str(), self.options.label_style.1))))?;

			if let Some(percentage_style) = &self.options.percentage_style {
				let percentage_style = (percentage_style.0.as_str(), percentage_style.1, &percentage_style.2).into_text_style(&root);
				let perc_label = format!("{:.1}%", (ratio * 100.0));
				let label_size = root.estimate_text_size(&perc_label, &percentage_style)?;
				let text_x_mid = (label_size.0 as f32 / 2.0).round() as i32;
				let text_y_mid = (label_size.1 as f32 / 2.0).round() as i32;
				let perc_radius = (self.options.radius + self.options.donut_hole) / 2.0;
				let perc_coord = self.theta_to_ordinal_coord(perc_radius, middle_theta);
				let perc_coord = (perc_coord.0 - text_x_mid, perc_coord.1 - text_y_mid);
				perc_labels.push((perc_label, perc_coord));
			}
		}
		if let Some(percentage_style) = &self.options.percentage_style {
			let percentage_style = (percentage_style.0.as_str(), percentage_style.1, &percentage_style.2).into_text_style(&root);
			for (label, coord) in perc_labels {
				chart.draw_series(std::iter::once(Text::new(label, coord, &percentage_style)))?;
			}
		}
		root.present()?;
		Ok(())
	}

	fn theta_to_ordinal_coord(&self, radius: f32, theta: f32) -> (i32, i32) {
		let (sin, cos) = theta.sin_cos();
		((radius * cos + self.options.center.0 as f32).round() as i32, (radius * sin + self.options.center.1 as f32).round() as i32)
	}
}

use dioxus::prelude::*;
use dioxus_logger::tracing::info;
use maestro_toast::toast_info::ToastInfo;

pub fn use_pie_chart(canvas_id: String, sizes: Memo<Option<Vec<i32>>>, colors: Vec<RGBColor>, labels: Memo<Option<Vec<String>>>, options: PieChartOptions) {
	let mut toast = use_toast();
	use_effect(move || {
		if let (Some(sizes), Some(labels)) = (sizes(), labels()) {
			if let Err(e) = PieChart::new(sizes.clone(), colors.clone(), labels.clone(), options.clone()).render(canvas_id.as_str()) {
				info!("{e}");
				toast.write().popup(ToastInfo::builder().context(e.to_string()).build());
			}
		}
	});
}
