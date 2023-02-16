//! [![github]](https://github.com/pinax-network/substreams-sink-prometheus)&ensp;[![crates-io]](https://crates.io/crates/substreams-sink-prometheus)&ensp;[![docs-rs]](crate)
//!
//! [github]: https://img.shields.io/badge/github-8da0cb?style=for-the-badge&labelColor=555555&logo=github
//! [crates-io]: https://img.shields.io/badge/crates.io-fc8d62?style=for-the-badge&labelColor=555555&logo=rust
//! [docs-rs]: https://img.shields.io/badge/docs.rs-66c2a5?style=for-the-badge&labelColor=555555&logo=docs.rs
//!
//! > `substreams-sink-prom` is a tool that allows developers to pipe metrics extracted from a blockchain into a Prometheus time series database.
//!
//! ## 🛠 Feature Roadmap
//!
//! ### [Gauge Metric](https://pkg.go.dev/github.com/prometheus/client_golang/prometheus#Gauge)
//! - [x] Set
//! - [x] Inc
//! - [x] Dec
//! - [x] Add
//! - [x] Sub
//! - [x] SetToCurrentTime
//!
//! ### [Counter Metric](https://pkg.go.dev/github.com/prometheus/client_golang/prometheus#Counter)
//! - [x] Inc
//! - [x] Add
//!
//! ### [Histogram Metric](https://pkg.go.dev/github.com/prometheus/client_golang/prometheus#Histogram)
//! - [x] Observe
//!   - [ ] buckets
//! - [x] Zero
//! 
//! ### [Summary Metric](https://pkg.go.dev/github.com/prometheus/client_golang/prometheus#Summary)
//! > Summaries calculate percentiles of observed values.
//! - [x] Observe
//!   - [ ] percentiles
//!   - [ ] maxAgeSeconds
//!   - [ ] ageBuckets
//!   - [ ] compressCount
//! - [x] StartTimer
//!
//! ### [Registry](https://pkg.go.dev/github.com/prometheus/client_golang/prometheus#Summary)
//! - [ ] Clear
//! - [ ] SetDefaultLabels
//! - [ ] RemoveSingleMetric
//!
//! ### Example
//! ```
//! use std::collections::HashMap;
//! use substreams_sink_prometheus::{PrometheusOperations, Gauge, Counter, Summary, Histogram};
//!
//! // Initialize Prometheus Operations container
//! let mut prom_ops: PrometheusOperations = Default::default();
//!
//! // Counter Metric
//! // ==============
//! // Initialize Gauge with a name & labels
//! let mut counter = Counter::from("counter_name");
//!
//! // Increments the Counter by 1.
//! prom_ops.push(counter.inc());
//!
//! // Adds an arbitrary value to a Counter. (Returns an error if the value is < 0.)
//! prom_ops.push(counter.add(123.456));
//!
//! // Labels
//! // ======
//! // Create a HashMap of labels
//! // Labels represents a collection of label name -> value mappings. 
//! let labels1 = HashMap::from([("label1".to_string(), "value1".to_string())]);
//! let mut labels2 = HashMap::new();
//! labels2.insert("label2".to_string(), "value2".to_string());
//! 
//! // Gauge Metric
//! // ============
//! // Initialize Gauge
//! let mut gauge = Gauge::from("gauge_name").with(labels1);
//!
//! // Sets the Gauge to an arbitrary value.
//! prom_ops.push(gauge.set(88.8));
//!
//! // Increments the Gauge by 1.
//! prom_ops.push(gauge.inc());
//!
//! // Decrements the Gauge by 1.
//! prom_ops.push(gauge.dec());
//!
//! // Adds an arbitrary value to a Gauge. (The value can be negative, resulting in a decrease of the Gauge.)
//! prom_ops.push(gauge.add(50.0));
//! prom_ops.push(gauge.add(-10.0));
//!
//! // Subtracts arbitrary value from the Gauge. (The value can be negative, resulting in an increase of the Gauge.)
//! prom_ops.push(gauge.sub(25.0));
//! prom_ops.push(gauge.sub(-5.0));
//!
//! // Set Gauge to the current Unix time in seconds.
//! prom_ops.push(gauge.set_to_current_time());
//! 
//! // Remove metrics for the given label values
//! prom_ops.push(gauge.remove(labels2));
//! 
//! // Reset gauge values
//! prom_ops.push(gauge.reset());
//! 
//! // Summary Metric
//! // ==============
//! // Initialize Summary
//! let mut summary = Summary::from("summary_name");
//!
//! /// Observe adds a single observation to the summary.
//! /// Observations are usually positive or zero.
//! /// Negative observations are accepted but prevent current versions of Prometheus from properly detecting counter resets in the sum of observations
//! prom_ops.push(summary.observe(88.8));
//! 
//! // Start a timer. Calling the returned function will observe the duration in seconds in the summary.
//! prom_ops.push(summary.start_timer());
//! 
//! // Histogram Metric
//! // ==============
//! // Initialize Summary
//! let mut histogram = Histogram::from("histogram_name");
//!
//! /// Observe adds a single observation to the histogram.
//! /// Observations are usually positive or zero.
//! /// Negative observations are accepted but prevent current versions of Prometheus from properly detecting counter resets in the sum of observations
//! prom_ops.push(histogram.observe(88.8));
//! 
//! // Start a timer. Calling the returned function will observe the duration in seconds in the histogram.
//! prom_ops.push(histogram.start_timer());
//! 
//! // Initialize the metrics for the given combination of labels to zero
//! prom_ops.push(histogram.zero(HashMap::from([("label1".to_string(), "value1".to_string())])));
//! ```
#[path = "pb/pinax.substreams.sink.prometheus.v1.rs"]
#[allow(dead_code)]
pub mod pb;
pub use self::pb::*;

mod counter;
mod helpers;
pub use self::counter::*;
mod gauge;
pub use self::gauge::*;
mod summary;
pub use self::summary::*;
mod histogram;
pub use self::histogram::*;
mod labels;
pub use self::labels::*;
