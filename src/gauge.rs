use std::collections::HashMap;

use crate::{PrometheusOperation, prometheus_operation, GaugeOp, gauge_op};

#[derive(Eq, Debug, PartialEq, Default)]
pub struct Gauge {
    pub name: String,
    pub labels: HashMap<String, String>,
}

impl Gauge {
    /// Create new Gauge
    ///
    /// ### Example
    /// ```
    /// use substreams_sink_prometheus::Gauge;
    /// let mut gauge = Gauge::from("gauge_name");
    /// ```
    #[inline]
    #[must_use]
    pub fn from(name: &str) -> Self {
        Self {
            name: name.to_string(),
            labels: Default::default(),
        }
    }

    /// Set label to Gauge
    ///
    /// ### Example
    /// ```
    /// use std::collections::HashMap;
    /// use substreams_sink_prometheus::Gauge;
    /// let mut gauge = Gauge::from("gauge_name");
    /// let mut labels = HashMap::new();
    /// labels.insert("label1".to_string(), "value1".to_string());
    /// gauge.with(labels);
    /// ```
    #[inline]
    pub fn with(mut self, labels: HashMap<String, String>) -> Self {
        self.labels = labels;
        self
    }

    /// Sets the Gauge to an arbitrary value.
    ///
    /// ### Example
    /// ```
    /// use substreams_sink_prometheus::{PrometheusOperations, Gauge};
    /// let mut prom_ops: PrometheusOperations = Default::default();
    /// prom_ops.push(Gauge::from("gauge_name").set(88.8));
    /// ```
    #[inline]
    #[must_use]
    pub fn set(&mut self, value: f64) -> PrometheusOperation {
        let op = GaugeOp {
            value,
            operation: gauge_op::Operation::Set.into(),
        };
        PrometheusOperation {
            name: self.name.to_owned(),
            labels: self.labels.to_owned(),
            operation: Some(prometheus_operation::Operation::Gauge(op)),
        }
    }

    /// Increments the Gauge by 1.
    ///
    /// ### Example
    /// ```
    /// use substreams_sink_prometheus::{PrometheusOperations, Gauge};
    /// let mut prom_ops: PrometheusOperations = Default::default();
    /// prom_ops.push(Gauge::from("gauge_name").inc());
    /// ```
    #[inline]
    #[must_use]
    pub fn inc(&mut self) -> PrometheusOperation {
        let op = GaugeOp {
            value: 1.0,
            operation: gauge_op::Operation::Inc.into(),
        };
        PrometheusOperation {
            name: self.name.to_owned(),
            labels: self.labels.to_owned(),
            operation: Some(prometheus_operation::Operation::Gauge(op)),
        }
    }

    /// Decrements the Gauge by 1.
    ///
    /// ### Example
    /// ```
    /// use substreams_sink_prometheus::{PrometheusOperations, Gauge};
    /// let mut prom_ops: PrometheusOperations = Default::default();
    /// prom_ops.push(Gauge::from("gauge_name").dec());
    /// ```
    #[inline]
    #[must_use]
    pub fn dec(&mut self) -> PrometheusOperation {
        let op = GaugeOp {
            value: 1.0,
            operation: gauge_op::Operation::Dec.into(),
        };
        PrometheusOperation {
            name: self.name.to_owned(),
            labels: self.labels.to_owned(),
            operation: Some(prometheus_operation::Operation::Gauge(op)),
        }
    }

    /// Adds an arbitrary value to a Gauge. (The value can be negative, resulting in a decrease of the Gauge.)
    ///
    /// ### Example
    /// ```
    /// use substreams_sink_prometheus::{PrometheusOperations, Gauge};
    /// let mut prom_ops: PrometheusOperations = Default::default();
    /// prom_ops.push(Gauge::from("gauge_name").add(123.456));
    /// ```
    #[inline]
    #[must_use]
    pub fn add(&mut self, value: f64) -> PrometheusOperation {
        let op = GaugeOp {
            value,
            operation: gauge_op::Operation::Add.into(),
        };
        PrometheusOperation {
            name: self.name.to_owned(),
            labels: self.labels.to_owned(),
            operation: Some(prometheus_operation::Operation::Gauge(op)),
        }
    }

    /// Subtracts arbitrary value from the Gauge. (The value can be negative, resulting in an increase of the Gauge.)
    ///
    /// ### Example
    /// ```
    /// use substreams_sink_prometheus::{PrometheusOperations, Gauge};
    /// let mut prom_ops: PrometheusOperations = Default::default();
    /// prom_ops.push(Gauge::from("gauge_name").sub(123.456));
    /// ```
    #[inline]
    #[must_use]
    pub fn sub(&mut self, value: f64) -> PrometheusOperation {
        let op = GaugeOp {
            value,
            operation: gauge_op::Operation::Sub.into(),
        };
        PrometheusOperation {
            name: self.name.to_owned(),
            labels: self.labels.to_owned(),
            operation: Some(prometheus_operation::Operation::Gauge(op)),
        }
    }

    /// SetToCurrentTime sets the Gauge to the current Unix time in seconds.
    ///
    /// ### Example
    /// ```
    /// use substreams_sink_prometheus::{PrometheusOperations, Gauge};
    /// let mut prom_ops: PrometheusOperations = Default::default();
    /// prom_ops.push(Gauge::from("gauge_name").set_to_current_time());
    /// ```
    #[inline]
    #[must_use]
    pub fn set_to_current_time(&mut self) -> PrometheusOperation {
        let op = GaugeOp {
            value: f64::NAN,
            operation: gauge_op::Operation::SetToCurrentTime.into(),
        };
        PrometheusOperation {
            name: self.name.to_owned(),
            labels: self.labels.to_owned(),
            operation: Some(prometheus_operation::Operation::Gauge(op)),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::PrometheusOperations;

    #[test]
    fn test_gauge() {
        let mut prom_ops: PrometheusOperations = Default::default();
        prom_ops.push(Gauge::from("a").set(88.8));
        prom_ops.push(Gauge::from("b").inc());
        prom_ops.push(Gauge::from("c").dec());
        prom_ops.push(Gauge::from("d").add(123.456));
        prom_ops.push(Gauge::from("e").sub(123.456));

        assert_eq!(prom_ops.operations.len(), 5);
    }
}
