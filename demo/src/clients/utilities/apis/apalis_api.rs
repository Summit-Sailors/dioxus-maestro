use {
	anyhow::Result,
	dioxus::{logger::tracing::info, prelude::*},
};

#[server]
pub async fn add_email_job(to: String, subject: String, body: String) -> Result<String, ServerFnError> {
	use {
		apalis::{layers::retry::RetryPolicy, prelude::*},
		apalis_core::worker::Event,
		maestro_apalis::server_ctx::{Storage, apalis_storage_from_ctx},
		std::sync::{Arc, Mutex},
	};
	let mut storage = apalis_storage_from_ctx::<crate::clients::utilities::EmailJob>().await?;

	let job = crate::clients::utilities::EmailJob { to, subject, body };
	let job_sql_ctx = storage.push(job).await?;

	// a listener to know when the job completes
	let mut listener = apalis_sql::postgres::PgListen::new(storage.pool().clone()).await?;

	// a channel to signal when the job is complete
	let (tx, rx) = tokio::sync::oneshot::channel();

	let tx = Arc::new(Mutex::new(Some(tx)));

	// a worker that will process the job
	let worker = WorkerBuilder::new("email-job").retry(RetryPolicy::retries(5)).backend(storage.clone()).build_fn(send_email);

	let tx_for_event = tx.clone();

	// a task to run the worker
	tokio::spawn(async move {
		let monitor = Monitor::new().register(worker).on_event(move |e| {
			if matches!(e.inner(), Event::Exit) {
				if let Some(sender) = tx_for_event.lock().unwrap().take() {
					let _ = sender.send(());
				}
			}
		});

		// run the monitor until the job completes
		let mut rx = rx;
		let run_result = monitor
			.run_with_signal(async {
				rx.await.ok();
				Ok(())
			})
			.await;

		if let Err(e) = run_result {
			eprintln!("Monitor error: {}", e);
		}
	});

	// a short time to ensure monitoring is started
	tokio::time::sleep(tokio::time::Duration::from_millis(1000)).await;

	Ok("Job added and processed!".to_string())
}

#[server]
pub async fn list_completed_jobs() -> Result<Vec<String>, ServerFnError> {
	use {
		apalis::prelude::*,
		maestro_apalis::server_ctx::{Storage, apalis_storage_from_ctx},
	};
	let mut storage = apalis_storage_from_ctx::<crate::clients::utilities::EmailJob>().await?;

	let results = storage.list_jobs(&apalis::prelude::State::Done, 10).await?;

	info!("Completed jobs: {:?}", results.len());

	let job_details = results
		.iter()
		.map(|result| format!("Email job to: {} \nwith message: {} \nand body: {} completed", result.args.to, result.args.subject, result.args.body))
		.collect::<Vec<String>>();

	Ok(job_details)
}

#[cfg(feature = "server")]
pub async fn send_email(job: crate::clients::utilities::EmailJob) -> Result<(), apalis::prelude::Error> {
	use tokio::time::{Duration, sleep};
	println!("Sending email to: {}", job.to);
	println!("Subject: {}", job.subject);
	println!("Body: {}", job.body);

	// email sending delay sim
	sleep(Duration::from_secs(2)).await;

	println!("Email sent successfully!");
	Ok(())
}
