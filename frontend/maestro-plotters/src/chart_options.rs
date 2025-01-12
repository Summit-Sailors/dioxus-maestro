use plotters::prelude::*;

#[derive(Debug, Clone, PartialEq, bon::Builder)]
pub struct ChartOptions {
	#[builder(default = "Chart".to_string())]
	pub title: String,
	#[builder(default = "X Axis".to_string())]
	pub x_label: String,
	#[builder(default = "Y Axis".to_string())]
	pub y_label: String,
	#[builder(default = vec!["Series 1".to_string()])]
	pub series_labels: Vec<String>,
	#[builder(default = vec![RED, BLUE, GREEN, YELLOW, CYAN, MAGENTA])]
	pub colors: Vec<RGBColor>,
	#[builder(default = 10)]
	pub margin: u32,
	#[builder(default = 30)]
	pub x_label_area_size: u32,
	#[builder(default = 30)]
	pub y_label_area_size: u32,
	#[builder(default = ("sans-serif".to_string(), 20))]
	pub font: (String, u32),
}
