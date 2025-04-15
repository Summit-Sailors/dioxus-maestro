use std::cmp::Ordering;

pub fn clamp(value: f32, min_value: f32, max_value: f32) -> f32 {
	max_value.min(min_value.max(value))
}

pub fn linear_scale(input: [f32; 2], output: [f32; 2]) -> impl Fn(f32) -> f32 {
	move |value: f32| {
		if input[0] == input[1] || output[0] == output[1] {
			output[0]
		} else {
			let ratio = (output[1] - output[0]) / (input[1] - input[0]);
			output[0] + ratio * (value - input[0])
		}
	}
}

pub fn get_decimal_count(value: f32) -> usize {
	value.to_string().split('.').nth(1).map_or(0, |s| s.len())
}

pub fn round_value(value: f32, decimal_count: usize) -> f32 {
	let rounder = 10.0_f32.powi(decimal_count as i32);
	(value * rounder).round() / rounder
}

pub fn get_next_sorted_values(prev_values: &[f32], next_value: f32, at_index: usize) -> Vec<f32> {
	let mut next_values = prev_values.to_vec();
	next_values[at_index] = next_value;
	next_values.sort_by(|a, b| a.partial_cmp(b).unwrap_or(Ordering::Equal));
	next_values
}

pub fn convert_value_to_percentage(value: f32, min: f32, max: f32) -> f32 {
	let max_steps = max - min;
	let percent_per_step = 100.0 / max_steps;
	let percentage = percent_per_step * (value - min);
	clamp(percentage, 0.0, 100.0)
}

pub fn get_label(index: usize, total_values: usize) -> String {
	if total_values > 2 {
		format!("Value {} of {}", index + 1, total_values)
	} else if total_values == 2 {
		["Minimum", "Maximum"].get(index).map_or("", |v| v).to_string()
	} else {
		"".to_string()
	}
}

pub fn get_closest_value_index(values: &[f32], next_value: f32) -> usize {
	if values.len() == 1 {
		0
	} else {
		let distances: Vec<f32> = values.iter().map(|&value| (value - next_value).abs()).collect();
		distances.iter().enumerate().min_by(|(_, a), (_, b)| a.partial_cmp(b).unwrap_or(Ordering::Equal)).map(|(index, _)| index).unwrap_or(0)
	}
}

pub fn get_thumb_in_bounds_offset(width: f32, left: f32) -> f32 {
	let half_width = width / 2.0;
	let half_percent = 50.0;
	let offset = linear_scale([0.0, half_percent], [0.0, half_width]);
	half_width - offset(left)
}

pub fn get_steps_between_values(values: &[f32]) -> Vec<f32> {
	values.windows(2).map(|w| w[1] - w[0]).collect()
}

pub fn has_min_steps_between_values(values: &[f32], min_steps_between_values: f32) -> bool {
	if min_steps_between_values > 0.0 {
		let steps_between_values = get_steps_between_values(values);
		let actual_min_steps_between_values = steps_between_values.iter().cloned().reduce(f32::min).unwrap_or(0.0);
		actual_min_steps_between_values >= min_steps_between_values
	} else {
		true
	}
}
