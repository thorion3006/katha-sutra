#![forbid(unsafe_code)]

/// Marker for the source execution boundary.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct SourceRuntime;
