# [`Substreams`](https://substreams.streamingfast.io/) Sink to Prometheus 

[<img alt="github" src="https://img.shields.io/badge/Github-substreams.prometheus-8da0cb?style=for-the-badge&logo=github" height="20">](https://github.com/pinax-network/substreams-sink-prometheus)
[<img alt="crates.io" src="https://img.shields.io/crates/v/substreams-sink-prometheus.svg?style=for-the-badge&color=fc8d62&logo=rust" height="20">](https://crates.io/crates/substreams-sink-prometheus)
[<img alt="docs.rs" src="https://img.shields.io/badge/docs.rs-substreams.prometheus-66c2a5?style=for-the-badge&labelColor=555555&logo=docs.rs" height="20">](https://docs.rs/substreams-sink-prometheus)
[<img alt="GitHub Workflow Status" src="https://img.shields.io/github/actions/workflow/status/pinax-network/substreams-sink-prometheus/ci.yml?branch=develop&style=for-the-badge" height="20">](https://github.com/pinax-network/substreams-sink-prometheus/actions?query=branch%3Adevelop)

> `substreams-sink-prom` is a tool that allows developers to pipe data extracted from a blockchain into a Prometheus time series store.

## 📖 Documentation

### https://docs.rs/substreams-sink-prometheus

### Further resources

- [Substreams documentation](https://substreams.streamingfast.io)

## 🛠 Feature Roadmap

- [ ] [Counter](https://pkg.go.dev/github.com/prometheus/client_golang/prometheus#Counter)
  - [ ] Inc
  - [ ] Add
- [ ] [Gauge](https://pkg.go.dev/github.com/prometheus/client_golang/prometheus#Gauge)
  - [ ] Set
  - [ ] Inc
  - [ ] Dec
  - [ ] Add
  - [ ] Sub
  - [ ] SetToCurrentTime

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
use substreams_sink_prometheus::PrometheusOperations;

#[substreams::handlers::map]
fn map_action_traces(
    ... some stores ...
) -> Result<PrometheusOperations, Error> {

    let mut prom_ops: PrometheusOperations = Default::default();

    // process your data, push to Prometheus
    prom_ops.push_new(someKey, someValue, ordinal);
    prom_ops.push_delete(anotherKey, anotherOrdinal);

    Ok(prom_ops)
}
```
