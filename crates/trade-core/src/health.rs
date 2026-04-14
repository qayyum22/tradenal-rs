//! Minimal health primitives used during repository bootstrap.

use thiserror::Error;

/// High-level health state for simple smoke checks.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum HealthStatus {
    /// The application is healthy.
    Healthy,
}

impl HealthStatus {
    /// Returns a stable string representation for output.
    #[must_use]
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Healthy => "healthy",
        }
    }
}

/// Errors that can occur while performing simple bootstrap checks.
#[derive(Debug, Error)]
pub enum HealthError {
    /// Returned when an unknown command or invalid state is encountered.
    #[error("invalid health state")]
    InvalidState,
}

/// Returns the current health status.
///
/// This function is intentionally simple for the bootstrap PR.
#[must_use]
pub const fn healthcheck() -> HealthStatus {
    HealthStatus::Healthy
}

#[cfg(test)]
mod tests {
    use super::{healthcheck, HealthStatus};

    #[test]
    fn healthcheck_returns_healthy() {
        assert_eq!(healthcheck(), HealthStatus::Healthy);
    }

    #[test]
    fn health_status_as_str_is_stable() {
        assert_eq!(HealthStatus::Healthy.as_str(), "healthy");
    }
}
