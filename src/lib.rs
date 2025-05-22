//! OpenTelemetry utilities for Rust applications.
//!
//! This crate provides convenient abstractions and utilities for working with
//! OpenTelemetry in Rust applications. It simplifies the process of setting up
//! telemetry providers and ensures proper resource cleanup.
//!
//! # Features
//!
//! - Unified management of OpenTelemetry providers (logs, metrics, traces)
//! - Automatic resource cleanup through Drop implementations
//! - Error handling for telemetry operations

pub mod keys;
/// Module containing OpenTelemetry provider management functionality
pub mod providers;
