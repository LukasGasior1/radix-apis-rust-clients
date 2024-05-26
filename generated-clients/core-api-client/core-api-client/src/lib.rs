#[cfg(all(feature = "async", feature = "async"))]
compile_error!("features `sync` and `async` are mutually exclusive");

#[cfg(feature = "sync")]
use core_api_client_sync::*;
#[cfg(feature = "async")]
use core_api_client_async::*;
