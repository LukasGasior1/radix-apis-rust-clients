#[cfg(all(feature = "async", feature = "sync"))]
compile_error!("features `sync` and `async` are mutually exclusive");

#[cfg(feature = "sync")]
pub use gateway_api_client_sync::*;
#[cfg(feature = "async")]
pub use gateway_api_client_async::*;
