#[cfg(feature = "diesel-async")]
pub mod async_client;
#[cfg(feature = "diesel-async")]
pub mod async_types;

#[cfg(feature = "diesel-sync")]
pub mod sync_client;
#[cfg(feature = "diesel-sync")]
pub mod sync_types;
