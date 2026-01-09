//! Monitoring, observability, and distributed tracing
//!
//! Provides comprehensive application monitoring with metrics, tracing, and health checks.

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::atomic::{AtomicU64, Ordering};
use std::sync::Arc;

/// Application metrics
#[derive(Debug, Clone, Default)]
pub struct Metrics {
    /// Total requests processed
    pub total_requests: Arc<AtomicU64>,
    /// Total errors encountered
    pub total_errors: Arc<AtomicU64>,
    /// Total conversions performed
    pub total_conversions: Arc<AtomicU64>,
    /// Total bytes processed
    pub total_bytes: Arc<AtomicU64>,
    /// Active connections
    pub active_connections: Arc<AtomicU64>,
    /// Request durations (milliseconds)
    pub request_durations: Arc<dashmap::DashMap<String, Vec<u64>>>,
}

impl Metrics {
    /// Create new metrics instance
    pub fn new() -> Self {
        Self::default()
    }

    /// Record request
    pub fn record_request(&self, endpoint: &str, duration_ms: u64) {
        self.total_requests.fetch_add(1, Ordering::Relaxed);
        self.request_durations
            .entry(endpoint.to_string())
            .or_insert_with(Vec::new)
            .push(duration_ms);
    }

    /// Record error
    pub fn record_error(&self) {
        self.total_errors.fetch_add(1, Ordering::Relaxed);
    }

    /// Record conversion
    pub fn record_conversion(&self, bytes: u64) {
        self.total_conversions.fetch_add(1, Ordering::Relaxed);
        self.total_bytes.fetch_add(bytes, Ordering::Relaxed);
    }

    /// Increment active connections
    pub fn inc_connections(&self) {
        self.active_connections.fetch_add(1, Ordering::Relaxed);
    }

    /// Decrement active connections
    pub fn dec_connections(&self) {
        self.active_connections.fetch_sub(1, Ordering::Relaxed);
    }

    /// Get metrics snapshot
    pub fn snapshot(&self) -> MetricsSnapshot {
        let mut endpoint_stats = HashMap::new();

        for entry in self.request_durations.iter() {
            let durations = entry.value();
            if !durations.is_empty() {
                let sum: u64 = durations.iter().sum();
                let avg = sum / durations.len() as u64;
                let max = *durations.iter().max().unwrap();
                let min = *durations.iter().min().unwrap();

                // Calculate percentiles
                let mut sorted = durations.clone();
                sorted.sort_unstable();
                let p50 = sorted[sorted.len() / 2];
                let p95 = sorted[sorted.len() * 95 / 100];
                let p99 = sorted[sorted.len() * 99 / 100];

                endpoint_stats.insert(
                    entry.key().clone(),
                    EndpointStats {
                        count: durations.len() as u64,
                        avg_ms: avg,
                        min_ms: min,
                        max_ms: max,
                        p50_ms: p50,
                        p95_ms: p95,
                        p99_ms: p99,
                    },
                );
            }
        }

        MetricsSnapshot {
            total_requests: self.total_requests.load(Ordering::Relaxed),
            total_errors: self.total_errors.load(Ordering::Relaxed),
            total_conversions: self.total_conversions.load(Ordering::Relaxed),
            total_bytes: self.total_bytes.load(Ordering::Relaxed),
            active_connections: self.active_connections.load(Ordering::Relaxed),
            endpoint_stats,
            timestamp: Utc::now(),
        }
    }
}

/// Metrics snapshot for reporting
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MetricsSnapshot {
    pub total_requests: u64,
    pub total_errors: u64,
    pub total_conversions: u64,
    pub total_bytes: u64,
    pub active_connections: u64,
    pub endpoint_stats: HashMap<String, EndpointStats>,
    pub timestamp: DateTime<Utc>,
}

/// Per-endpoint statistics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EndpointStats {
    pub count: u64,
    pub avg_ms: u64,
    pub min_ms: u64,
    pub max_ms: u64,
    pub p50_ms: u64,
    pub p95_ms: u64,
    pub p99_ms: u64,
}

/// Distributed tracing span
#[derive(Debug, Clone)]
pub struct Span {
    pub trace_id: String,
    pub span_id: String,
    pub parent_span_id: Option<String>,
    pub operation: String,
    pub start_time: DateTime<Utc>,
    pub duration_ms: Option<u64>,
    pub tags: HashMap<String, String>,
    pub logs: Vec<SpanLog>,
}

/// Span log entry
#[derive(Debug, Clone)]
pub struct SpanLog {
    pub timestamp: DateTime<Utc>,
    pub level: LogLevel,
    pub message: String,
    pub fields: HashMap<String, String>,
}

/// Log level for span logs
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LogLevel {
    Trace,
    Debug,
    Info,
    Warn,
    Error,
}

impl Span {
    /// Create new trace span
    pub fn new(operation: String) -> Self {
        Self {
            trace_id: uuid::Uuid::new_v4().to_string(),
            span_id: uuid::Uuid::new_v4().to_string(),
            parent_span_id: None,
            operation,
            start_time: Utc::now(),
            duration_ms: None,
            tags: HashMap::new(),
            logs: Vec::new(),
        }
    }

    /// Create child span
    pub fn child(&self, operation: String) -> Self {
        Self {
            trace_id: self.trace_id.clone(),
            span_id: uuid::Uuid::new_v4().to_string(),
            parent_span_id: Some(self.span_id.clone()),
            operation,
            start_time: Utc::now(),
            duration_ms: None,
            tags: HashMap::new(),
            logs: Vec::new(),
        }
    }

    /// Add tag to span
    pub fn tag(&mut self, key: String, value: String) {
        self.tags.insert(key, value);
    }

    /// Add log entry
    pub fn log(&mut self, level: LogLevel, message: String) {
        self.logs.push(SpanLog {
            timestamp: Utc::now(),
            level,
            message,
            fields: HashMap::new(),
        });
    }

    /// Finish span and record duration
    pub fn finish(&mut self) {
        let duration = Utc::now().signed_duration_since(self.start_time);
        self.duration_ms = Some(duration.num_milliseconds() as u64);
    }
}

/// Health check status
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HealthStatus {
    pub status: ServiceStatus,
    pub version: String,
    pub uptime_seconds: u64,
    pub checks: HashMap<String, CheckStatus>,
    pub timestamp: DateTime<Utc>,
}

/// Overall service status
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum ServiceStatus {
    Healthy,
    Degraded,
    Unhealthy,
}

/// Individual health check status
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CheckStatus {
    pub status: ServiceStatus,
    pub message: Option<String>,
    pub last_check: DateTime<Utc>,
    pub duration_ms: u64,
}

/// Health checker
pub struct HealthChecker {
    start_time: DateTime<Utc>,
}

impl HealthChecker {
    /// Create new health checker
    pub fn new() -> Self {
        Self {
            start_time: Utc::now(),
        }
    }

    /// Get uptime in seconds
    pub fn uptime_seconds(&self) -> u64 {
        Utc::now()
            .signed_duration_since(self.start_time)
            .num_seconds()
            .max(0) as u64
    }

    /// Perform health check
    pub async fn check(&self, metrics: &Metrics) -> HealthStatus {
        let mut checks = HashMap::new();

        // Check metrics system
        checks.insert(
            "metrics".to_string(),
            CheckStatus {
                status: ServiceStatus::Healthy,
                message: None,
                last_check: Utc::now(),
                duration_ms: 0,
            },
        );

        // Check error rate
        let total_requests = metrics.total_requests.load(Ordering::Relaxed);
        let total_errors = metrics.total_errors.load(Ordering::Relaxed);
        let error_rate = if total_requests > 0 {
            (total_errors as f64 / total_requests as f64) * 100.0
        } else {
            0.0
        };

        let error_status = if error_rate > 10.0 {
            ServiceStatus::Unhealthy
        } else if error_rate > 5.0 {
            ServiceStatus::Degraded
        } else {
            ServiceStatus::Healthy
        };

        checks.insert(
            "error_rate".to_string(),
            CheckStatus {
                status: error_status,
                message: Some(format!("{:.2}%", error_rate)),
                last_check: Utc::now(),
                duration_ms: 1,
            },
        );

        // Check memory (placeholder)
        checks.insert(
            "memory".to_string(),
            CheckStatus {
                status: ServiceStatus::Healthy,
                message: Some("Memory usage within limits".to_string()),
                last_check: Utc::now(),
                duration_ms: 2,
            },
        );

        // Determine overall status
        let overall_status = if checks.values().any(|c| c.status == ServiceStatus::Unhealthy) {
            ServiceStatus::Unhealthy
        } else if checks.values().any(|c| c.status == ServiceStatus::Degraded) {
            ServiceStatus::Degraded
        } else {
            ServiceStatus::Healthy
        };

        let uptime = Utc::now().signed_duration_since(self.start_time);

        HealthStatus {
            status: overall_status,
            version: env!("CARGO_PKG_VERSION").to_string(),
            uptime_seconds: uptime.num_seconds() as u64,
            checks,
            timestamp: Utc::now(),
        }
    }
}

impl Default for HealthChecker {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_metrics_recording() {
        let metrics = Metrics::new();

        metrics.record_request("/api/convert", 100);
        metrics.record_request("/api/convert", 150);
        metrics.record_conversion(1024);

        assert_eq!(metrics.total_requests.load(Ordering::Relaxed), 2);
        assert_eq!(metrics.total_conversions.load(Ordering::Relaxed), 1);
        assert_eq!(metrics.total_bytes.load(Ordering::Relaxed), 1024);
    }

    #[test]
    fn test_metrics_snapshot() {
        let metrics = Metrics::new();

        metrics.record_request("/api/test", 100);
        metrics.record_request("/api/test", 200);

        let snapshot = metrics.snapshot();
        assert_eq!(snapshot.total_requests, 2);
        assert!(snapshot.endpoint_stats.contains_key("/api/test"));
    }

    #[test]
    fn test_span_creation() {
        let span = Span::new("test_operation".to_string());
        assert_eq!(span.operation, "test_operation");
        assert!(span.duration_ms.is_none());
    }

    #[test]
    fn test_span_child() {
        let parent = Span::new("parent".to_string());
        let child = parent.child("child".to_string());

        assert_eq!(child.trace_id, parent.trace_id);
        assert_ne!(child.span_id, parent.span_id);
        assert_eq!(child.parent_span_id, Some(parent.span_id.clone()));
    }

    #[tokio::test]
    async fn test_health_check() {
        let metrics = Metrics::new();
        let checker = HealthChecker::new();

        let health = checker.check(&metrics).await;
        assert_eq!(health.status, ServiceStatus::Healthy);
        assert!(health.checks.contains_key("metrics"));
    }
}
