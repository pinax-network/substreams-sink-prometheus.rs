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
//! - [ ] Observe
//! - [ ] buckets
//! - [ ] zero
//! 
//! ### [Summary Metric](https://pkg.go.dev/github.com/prometheus/client_golang/prometheus#Summary)
//! > Summaries calculate percentiles of observed values.
//! - [ ] Observe
//! - [ ] percentiles
//! - [ ] maxAgeSeconds
//! - [ ] ageBuckets
//! - [ ] startTimer
//! 
//! ### [Registry](https://pkg.go.dev/github.com/prometheus/client_golang/prometheus#Summary)
//! - [ ] Clear
//! - [ ] SetDefaultLabels
//! - [ ] RemoveSingleMetric
//!
//! ### Example
//! ```
//! use substreams_sink_prometheus::{PrometheusOperations, Gauge, Counter};
//! let mut prom_ops: PrometheusOperations = Default::default();
//!
//! // Gauge Metric
//! // ============
//! // Initialize Gauge with a name & labels
//! let mut gauge = Gauge::new("gauge_name");
//! gauge.set_label("custom_label");
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
//! // Counter Metric
//! // ==============
//! // Increments the Counter by 1.
//! let mut counter = Counter::new("counter_name");
//! prom_ops.push(counter.inc());
//! 
//! // Adds an arbitrary value to a Counter. (Returns an error if the value is < 0.)
//! prom_ops.push(counter.add(123.456));
//!
//! // Set Counter to the current Unix time in seconds.
//! prom_ops.push(counter.set_to_current_time());
//! ```

#[path = "pb/pinax.substreams.sink.prometheus.v1.rs"]
#[allow(dead_code)]
pub mod pb;
pub use self::pb::*;

mod helpers;
mod counter;
pub use self::counter::*;
mod gauge;
pub use self::gauge::*;