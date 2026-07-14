#![forbid(unsafe_code)]

/// Marker for the identity and authorization boundary.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct AuthLayer;
