mod shared;
mod sync_client;
mod util;

pub use shared::{UggAPIVersions, UggError};
pub use sync_client::{DataApi, UggApi, UggApiBuilder};

#[cfg(feature = "async")]
mod async_client;

#[cfg(feature = "async")]
pub use async_client::{AsyncDataApi, AsyncUggApi, AsyncUggApiBuilder};
