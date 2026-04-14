//! Core domain building blocks for Tradenal.

pub mod health;

pub use health::{healthcheck, HealthError, HealthStatus};
