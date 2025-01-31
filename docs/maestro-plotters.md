# Maestro Plotters

A powerful, flexible, and reactive charting library for Dioxus applications built on top of the robust plotters crate.

## Features

- 🎯 **Dioxus-Native**: Built specifically for Dioxus with reactive hooks and seamless integration
- 📊 **Comprehensive Chart Types**:
  - Bar Charts
  - Line Charts
  - Scatter Plots
  - Heat Maps
  - Pie Charts
  - Radar Charts
  - Box Plots
  - Candlestick Charts
  - Stacked Bar Charts
- 🎨 **Consistent Styling**: Unified chart options and styling system
- 🔄 **Reactive Updates**: Charts automatically update when data changes
- 🚨 **Built-in Error Handling**: Integrated with maestro-toast for elegant error notifications
- 🎛️ **Flexible Configuration**: Extensive customization options for each chart type

## Why Maestro Plotters?

While Dioxus provides basic charting capabilities, Maestro Plotters offers several key advantages:

1. **Superior Type Safety**: Leverages Rust's type system for chart configurations and data validation
2. **Enhanced Error Handling**: Built-in integration with maestro-toast for graceful error handling
3. **Reactive Architecture**: Uses Dioxus hooks for automatic updates and efficient rendering
4. **Unified Configuration**: Consistent API across all chart types through the `ChartOptions` system
5. **Canvas Integration**: Direct canvas backend integration for optimal performance

## Installation

Add maestro-plotters to your Cargo.toml:

```toml
[dependencies]
dioxus-maestro = ""
```

## Usage

### Creating a Complete Analytics Dashboard

Here's an example of how to create a comprehensive analytics dashboard with multiple chart types:

```rust
use maestro_plotters::{
  bar_chart::use_bar_chart,
  line_map::use_line_map,
  pie::{use_pie_chart, PieChartOptions},
  radar::use_radar,
  chart_options::ChartOptions,
};

#[component]
fn AnalyticsDashboard() -> Element {
  // Configure shared chart options
  let chart_options = ChartOptions::builder()
    .title("Performance Analytics".into())
    .x_label("Time Period".into())
    .y_label("Value".into())
    .margin(60)
    .x_label_area_size(50)
    .y_label_area_size(50)
    .series_labels(vec!["Current".into(), "Previous".into(), "Target".into()])
    .colors(vec![
      RGBColor(65, 105, 225),
      RGBColor(46, 139, 87),
      RGBColor(255, 99, 71),
    ])
    .font(("Arial".into(), 24))
    .build();

  // Define chart data
  let bar_data = use_memo(cx, || Some(vec![
    ("Jan".to_string(), 100.0f32),
    ("Feb".to_string(), 150.0f32),
    // ... more data
  ]));

  rsx! {
    div { class: "grid grid-cols-2 gap-8",
      // Bar Chart
      div { class: "bg-white rounded-lg p-6",
        canvas { 
          id: "bar-chart",
          width: "500",
          height: "500"
        }
        {use_bar_chart("bar-chart".into(), bar_data, chart_options.clone())}
      }
      // Add more charts...
    }
  }
}
```

### Customizing Individual Charts

Each chart type supports specific customizations while maintaining consistency through the shared options system:

#### Pie Chart Customization

```rust
let pie_options = PieChartOptions::builder()
    .center((250, 250))
    .radius(200.0)
    .start_angle(0.0)
    .label_style(("Arial".to_string(), 18))
    .label_offset(35.0)
    .percentage_style(("Arial".to_string(), 16, RGBColor(255, 255, 255)))
    .donut_hole(100.0)  // Creates a donut chart
    .build();
```

#### Heat Map with Custom Labels

```rust
let heatmap_data = use_memo(cx, || Some(vec![
    vec![0.8, 0.2, 0.6, 0.4],
    vec![0.3, 0.7, 0.5, 0.9],
    // ... more data
]));

let heatmap_labels = use_memo(cx, || Some(vec!["Q1", "Q2", "Q3", "Q4"]));

use_heat_map(
    "heat-map".into(),
    heatmap_data,
    heatmap_labels,
    chart_options
);
```

#### Scatter Plot with Highlights

```rust
let scatter_data = use_memo(cx, || Some(vec![
    (1.2, 2.4), (2.3, 3.5), (3.1, 4.2),
    // ... more data points
]));

let highlights = use_memo(cx, || Some(vec![
    (4.2, 3.8), (8.3, 7.1)  // Key points to highlight
]));

use_scatter_plot_hook(
    "scatter-plot".into(),
    scatter_data,
    highlights,
    custom_options
);
```

### Responsive Design Integration

Maestro Plotters works seamlessly with responsive design patterns:

```rust
rsx! {
  div { 
    class: "aspect-square", // Maintains aspect ratio
    canvas { 
      id: "chart",
      width: "500",
      height: "500",
      class: "w-full h-full"  // Responsive sizing
    }
    {use_chart_hook(...)}
  }
}
```

### Data Reactivity

All charts automatically update when their data changes, thanks to Dioxus's reactive system and our use of the `use_memo` hook:

```rust
let dynamic_data = use_memo(cx, || {
  use_explicit_memo((), || Some(vec![
    // Your data here
  ]))
});
```

## Advanced Styling

### Color Schemes

```rust
let colors = vec![
  RGBColor(65, 105, 225),
  RGBColor(46, 139, 87),
  RGBColor(255, 99, 71),
  RGBColor(255, 215, 0),
  RGBColor(148, 0, 211),
];
```

## Best Practices

1. **Memory Management**: Use `use_memo` for data that doesn't need frequent updates
2. **Error Handling**: The integrated toast system will automatically handle and display errors
3. **Responsive Design**: Use aspect-ratio containers and relative sizing
4. **Performance**: Share ChartOptions instances when possible for multiple charts
5. **Accessibility**: Provide meaningful titles and axis labels

## Contributing

We welcome contributions! See our [Contributing Guide](CONTRIBUTING.md) for details.

## License

MIT or Apache-2.0, at your option.
