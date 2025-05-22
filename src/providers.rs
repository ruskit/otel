//! OpenTelemetry providers management module.
//!
//! This module provides functionality to create and manage OpenTelemetry providers
//! for logs, metrics, and traces. It ensures proper shutdown sequence when the
//! providers are dropped.

use opentelemetry_sdk::{
    logs::SdkLoggerProvider, metrics::SdkMeterProvider, trace::SdkTracerProvider,
};
use tracing::{error, info};

/// A container for OpenTelemetry provider instances.
///
/// This struct holds references to the three main OpenTelemetry providers:
/// - Logger provider for logs
/// - Meter provider for metrics
/// - Tracer provider for distributed tracing
#[derive(Debug, Clone)]
pub struct OtelProviders {
    pub log: SdkLoggerProvider,
    pub metric: SdkMeterProvider,
    pub trace: SdkTracerProvider,
}

impl OtelProviders {
    /// Creates a new instance of `OtelProviders`.
    ///
    /// # Parameters
    ///
    /// * `log` - The SDK logger provider for OpenTelemetry logs
    /// * `metric` - The SDK meter provider for OpenTelemetry metrics
    /// * `trace` - The SDK tracer provider for OpenTelemetry traces
    ///
    /// # Returns
    ///
    /// A new `OtelProviders` instance containing the provided components
    pub fn new(log: SdkLoggerProvider, metric: SdkMeterProvider, trace: SdkTracerProvider) -> Self {
        OtelProviders { log, metric, trace }
    }
}

/// Ensures proper cleanup of OpenTelemetry resources when an instance is dropped.
///
/// The Drop implementation follows a specific shutdown order to ensure resources
/// are cleaned up properly, with each provider shutdown logged for monitoring.
impl Drop for OtelProviders {
    /// Shuts down all providers in the recommended order (trace, metric, log).
    ///
    /// Logs success or failure for each provider shutdown operation.
    fn drop(&mut self) {
        // Shutdown the providers in reverse order of importance
        // First shut down the tracer provider
        match self.trace.shutdown() {
            Ok(_) => info!("tracer provider shut down successfully."),
            Err(e) => error!("failed to shut down tracer provider: {}", e),
        };

        // Then shut down the meter provider
        match self.metric.shutdown() {
            Ok(_) => info!("meter provider shut down successfully."),
            Err(e) => error!("failed to shut down meter provider: {}", e),
        };

        // Finally shut down the logger provider
        match self.log.shutdown() {
            Ok(_) => info!("logger provider shut down successfully."),
            Err(e) => error!("failed to shut down logger provider: {}", e),
        }
    }
}
