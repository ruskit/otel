use opentelemetry_sdk::{
    logs::SdkLoggerProvider, metrics::SdkMeterProvider, trace::SdkTracerProvider,
};
use tracing::{error, info};

#[derive(Debug, Clone)]
pub struct OtelProviders {
    pub log: SdkLoggerProvider,
    pub metric: SdkMeterProvider,
    pub trace: SdkTracerProvider,
}

impl OtelProviders {
    pub fn new(log: SdkLoggerProvider, metric: SdkMeterProvider, trace: SdkTracerProvider) -> Self {
        OtelProviders { log, metric, trace }
    }
}

impl Drop for OtelProviders {
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
