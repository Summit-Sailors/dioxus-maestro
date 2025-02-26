use {
  crate::components::ui::features::Features, dioxus::prelude::*, maestro_hooks::explicit_memo::use_explicit_memo, maestro_plotters::{
    bar_chart::use_bar_chart, 
    candle_stick::use_candle_stick_hook, 
    chart_options::ChartOptions, 
    heat_map::use_heat_map, 
    line_map::use_line_map, 
    pie::{use_pie_chart, 
      PieChartOptions
    }, 
    radar::use_radar, 
    scatter_plot::use_scatter_plot_hook
  }, plotters::style::RGBColor
};

#[component]
pub fn PlottersDemo() -> Element {
  let bar_data = Memo::new(move || {
    use_explicit_memo((), || {
      Some(vec![
        ("Jan".to_string(), 100.0f32),
        ("Feb".to_string(), 150.0f32),
        ("Mar".to_string(), 80.0f32),
        ("Apr".to_string(), 200.0f32),
        ("May".to_string(), 120.0f32),
        ("Jun".to_string(), 160.0f32),
        ("Jul".to_string(), 190.0f32),
        ("Aug".to_string(), 140.0f32),
        ("Sep".to_string(), 170.0f32),
        ("Oct".to_string(), 130.0f32),
        ("Nov".to_string(), 180.0f32),
        ("Dec".to_string(), 210.0f32),
      ])
    })
  });

  let line_data = Memo::new(move || {
    use_explicit_memo((), || {
      Some(vec![
        // current year
        vec![
          (1.0f32, 10.0f32), (2.0f32, 25.0f32), (3.0f32, 15.0f32),
          (4.0f32, 30.0f32), (5.0f32, 20.0f32), (6.0f32, 35.0f32),
          (7.0f32, 28.0f32), (8.0f32, 40.0f32), (9.0f32, 32.0f32),
          (10.0f32, 45.0f32), (11.0f32, 38.0f32), (12.0f32, 50.0f32),
        ],
        // previous year
        vec![
          (1.0f32, 15.0f32), (2.0f32, 20.0f32), (3.0f32, 25.0f32),
          (4.0f32, 22.0f32), (5.0f32, 28.0f32), (6.0f32, 30.0f32),
          (7.0f32, 24.0f32), (8.0f32, 35.0f32), (9.0f32, 28.0f32),
          (10.0f32, 40.0f32), (11.0f32, 32.0f32), (12.0f32, 45.0f32),
        ],
        // target
        vec![
          (1.0f32, 20.0f32), (2.0f32, 22.0f32), (3.0f32, 24.0f32),
          (4.0f32, 26.0f32), (5.0f32, 28.0f32), (6.0f32, 30.0f32),
          (7.0f32, 32.0f32), (8.0f32, 34.0f32), (9.0f32, 36.0f32),
          (10.0f32, 38.0f32), (11.0f32, 40.0f32), (12.0f32, 42.0f32),
        ],
      ])
    })
  });

  let pie_sizes = Memo::new(move || {
    use_explicit_memo((), || Some(vec![35, 25, 20, 15, 5]))
  });

  let pie_labels = Memo::new(move || {
    use_explicit_memo((), || {
      Some(vec![
        "Product A".to_string(),
        "Product B".to_string(),
        "Product C".to_string(),
        "Product D".to_string(),
        "Others".to_string(),
      ])
    })
  });

  let radar_data = Memo::new(move || {
    use_explicit_memo((), || {
      Some(vec![
        ("Speed".to_string(), 85.0f32),
        ("Power".to_string(), 92.0f32),
        ("Accuracy".to_string(), 88.0f32),
        ("Agility".to_string(), 95.0f32),
        ("Defense".to_string(), 78.0f32),
        ("Recovery".to_string(), 83.0f32),
        ("Range".to_string(), 90.0f32),
        ("Efficiency".to_string(), 87.0f32),
      ])
    })
  });

  let chart_options = ChartOptions::builder()
    .title("Performance Analytics Dashboard".into())
    .x_label("Time Period".into())
    .y_label("Value".into())
    .margin(60)
    .x_label_area_size(50)
    .y_label_area_size(50)
    .series_labels(vec![
      "Current Year".into(),
      "Previous Year".into(),
      "Target".into(),
    ])
    .colors(vec![
      RGBColor(65, 105, 225),
      RGBColor(46, 139, 87),
      RGBColor(255, 99, 71),
      RGBColor(255, 215, 0),
      RGBColor(148, 0, 211),
    ])
    .font(("Arial".into(), 24))
    .build();

  let pie_options = PieChartOptions::builder()
    .center((250, 250))
    .radius(200.0)
    .start_angle(0.0)
    .label_style(("Arial".to_string(), 18))
    .label_offset(35.0)
    .percentage_style(("Arial".to_string(), 16, RGBColor(255, 255, 255)))
    .donut_hole(100.0)
    .build();

  let pie_colors = vec![
    RGBColor(65, 105, 225),
    RGBColor(46, 139, 87),
    RGBColor(255, 99, 71),
    RGBColor(255, 215, 0),
    RGBColor(148, 0, 211),
  ];

  let scatter_data = Memo::new(move || {
    use_explicit_memo((), || {
      Some(vec![
        (1.2, 2.4), (2.3, 3.5), (3.1, 4.2), (4.2, 3.8),
        (5.4, 5.2), (6.1, 5.8), (7.2, 6.4), (8.3, 7.1),
        (9.1, 8.2), (10.2, 9.4), (11.3, 10.1), (12.4, 11.2)
      ])
    })
  });

  let scatter_highlights = Memo::new(move || {
    use_explicit_memo((), || {
      Some(vec![(4.2, 3.8), (8.3, 7.1), (12.4, 11.2)])
    })
  });

  // (date, open, high, low, close)
  let candlestick_data = Memo::new(move || {
    use_explicit_memo((), || {
      Some(vec![
        (1.0, 100.0, 110.0, 95.0, 105.0),
        (2.0, 105.0, 115.0, 100.0, 110.0),
        (3.0, 110.0, 120.0, 105.0, 115.0),
        (4.0, 115.0, 125.0, 110.0, 108.0),
        (5.0, 108.0, 118.0, 103.0, 113.0),
        (6.0, 113.0, 123.0, 108.0, 120.0),
        (7.0, 120.0, 130.0, 115.0, 125.0),
        (8.0, 125.0, 135.0, 120.0, 118.0),
      ])
    })
  });

  let heatmap_data = Memo::new(move || {
    use_explicit_memo((), || {
      Some(vec![
        vec![0.8, 0.2, 0.6, 0.4],
        vec![0.3, 0.7, 0.5, 0.9],
        vec![0.5, 0.4, 0.8, 0.2],
        vec![0.4, 0.6, 0.3, 0.7],
      ])
    })
  });
  
  let heatmap_labels = Memo::new(move || {
    use_explicit_memo((), || {
      Some(vec!["Q1", "Q2", "Q3", "Q4"])
    })
  });

  rsx! {
    div { 
      class: "container mx-auto p-6 bg-gray-900 rounded-lg",
      div { class: "mb-8",
        h1 { class: "text-gray-100 text-center text-3xl font-bold mb-2", "Maestro Plotters" }
        p { class: "text-gray-300 text-center",
          "A powerful, flexible, and reactive charting library for Dioxus applications built on top of the plotters crate."
        }
      }

      div {
        class: "flex space-x-2",  
        Features {
          title: "Form".to_string(),
          features: vec![
            "Superior Type Safety: Leverages Rust's type system for chart configurations and data validation".to_string(),
            "Enhanced Error Handling: Built-in integration with maestro-toast for graceful error handling".to_string(),
            "Reactive Architecture: Uses Dioxus hooks for automatic updates and efficient rendering".to_string(),
            "Unified Configuration: Consistent API across all chart types through the ChartOptions system".to_string(),
            "Canvas Integration: Direct canvas backend integration for optimal performance".to_string(),
            "Memory Management: Use Memo::new for data that doesn't need frequent updates".to_string(),
            "Performance: Share ChartOptions instances when possible for multiple charts".to_string(),
          ]
        }
      }

      h1 { 
        class: "text-4xl font-bold mb-8 text-gray-400 text-center",
        "Analytics Dashboard"
      }

      div { 
          class: "grid grid-cols-1 md:grid-cols-2 gap-8",
          // bar chart section
          div { 
            class: "bg-gray-800 text-center rounded-lg shadow-lg p-6 border border-gray-700 hover:shadow-xl transition-shadow",
            h3 { 
              class: "text-2xl font-bold mb-4 text-gray-200",
              "Monthly Revenue Performance"
            }
            p {
              class: "text-gray-400 mb-4",
              "Year-to-date revenue breakdown by month"
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
            class: "bg-gray-800 text-center rounded-lg shadow-lg p-6 border border-gray-700 hover:shadow-xl transition-shadow",
            h3 { 
              class: "text-2xl font-bold mb-4 text-gray-200",
              "Performance Trend Analysis"
            }
            p {
              class: "text-gray-400 mb-4",
              "Comparative analysis with previous year and targets"
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
            class: "bg-gray-800 text-center rounded-lg shadow-lg p-6 border border-gray-700 hover:shadow-xl transition-shadow",
            h3 { 
              class: "text-2xl font-bold mb-4 text-gray-200",
              "Product Revenue Distribution"
            }
            p {
              class: "text-gray-400 mb-4",
              "Revenue breakdown by product category"
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
            class: "bg-gray-800 text-center rounded-lg shadow-lg p-6 border border-gray-700 hover:shadow-xl transition-shadow",
            h3 { 
              class: "text-2xl font-bold mb-4 text-gray-200",
              "Performance Metrics Overview"
            }
            p {
              class: "text-gray-400 mb-4",
              "Multi-dimensional performance analysis"
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

          // scatter plot section
          div { 
            class: "bg-gray-800 text-center rounded-lg shadow-lg p-6 border border-gray-700 hover:shadow-xl transition-shadow",
            h3 { 
              class: "text-2xl font-bold mb-4 text-gray-200",
              "Growth Correlation Analysis"
            }
            p {
              class: "text-gray-400 mb-4",
              "Revenue vs Customer Acquisition with key highlights"
            }
            div {
              class: "aspect-square",
              canvas { 
                id: "scatter-plot",
                width: "500",
                height: "500",
                class: "w-full h-full"
              }
              {use_scatter_plot_hook(
                "scatter-plot".into(),
                scatter_data,
                scatter_highlights,
                ChartOptions::builder()
                  .title("Growth Metrics Correlation".into())
                  .x_label("Revenue Growth (%)".into())
                  .y_label("Customer Growth (%)".into())
                  .build()
              )}
            }
        }

        // candlestick chart section
        div { 
          class: "bg-gray-800 text-center rounded-lg shadow-lg p-6 border border-gray-700 hover:shadow-xl transition-shadow",
          h3 { 
            class: "text-2xl font-bold mb-4 text-gray-200",
            "Stock Price Movement"
          }
          p {
            class: "text-gray-400 mb-4",
            "Daily price movements with trading ranges"
          }
          div {
            class: "aspect-square",
            canvas { 
              id: "candlestick-chart",
              width: "500",
              height: "500",
              class: "w-full h-full"
            }
            {use_candle_stick_hook(
              "candlestick-chart".into(),
              candlestick_data,
              ChartOptions::builder()
                .title("Stock Price Analysis".into())
                .x_label("Trading Day".into())
                .y_label("Price ($)".into())
                .build()
            )}
          }
        }

        // Heatmap section
        div { 
          class: "bg-gray-800 text-center rounded-lg shadow-lg p-6 border border-gray-700 hover:shadow-xl transition-shadow",
          h3 { 
            class: "text-2xl font-bold mb-4 text-gray-200",
            "Performance Heat Map"
          }
          p {
            class: "text-gray-400 mb-4",
            "Quarterly performance intensity analysis"
          }
          div {
            class: "aspect-square",
            canvas { 
              id: "heat-map",
              width: "500",
              height: "500",
              class: "w-full h-full"
            }
            {use_heat_map(
              "heat-map".into(),
              heatmap_data,
              heatmap_labels,
              ChartOptions::builder()
                .title("Quarterly Performance Matrix".into())
                .x_label("Period".into())
                .y_label("Metrics".into())
                .build()
            )}
          }
        }
      }
    }
  }
}
