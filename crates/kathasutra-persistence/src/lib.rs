#![forbid(unsafe_code)]

/// Marker for persistence contracts. Concrete Turso and PostgreSQL adapters
/// are introduced by the implementation backlog.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct PersistenceLayer;
