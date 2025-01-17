use {
	api::prompt_preset::models::SerpapiDTO,
	dioxus::prelude::*,
	dioxus_logger::tracing::{debug, error},
};
#[cfg(feature = "server")]
use {
	chrome_fastapi::codegen::types::UrlRequest,
	chrome_fastapi::codegen::Client as ChromeClient,
	futures::future::join_all,
	maestro_serpapi::client::serpapi_request,
	readability::ExtractOptions,
	std::io::Cursor,
	tokio_retry2::{strategy::ExponentialBackoff, Retry},
	url::Url,
};

#[cfg(feature = "server")]
async fn process_url(client: ChromeClient, url: Url, query: String) -> Result<SerpapiDTO, anyhow::Error> {
	let html = Retry::spawn(ExponentialBackoff::from_millis(10).factor(1).max_delay_millis(100).take(3), || async {
		Ok(client.fetch_html_fetch_html_post(&UrlRequest { url: url.to_string() }).await?.html.to_owned())
	})
	.await?;
	let readable = readability::extract(&mut Cursor::new(html), &url, ExtractOptions::default())?;
	Ok((readable, url.clone(), query.to_string()).into())
}

#[server]
pub async fn serpapi_server_request(query: String) -> Result<Vec<SerpapiDTO>, ServerFnError> {
	let chrome_client = chrome_fastapi::codegen::Client::new("http://localhost:8231");
	debug!("calling serp api");
	match serpapi_request().q(query.clone()).call().await {
		Ok(resp) => {
			debug!("got serp result");
			let fetch_futures = resp.organic_results.into_iter().filter_map(|result| Url::parse(&result.link).ok()).map(|url| {
				let chrome_client = chrome_client.clone();
				let url = url.clone();
				let query = query.clone();
				debug!("starting async move");
				async move {
					debug!("async process url");
					match process_url(chrome_client, url, query).await {
						Ok(result) => {
							debug!("have a serp dto");
							Some(result)
						},
						Err(e) => {
							error!("{e}");
							None
						},
					}
				}
			});
			debug!("joining futures");
			let joined_futures = join_all(fetch_futures).await;
			debug!("filtering nones");
			let filtered_results = joined_futures.into_iter().flatten().collect::<Vec<SerpapiDTO>>();
			debug!("{:#?}", filtered_results);
			Ok(filtered_results)
		},
		Err(e) => {
			error!("{e}");
			Err(e.into())
		},
	}
}
