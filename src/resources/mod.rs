//! API resources and the request builders for their methods.
//!
//! Request builders are created by methods on the [`Client`](crate::Client) (or on resource handles like
//! the ones returned by `Client::namespace`). Configure them with
//! their builder methods, then send them with `.send()` or by `.await`ing them.

pub mod client;
pub mod namespaces;
