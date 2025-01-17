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
pub fn Plotters() -> Element {
  let bar_data = Memo::new(move || {
    use_explicit_memo((), || {
      Some(vec![
        ("Jan".to_string(), 100.0f32),
        ("Feb".to_string(), 150.0f32),
        ("Mar".to_string(), 80.0f32),
        ("Apr".to_string(), 200.0f32),
        ("May".to_string(), 120.0f32),
        ("Jun".to_string(), 160.0f32),
      ])
    })
  });

  let line_data = Memo::new(move || {
    use_explicit_memo((), || {
      Some(vec![
        vec![
          (1.0f32, 10.0f32),
          (2.0f32, 25.0f32),
          (3.0f32, 15.0f32),
          (4.0f32, 30.0f32),
          (5.0f32, 20.0f32),
        ],
        vec![
          (1.0f32, 15.0f32),
          (2.0f32, 20.0f32),
          (3.0f32, 25.0f32),
          (4.0f32, 22.0f32),
          (5.0f32, 28.0f32),
        ],
      ])
    })
  });

  let pie_sizes = Memo::new(move || {
    use_explicit_memo((), || Some(vec![30, 20, 15, 35]))
  });

  let pie_labels = Memo::new(move || {
    use_explicit_memo((), || {
      Some(vec![
        "Category A".to_string(),
        "Category B".to_string(),
        "Category C".to_string(),
        "Category D".to_string(),
      ])
    })
  });

  let radar_data = Memo::new(move || {
    use_explicit_memo((), || {
      Some(vec![
        ("Speed".to_string(), 80.0f32),
        ("Power".to_string(), 60.0f32),
        ("Accuracy".to_string(), 90.0f32),
        ("Agility".to_string(), 70.0f32),
        ("Defense".to_string(), 85.0f32),
        ("Recovery".to_string(), 75.0f32),
      ])
    })
  });

  let chart_options = ChartOptions::builder()
    .title("Performance Metrics".into())
    .x_label("Time Period".into())
    .y_label("Value".into())
    .margin(50)
    .x_label_area_size(40)
    .y_label_area_size(40)
    .series_labels(vec!["Current Period".into(), "Previous Period".into()])
    .colors(vec![
      RGBColor(65, 105, 225),
      RGBColor(46, 139, 87),
      RGBColor(255, 99, 71),
      RGBColor(255, 215, 0),
    ])
    .font(("Arial".into(), 20))
    .build();

  let pie_options = PieChartOptions::builder()
    .center((250, 250))
    .radius(200.0)
    .start_angle(0.0)
    .label_style(("Arial".to_string(), 16))
    .label_offset(30.0)
    .percentage_style(("Arial".to_string(), 14, RGBColor(255, 255, 255)))
    .donut_hole(100.0)
    .build();

  let pie_colors = vec![
    RGBColor(65, 105, 225),
    RGBColor(46, 139, 87),
    RGBColor(255, 99, 71),
    RGBColor(255, 215, 0),
  ];

  rsx! {
    div { 
      class: "container mx-auto p-8",
      div { 
        class: "grid grid-cols-1 md:grid-cols-2 gap-8",
        // bar chart section
        div { 
            class: "bg-white rounded-lg shadow-lg p-6",
            h3 { 
              class: "text-2xl font-bold mb-4 text-gray-800",
              "Monthly Performance"
            }
            div {
              class: "aspect-square",
              canvas { 
                id: "bar-chart",
                width: "500",
                height: "500",
                class: "w-full h-full"
              }
              {use_bar_chart("bar-chart".into(), bar_data, chart_options.clone())}
            }
        }

        // line chart section
        div { 
          class: "bg-white rounded-lg shadow-lg p-6",
          h3 { 
            class: "text-2xl font-bold mb-4 text-gray-800",
            "Trend Analysis"
          }
          div {
            class: "aspect-square",
            canvas { 
              id: "line-chart",
              width: "500",
              height: "500",
              class: "w-full h-full"
            }
            {use_line_map("line-chart".into(), line_data, chart_options.clone())}
          }
        }

        // pie chart section
        div { 
          class: "bg-white rounded-lg shadow-lg p-6",
          h3 { 
            class: "text-2xl font-bold mb-4 text-gray-800",
            "Distribution Analysis"
          }
          div {
            class: "aspect-square",
            canvas { 
              id: "pie-chart",
              width: "500",
              height: "500",
              class: "w-full h-full"
            }
            {use_pie_chart("pie-chart".into(), pie_sizes, pie_colors.clone(), pie_labels, pie_options.clone())}
          }
        }

        // radar chart section
        div { 
          class: "bg-white rounded-lg shadow-lg p-6",
          h3 { 
            class: "text-2xl font-bold mb-4 text-gray-800",
            "Capability Overview"
          }
          div {
            class: "aspect-square",
            canvas { 
              id: "radar-chart",
              width: "500",
              height: "500",
              class: "w-full h-full"
            }
            {use_radar("radar-chart".into(), radar_data, chart_options.clone())}
          }
        }
      }
    }
  }
}
