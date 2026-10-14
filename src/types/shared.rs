//! Types shared across resources.

#[allow(unused_imports)]
use super::*;

use serde::{Deserialize, Serialize};

/// A summary of a namespace.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct NamespaceSummary {
    /// The namespace ID.
    pub id: String,
}
