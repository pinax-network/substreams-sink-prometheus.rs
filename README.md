# [`Substreams`](https://substreams.streamingfast.io/) [Prometheus](https://prometheus.io/) sink module

[<img alt="github" src="https://img.shields.io/badge/Github-substreams.prometheus-8da0cb?style=for-the-badge&logo=github" height="20">](https://github.com/pinax-network/substreams-sink-prometheus)
[<img alt="crates.io" src="https://img.shields.io/crates/v/substreams-sink-prometheus.svg?style=for-the-badge&color=fc8d62&logo=rust" height="20">](https://crates.io/crates/substreams-sink-prometheus)
[<img alt="docs.rs" src="https://img.shields.io/badge/docs.rs-substreams.prometheus-66c2a5?style=for-the-badge&labelColor=555555&logo=docs.rs" height="20">](https://docs.rs/substreams-sink-prometheus)
[<img alt="GitHub Workflow Status" src="https://img.shields.io/github/actions/workflow/status/pinax-network/substreams-sink-prometheus/ci.yml?branch=main&style=for-the-badge" height="20">](https://github.com/pinax-network/substreams-sink-prometheus/actions?query=branch%3Amain)

> `substreams-sink-prometheus` is a tool that allows developers to pipe data extracted metrics from a blockchain into a Prometheus time series database.

## 📖 Documentation

### https://docs.rs/substreams-sink-prometheus

### Further resources

- [Substreams documentation](https://substreams.streamingfast.io)
- [Prometheus documentation](https://prometheus.io)

## 🛠 Feature Roadmap

### [Gauge Metric](https://pkg.go.dev/github.com/prometheus/client_golang/prometheus#Gauge)
- [x] Set
- [x] Inc
- [x] Dec
- [x] Add
- [x] Sub
- [x] SetToCurrentTime

### [Counter Metric](https://pkg.go.dev/github.com/prometheus/client_golang/prometheus#Counter)
- [x] Inc
- [x] Add

### [Histogram Metric](https://pkg.go.dev/github.com/prometheus/client_golang/prometheus#Histogram)
- [ ] Observe
- [ ] buckets
- [ ] zero

### [Summary Metric](https://pkg.go.dev/github.com/prometheus/client_golang/prometheus#Summary)
> Summaries calculate percentiles of observed values.
- [ ] Observe
- [ ] percentiles
- [ ] maxAgeSeconds
- [ ] ageBuckets
- [ ] startTimer

### [Registry](https://pkg.go.dev/github.com/prometheus/client_golang/prometheus#Registry)
- [ ] Clear
- [ ] SetDefaultLabels
- [ ] RemoveSingleMetric

## Install

```
$ cargo add substreams-sink-prometheus
```

## Quickstart

**Cargo.toml**

```toml
[dependencies]
substreams = "0.5"
substreams-sink-prometheus = "0.1"
```

**src/lib.rs**

```rust
use substreams::prelude::*;
use substreams::errors::Error;
use substreams_sink_prometheus::PrometheusOperations;

#[substreams::handlers::map]
fn prom_out(
    ... some stores ...
) -> Result<PrometheusOperations, Error> {

    let mut prom_ops: PrometheusOperations = Default::default();

    // Gauge Metric
    // ============
    // SET Gauge to an arbitrary value.
    prom_ops.push_set("gauge_name", 123.456, vec![], None);
    
    // Adds an arbitrary value to a Gauge. (The value can be negative, resulting in a decrease of the Gauge.)
    prom_ops.push_add("gauge_name", 123.456, vec![], None);

    // SET Gauge using labels & help documentation
    prom_ops.push_set("custom_gauge", 50.0, vec!["custom_label"], Some("Gauge documentation"));

    // Counter Metric
    // ==============
    // process your data, push to Prometheus metrics
    prom_ops.push_counter_inc("counter_name", vec![], None);

    // Adds an arbitrary value to a Counter. (Returns an error if the value is < 0.)
    prom_ops.push_counter_add("counter_name", 123.456, vec![], None);

    Ok(prom_ops)
}
```
