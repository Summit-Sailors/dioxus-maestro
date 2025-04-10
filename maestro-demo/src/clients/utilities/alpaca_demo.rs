use {
	crate::components::ui::features::Features,
	dioxus::prelude::*,
	maestro_alpaca::data::{
		bars::bars_dtos::{BarsDTO, BarsSingleRequestDTO, NewBar},
		enums::{feed::Feed, timeframe::TimeFrame},
	},
};

#[component]
pub fn AlpacaDemo() -> Element {
	// stocks to choose from
	let stock_symbols = use_signal(|| {
		vec![
			"AAPL".to_string(),
			"MSFT".to_string(),
			"GOOG".to_string(),
			"AMZN".to_string(),
			"TSLA".to_string(),
			"META".to_string(),
			"NVDA".to_string(),
			"AMD".to_string(),
			"INTC".to_string(),
		]
	});
	let mut selected_symbol = use_signal(|| "AAPL".to_string());
	let mut selected_timeframe = use_signal(|| TimeFrame::Hour);
	let mut selected_feed = use_signal(|| Feed::Iex);
	let mut selected_limit = use_signal(|| 20);
	let mut loading = use_signal(|| false);
	let mut error = use_signal(|| None::<String>);
	let mut bars_data = use_signal(|| None::<BarsDTO>);

	// trading related state
	let mut order_quantity = use_signal(|| "1".to_string());
	let order_price = use_signal(|| "".to_string());
	let mut order_status = use_signal(|| None::<String>);
	let mut show_order_form = use_signal(|| false);

	let server_bars_data = use_server_future(move || {
		loading.set(true);
		maestro_alpaca::data::bars::functions::get_alpaca_bars_from_server(
			selected_symbol(),
			BarsSingleRequestDTO::builder().timeframe(selected_timeframe()).feed(selected_feed()).limit(selected_limit() as usize).build(),
		)
	})?
	.suspend()?;

	let fetch_data = move || match &*server_bars_data.read_unchecked() {
		Ok(data) => {
			loading.set(false);
			bars_data.set(Some(data.clone()));
		},
		Err(e) => {
			loading.set(false);
			error.set(Some(format!("Error fetching data: {}", e)));
		},
	};

	let mut fetch_data_clone = fetch_data.clone();

	// submit order function
	let submit_order = move |_| {
		let symbol = selected_symbol();
		let quantity = order_quantity();
		let price = order_price();

		order_status.set(Some("Processing order...".to_string()));

		// in a real app, you would call the Alpaca API to place the order
		// this is just a placeholder for demonstration
		spawn(async move {
			// API call sim with a delay
			async_std::task::sleep(std::time::Duration::from_millis(1000)).await;

			// mock response
			order_status.set(Some(format!("Order submitted: {} shares of {} at {}", quantity, symbol, &price)));
		});
	};

	use_effect(fetch_data);

	rsx! {
		div { class: "container mx-auto p-4",
			div { class: "flex flex-col gap-3",
				h1 { class: "text-slate-100 text-center text-2xl sm:text-3xl lg:text-4xl 2xl:text-5xl font-semibold",
					"Maestro Alpaca"
				}
				p { class: "text-slate-300 text-center text-base lg:text-xl 2xl:text-2xl",
					"A Alpaca utility designed to make connecting to and using Alpaca with your Dioxus apps easier"
				}
			}

			div {
				id: "maestro-alpaca-features",
				class: "flex space-x-2 mt-4 mb-4",
				Features {
					title: "Features".to_string(),
					features: vec!["Simple integration with Dioxus".to_string(), "Comprehensive DTOs".to_string()],
				}
			}

			div { class: "grid grid-cols-1  gap-6 mb-8",
				// data controls
				div { class: "bg-gray-800 p-4 rounded shadow",
					h2 { class: "text-xl font-semibold mb-4", "Market Data Controls" }
					div { class: "grid grid-cols-1 md:grid-cols-2 gap-4 mb-4",
						div {
							label { class: "block text-sm font-medium mb-1", "Stock Symbol" }
							select {
								class: "w-full p-2 border rounded",
								value: "{selected_symbol}",
								onchange: move |evt| {
										let symbol = evt.value().clone();
										selected_symbol.set(symbol);
								},

								{
										stock_symbols
												.iter()
												.map(|symbol| {
														rsx! {
															option { value: "{symbol}", "{symbol}" }
														}
												})
								}
							}
						}

						div {
							label { class: "block text-sm font-medium mb-1", "Timeframe" }
							select {
								class: "w-full p-2 border rounded",
								value: format!("{:?}", selected_timeframe()),
								onchange: move |evt| {
										let timeframe_str = evt.value().clone();
										let timeframe = match timeframe_str.as_str() {
												"Minute" => TimeFrame::Minute,
												"Hour" => TimeFrame::Hour,
												"Day" => TimeFrame::Day,
												"Month" => TimeFrame::Month,
												_ => TimeFrame::Hour,
										};
										selected_timeframe.set(timeframe);
								},

								option { value: "Minute", "1 Minute" }
								option { value: "Hour", "1 Hour" }
								option { value: "Day", "1 Day" }
								option { value: "Month", "1 Month" }
							}
						}

						div {
							label { class: "block text-sm font-medium mb-1", "Data Feed" }
							select {
								class: "w-full p-2 border rounded",
								value: format!("{:?}", selected_feed()),
								onchange: move |evt| {
										let feed_str = evt.value().clone();
										let feed = match feed_str.as_str() {
												"Iex" => Feed::Iex,
												"Sip" => Feed::Sip,
												_ => Feed::Iex,
										};
										selected_feed.set(feed);
								},

								option { value: "Iex", "IEX" }
								option { value: "Sip", "SIP" }
								option { value: "Otc", "OTC" }
							}
						}

						div {
							label { class: "block text-sm font-medium mb-1", "Limit" }
							input {
								class: "w-full p-2 border rounded",
								r#type: "number",
								min: "1",
								max: "100",
								value: "{selected_limit}",
								oninput: move |evt| {
										if let Ok(limit) = evt.value().parse::<i32>() {
												selected_limit.set(limit);
										}
								},
							}
						}
					}

					div { class: "flex justify-end",
						button {
							class: "px-4 py-2 bg-blue-500 text-white rounded hover:bg-blue-600",
							disabled: loading(),
							onclick: move |_| fetch_data_clone(),

							if loading() {
								"Loading..."
							} else {
								"Fetch Data"
							}
						}
					}
				}

				// trading controls
				div { class: "bg-gray-800 p-4 rounded shadow",
					h2 { class: "text-xl font-semibold mb-4", "Trading Controls" }

					button {
						class: "px-4 py-2 bg-green-500 text-white rounded hover:bg-green-600 mb-4",
						onclick: move |_| show_order_form.set(!show_order_form()),

						if show_order_form() {
							"Hide Order Form"
						} else {
							"Show Order Form"
						}
					}

					if show_order_form() {
						div { class: "border-t pt-4 mt-2",
							div { class: "grid grid-cols-1 md:grid-cols-2 gap-4 mb-4",

								div {
									label { class: "block text-sm font-medium mb-1", "Quantity" }
									input {
										class: "w-full p-2 border rounded",
										r#type: "text",
										value: "{order_quantity}",
										oninput: move |evt| {
												order_quantity.set(evt.value().clone());
										},
									}
								}
							}

							div { class: "flex justify-end mt-4",
								button {
									class: "px-4 py-2 bg-green-500 text-white rounded hover:bg-green-600",
									onclick: submit_order,
									"Submit Order"
								}
							}

							if let Some(status) = order_status() {
								div { class: "mt-4 p-2 bg-blue-100 text-blue-800 rounded",
									"{status}"
								}
							}
						}
					}
				}

				{
						if let Some(data) = &bars_data() {
								rsx! {
									StockChart { bars: data.clone() }
								}
						} else if let Some(err) = error() {
								rsx! {
									div { class: "p-4 bg-red-100 text-red-800 rounded", "Error: {err}" }
								}
						} else {
								rsx! {
									div { class: "p-4 bg-gray-100 text-gray-800 rounded", "No data available" }
								}
						}
				}
			}
		}
	}
}

#[component]
fn StockChart(bars: BarsDTO) -> Element {
	let mut chart_type = use_signal(|| "table");

	rsx! {
		div { class: "bg-gray-800 p-4 rounded shadow w-full",
			div { class: "flex justify-between items-center mb-4",
				h2 { class: "text-xl font-semibold", "Price History for {bars.symbol}" }

				div { class: "flex space-x-2",
					button {
						class: "px-3 py-1 border rounded hover:bg-gray-400",
						class: if chart_type() == "table" { "bg-blue-500" } else { "" },
						onclick: move |_| chart_type.set("table"),
						"Table View"
					}
					button {
						class: "px-3 py-1 border rounded hover:bg-gray-400",
						class: if chart_type() == "stats" { "bg-blue-500" } else { "" },
						onclick: move |_| chart_type.set("stats"),
						"Statistics"
					}
				}
			}

			{
					if chart_type() == "table" {
							rsx! {
								div { class: "overflow-x-auto max-h-96",
									table { class: "w-full border-collapse",
										thead {
											tr { class: "bg-gray-500",
												th { class: "p-2 text-left", "Time" }
												th { class: "p-2 text-right", "Open" }
												th { class: "p-2 text-right", "High" }
												th { class: "p-2 text-right", "Low" }
												th { class: "p-2 text-right", "Close" }
												th { class: "p-2 text-right", "Volume" }
												th { class: "p-2 text-right", "Change" }
											}
										}
										tbody {
											{
													bars.bars
															.iter()
															.map(|bar| {
																	let time = bar.time.format("%Y-%m-%d %H:%M").to_string();
																	let price_change = bar.close - bar.open;
																	let price_change_percent = (price_change / bar.open) * 100.0;
																	let price_class = if price_change > 0.0 {
																			"text-green-600"
																	} else if price_change < 0.0 {
																			"text-red-600"
																	} else {
																			""
																	};
																	rsx! {
																		tr { class: "border-b hover:bg-gray-400",
																			td { class: "p-2 text-left", "{time}" }
																			td { class: "p-2 text-right", "{bar.open:.2}" }
																			td { class: "p-2 text-right", "{bar.high:.2}" }
																			td { class: "p-2 text-right", "{bar.low:.2}" }
																			td { class: "p-2 text-right {price_class}", "{bar.close:.2}" }
																			td { class: "p-2 text-right", "{bar.volume}" }
																			td { class: "p-2 text-right {price_class}", "{price_change:.2} ({price_change_percent:.2}%)" }
																		}
																	}
															})
											}
										}
									}
								}
							}
					} else {
							rsx! {
								ExtendedStockStats { bars: bars.bars }
							}
					}
			}
		}
	}
}

#[component]
fn ExtendedStockStats(bars: Vec<NewBar>) -> Element {
	// basic statistics
	let latest_bar = bars.first().unwrap();
	let earliest_bar = bars.last().unwrap();
	let period_change = latest_bar.close - earliest_bar.close;
	let period_change_percent = (period_change / earliest_bar.close) * 100.0;

	let highest_price = bars.iter().map(|b| b.high).fold(f32::MIN, f32::max);
	let lowest_price = bars.iter().map(|b| b.low).fold(f32::MAX, f32::min);
	let avg_price = bars.iter().map(|b| b.close).sum::<f32>() / bars.len() as f32;
	let total_volume = bars.iter().map(|b| b.volume).sum::<i32>();
	let avg_volume = total_volume / bars.len() as i32;

	// volatility
	let mean_close = bars.iter().map(|b| b.close).sum::<f32>() / bars.len() as f32;
	let variance = bars.iter().map(|b| (b.close - mean_close).powi(2)).sum::<f32>() / bars.len() as f32;
	let volatility = variance.sqrt();

	// price movement direction count
	let up_days = bars.windows(2).filter(|w| w[0].close > w[1].close).count();
	let down_days = bars.windows(2).filter(|w| w[0].close < w[1].close).count();
	let flat_days = bars.windows(2).filter(|w| w[0].close == w[1].close).count();

	// class based on price movement
	let change_class = if period_change > 0.0 {
		"text-green-600"
	} else if period_change < 0.0 {
		"text-red-600"
	} else {
		""
	};

	rsx! {
		div { class: "space-y-6",
			div { class: "mt-4 grid grid-cols-1 md:grid-cols-3 gap-4",
				div { class: "p-4 border rounded",
					h3 { class: "font-medium text-gray-600", "Period Change" }
					p { class: "text-xl {change_class}",
						"{period_change:.2} ({period_change_percent:.2}%)"
					}
				}

				div { class: "p-4 border rounded",
					h3 { class: "font-medium text-gray-600", "Price Range" }
					p { class: "text-xl", "Low: {lowest_price:.2} - High: {highest_price:.2}" }
					p { class: "text-sm text-gray-500", "Avg: {avg_price:.2}" }
				}

				div { class: "p-4 border rounded",
					h3 { class: "font-medium text-gray-600", "Volume" }
					p { class: "text-xl", "Total: {total_volume}" }
					p { class: "text-sm text-gray-500", "Avg: {avg_volume}" }
				}
			}

			// advanced statistics
			div { class: "mt-4 grid grid-cols-1 md:grid-cols-3 gap-4",
				div { class: "p-4 border rounded",
					h3 { class: "font-medium text-gray-600", "Volatility" }
					p { class: "text-xl", "{volatility:.4}" }
				}

				div { class: "p-4 border rounded",
					h3 { class: "font-medium text-gray-600", "Price Movement" }
					div { class: "flex items-center justify-between",
						span { class: "text-green-600", "Up: {up_days}" }
						span { class: "text-red-600", "Down: {down_days}" }
						span { class: "text-gray-600", "Flat: {flat_days}" }
					}
				}


				div { class: "p-4 border rounded",
					h3 { class: "font-medium text-gray-600", "Latest Price" }
					p { class: "text-xl {change_class}", "{latest_bar.close:.2}" }
					p { class: "text-sm text-gray-500",
						{format!("{}", latest_bar.time.format("%Y-%m-%d %H:%M"))}
					}
				}
			}
		}
	}
}
