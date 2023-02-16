use std::collections::HashMap;

use crate::{PrometheusOperation, prometheus_operation, HistogramOp, histogram_op};

#[derive(Eq, Debug, PartialEq, Default)]
pub struct Histogram {
    pub name: String,
    pub labels: HashMap<String, String>,
}

impl Histogram {
    /// Create new Histogram
    ///
    /// ### Example
    /// ```
    /// use substreams_sink_prometheus::Histogram;
    /// let mut histogram = Histogram::from("historam_name");
    /// ```
    #[inline]
    #[must_use]
    pub fn from(name: &str) -> Self {
        Self {
            name: name.to_string(),
            labels: Default::default(),
        }
    }

    /// Set label to Histogram
    /// Labels represents a collection of label name -> value mappings. 
    ///
    /// ### Example
    /// ```
    /// use std::collections::HashMap;
    /// use substreams_sink_prometheus::Histogram;
    /// let mut histogram = Histogram::from("historam_name");
    /// let labels = HashMap::from([("label1".to_string(), "value1".to_string())]);
    /// histogram.with(labels);
    /// ```
    #[inline]
    pub fn with(mut self, labels: HashMap<String, String>) -> Self {
        self.labels = labels;
        self
    }

    /// Observe adds a single observation to the histogram.
    /// Observations are usually positive or zero.
    /// Negative observations are accepted but prevent current versions of Prometheus from properly detecting counter resets in the sum of observations. 
    ///
    /// ### Example
    /// ```
    /// use substreams_sink_prometheus::{PrometheusOperations, Histogram};
    /// let mut prom_ops: PrometheusOperations = Default::default();
    /// prom_ops.push(Histogram::from("historam_name").observe(88.8));
    /// ```
    #[inline]
    #[must_use]
    pub fn observe(&mut self, value: f64) -> PrometheusOperation {
        let op = HistogramOp {
            value,
            operation: histogram_op::Operation::Observe.into(),
        };
        PrometheusOperation {
            name: self.name.to_owned(),
            labels: self.labels.to_owned(),
            operation: Some(prometheus_operation::Operation::Histogram(op)),
        }
    }

    /// Start a timer. Calling the returned function will observe the duration in seconds in the histogram.
    ///
    /// ### Example
    /// ```
    /// use substreams_sink_prometheus::{PrometheusOperations, Histogram};
    /// let mut prom_ops: PrometheusOperations = Default::default();
    /// prom_ops.push(Histogram::from("historam_name").start_timer());
    /// ```
    #[inline]
    #[must_use]
    pub fn start_timer(&mut self) -> PrometheusOperation {
        let op = HistogramOp {
            value: f64::NAN,
            operation: histogram_op::Operation::StartTimer.into(),
        };
        PrometheusOperation {
            name: self.name.to_owned(),
            labels: self.labels.to_owned(),
            operation: Some(prometheus_operation::Operation::Histogram(op)),
        }
    }

    /// Initialize the metrics for the given combination of labels to zero
    ///
    /// ### Example
    /// ```
    /// use std::collections::HashMap;
    /// use substreams_sink_prometheus::{PrometheusOperations, Histogram};
    /// let mut prom_ops: PrometheusOperations = Default::default();
    /// let labels = HashMap::from([("label1".to_string(), "value1".to_string())]);
    /// prom_ops.push(Histogram::from("historam_name").zero(labels));
    /// ```
    #[inline]
    #[must_use]
    pub fn zero(&mut self, labels: HashMap<String, String>) -> PrometheusOperation {
        let op = HistogramOp {
            value: f64::NAN,
            operation: histogram_op::Operation::Zero.into(),
        };
        PrometheusOperation {
            name: self.name.to_owned(),
            labels,
            operation: Some(prometheus_operation::Operation::Histogram(op)),
        }
    }

    /// Remove metrics for the given label values
    ///
    /// ### Example
    /// ```
    /// use std::collections::HashMap;
    /// use substreams_sink_prometheus::{PrometheusOperations, Histogram};
    /// let mut prom_ops: PrometheusOperations = Default::default();
    /// let labels = HashMap::from([("label1".to_string(), "value1".to_string())]);
    /// prom_ops.push(Histogram::from("historam_name").remove(labels));
    /// ```
    #[inline]
    #[must_use]
    pub fn remove(&mut self, labels: HashMap<String, String>) -> PrometheusOperation {
        let op = HistogramOp {
            value: f64::NAN,
            operation: histogram_op::Operation::Remove.into(),
        };
        PrometheusOperation {
            name: self.name.to_owned(),
            labels,
            operation: Some(prometheus_operation::Operation::Histogram(op)),
        }
    }

    /// Reset histogram values
    ///
    /// ### Example
    /// ```
    /// use substreams_sink_prometheus::{PrometheusOperations, Histogram};
    /// let mut prom_ops: PrometheusOperations = Default::default();
    /// prom_ops.push(Histogram::from("historam_name").reset());
    /// ```
    #[inline]
    #[must_use]
    pub fn reset(&mut self) -> PrometheusOperation {
        let op = HistogramOp {
            value: f64::NAN,
            operation: histogram_op::Operation::Reset.into(),
        };
        PrometheusOperation {
            name: self.name.to_owned(),
            labels: Default::default(),
            operation: Some(prometheus_operation::Operation::Histogram(op)),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::PrometheusOperations;
    use std::collections::HashMap;

    #[test]
    fn test_gauge() {
        let mut prom_ops: PrometheusOperations = Default::default();
        let labels = HashMap::from([("label1".to_string(), "value1".to_string())]);
        let mut histogram = Histogram::from("historam_name").with(labels.clone());

        prom_ops.push(histogram.observe(88.8));
        prom_ops.push(histogram.start_timer());
        prom_ops.push(histogram.zero(labels.clone()));
        prom_ops.push(histogram.reset());
        prom_ops.push(histogram.remove(labels));

        assert_eq!(prom_ops.operations.len(), 5);
    }
}
