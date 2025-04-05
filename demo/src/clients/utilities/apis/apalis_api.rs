use {anyhow::Result, dioxus::prelude::*};

#[server]
pub async fn add_email_job(to: String, subject: String, body: String) -> Result<(), ServerFnError> {
	use maestro_apalis::server_ctx::{Storage, apalis_storage_from_ctx};
	let mut storage = apalis_storage_from_ctx::<crate::clients::utilities::EmailJob>().await?;

	let job = crate::clients::utilities::EmailJob { to, subject, body };
	storage.push(job).await?;

	Ok(())
}

#[server]
pub async fn list_pending_jobs() -> Result<String, ServerFnError> {
	use maestro_apalis::server_ctx::{Storage, apalis_storage_from_ctx};
	let storage = apalis_storage_from_ctx::<crate::clients::utilities::EmailJob>().await?;

	let num_jobs = storage.len().await?;

	let job_details = format!("{} jobs in storage", num_jobs);

	Ok(job_details)
}

#[server]
pub async fn send_email(job: crate::clients::utilities::EmailJob) -> Result<(), ServerFnError> {
	use tokio::time::{Duration, sleep};
	println!("Sending email to: {}", job.to);
	println!("Subject: {}", job.subject);
	println!("Body: {}", job.body);

	// email sending delay sim
	sleep(Duration::from_secs(2)).await;

	println!("Email sent successfully!");
	Ok(())
}
