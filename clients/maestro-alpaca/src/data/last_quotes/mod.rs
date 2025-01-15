pub mod last_quotes_dtos;

#[cfg(feature = "server")]
pub mod last_quotes_reqwest;

#[cfg(feature = "dioxus")]
pub mod functions;
