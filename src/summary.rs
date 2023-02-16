use std::collections::HashMap;

use crate::{PrometheusOperation, prometheus_operation, SummaryOp, summary_op};

#[derive(Eq, Debug, PartialEq, Default)]
pub struct Summary {
    pub name: String,
    pub labels: HashMap<String, String>,
}

impl Summary {
    /// Create new Summary
    ///
    /// ### Example
    /// ```
    /// use substreams_sink_prometheus::Summary;
    /// let mut summary = Summary::from("summary_name");
    /// ```
    #[inline]
    #[must_use]
    pub fn from(name: &str) -> Self {
        Self {
            name: name.to_string(),
            labels: Default::default(),
        }
    }

    /// Set label to Summary
    /// Labels represents a collection of label name -> value mappings. 
    ///
    /// ### Example
    /// ```
    /// use std::collections::HashMap;
    /// use substreams_sink_prometheus::Summary;
    /// let mut summary = Summary::from("summary_name");
    /// let labels = HashMap::from([("label1".to_string(), "value1".to_string())]);
    /// summary.with(labels);
    /// ```
    #[inline]
    pub fn with(mut self, labels: HashMap<String, String>) -> Self {
        self.labels = labels;
        self
    }

    /// Observe adds a single observation to the summary.
    /// Observations are usually positive or zero.
    /// Negative observations are accepted but prevent current versions of Prometheus from properly detecting counter resets in the sum of observations
    ///
    /// ### Example
    /// ```
    /// use substreams_sink_prometheus::{PrometheusOperations, Summary};
    /// let mut prom_ops: PrometheusOperations = Default::default();
    /// prom_ops.push(Summary::from("summary_name").observe(88.8));
    /// ```
    #[inline]
    #[must_use]
    pub fn observe(&mut self, value: f64) -> PrometheusOperation {
        let op = SummaryOp {
            value,
            operation: summary_op::Operation::Observe.into(),
        };
        PrometheusOperation {
            name: self.name.to_owned(),
            labels: self.labels.to_owned(),
            operation: Some(prometheus_operation::Operation::Summary(op)),
        }
    }

    /// Start a timer. Calling the returned function will observe the duration in seconds in the summary.
    ///
    /// ### Example
    /// ```
    /// use substreams_sink_prometheus::{PrometheusOperations, Summary};
    /// let mut prom_ops: PrometheusOperations = Default::default();
    /// prom_ops.push(Summary::from("summary_name").start_timer());
    /// ```
    #[inline]
    #[must_use]
    pub fn start_timer(&mut self) -> PrometheusOperation {
        let op = SummaryOp {
            value: f64::NAN,
            operation: summary_op::Operation::StartTimer.into(),
        };
        PrometheusOperation {
            name: self.name.to_owned(),
            labels: self.labels.to_owned(),
            operation: Some(prometheus_operation::Operation::Summary(op)),
        }
    }

    /// Remove metrics for the given label values
    ///
    /// ### Example
    /// ```
    /// use std::collections::HashMap;
    /// use substreams_sink_prometheus::{PrometheusOperations, Summary};
    /// let mut prom_ops: PrometheusOperations = Default::default();
    /// let labels = HashMap::from([("label1".to_string(), "value1".to_string())]);
    /// prom_ops.push(Summary::from("summary_name").remove(labels));
    /// ```
    #[inline]
    #[must_use]
    pub fn remove(&mut self, labels: HashMap<String, String>) -> PrometheusOperation {
        let op = SummaryOp {
            value: f64::NAN,
            operation: summary_op::Operation::Remove.into(),
        };
        PrometheusOperation {
            name: self.name.to_owned(),
            labels,
            operation: Some(prometheus_operation::Operation::Summary(op)),
        }
    }

    /// Reset all values in the summary
    ///
    /// ### Example
    /// ```
    /// use substreams_sink_prometheus::{PrometheusOperations, Summary};
    /// let mut prom_ops: PrometheusOperations = Default::default();
    /// prom_ops.push(Summary::from("summary_name").reset());
    /// ```
    #[inline]
    #[must_use]
    pub fn reset(&mut self) -> PrometheusOperation {
        let op = SummaryOp {
            value: f64::NAN,
            operation: summary_op::Operation::Reset.into(),
        };
        PrometheusOperation {
            name: self.name.to_owned(),
            labels: Default::default(),
            operation: Some(prometheus_operation::Operation::Summary(op)),
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
        let mut summary = Summary::from("summary_name").with(labels.clone());

        prom_ops.push(summary.observe(88.8));
        prom_ops.push(summary.start_timer());
        prom_ops.push(summary.reset());
        prom_ops.push(summary.remove(labels));

        assert_eq!(prom_ops.operations.len(), 4);
    }
}
