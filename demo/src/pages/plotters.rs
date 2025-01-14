use dioxus::prelude::*;
use maestro_hooks::explicit_memo::use_explicit_memo;
use maestro_plotters::{
    bar_chart::use_bar_chart,
    chart_options::ChartOptions,
    line_map::use_line_map,
    pie::{use_pie_chart, PieChartOptions},
    radar::use_radar,
};
use plotters::style::RGBColor;

#[component]
pub fn Plotters(cx: Scope) -> Element {
  let bar_data = use_explicit_memo((), || {
    Some(vec![
      ("Jan".into(), 100.0),
      ("Feb".into(), 150.0),
      ("Mar".into(), 80.0),
      ("Apr".into(), 200.0),
    ])
  });

  let line_data = use_explicit_memo((), || {
    Some(vec![vec![
      (1.0, 10.0),
      (2.0, 25.0),
      (3.0, 15.0),
      (4.0, 30.0),
    ]])
  });

  let pie_sizes = use_explicit_memo((), || Some(vec![30, 20, 15, 35]));
  let pie_labels = use_explicit_memo((), || {
    Some(vec![
      "Category A".into(),
      "Category B".into(),
      "Category C".into(),
      "Category D".into(),
    ])
  });

  let radar_data = use_explicit_memo((), || {
    Some(vec![
      ("Speed".into(), 80.0),
      ("Power".into(), 60.0),
      ("Accuracy".into(), 90.0),
      ("Agility".into(), 70.0),
    ])
  });

  let chart_options = ChartOptions::builder()
    .title("Sample Chart".into())
    .x_label("X Axis".into())
    .y_label("Y Axis".into())
    .series_labels(vec!["Series 1".into()])
    .build();

  let pie_options = PieChartOptions::builder()
    .center((250, 250))
    .radius(200.0)
    .build();

  let pie_colors = vec![
    RGBColor(255, 0, 0),
    RGBColor(0, 255, 0),
    RGBColor(0, 0, 255),
    RGBColor(255, 255, 0),
  ];

  rsx! {
    div { class: "grid grid-cols-2 gap-4 p-4",
      div { class: "border p-4 rounded",
        h3 { class: "text-xl mb-2", "Bar Chart" }
        canvas { id: "bar-chart", width: "500", height: "500" }
        {use_bar_chart("bar-chart".into(), bar_data, chart_options.clone())}
      }
      div { class: "border p-4 rounded",
        h3 { class: "text-xl mb-2", "Line Chart" }
        canvas { id: "line-chart", width: "500", height: "500" }
        {use_line_map("line-chart".into(), line_data, chart_options.clone())}
      }
      div { class: "border p-4 rounded",
        h3 { class: "text-xl mb-2", "Pie Chart" }
        canvas { id: "pie-chart", width: "500", height: "500" }
        {use_pie_chart("pie-chart".into(), pie_sizes, pie_colors.clone(), pie_labels, pie_options.clone())}
      }
      div { class: "border p-4 rounded",
        h3 { class: "text-xl mb-2", "Radar Chart" }
        canvas { id: "radar-chart", width: "500", height: "500" }
        {use_radar("radar-chart".into(), radar_data, chart_options.clone())}
      }
    }
  }
}
