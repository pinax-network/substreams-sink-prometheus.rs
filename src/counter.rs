use std::collections::HashMap;

use crate::{PrometheusOperation, prometheus_operation, CounterOp, counter_op};

#[derive(Eq, Debug, PartialEq, Default)]
pub struct Counter {
    pub name: String,
    pub labels: HashMap<String, String>,
}

impl Counter {
    /// Create new Counter
    ///
    /// ### Example
    /// ```
    /// use substreams_sink_prometheus::Counter;
    /// let mut counter = Counter::from("counter_name");
    /// ```
    #[inline]
    #[must_use]
    pub fn from(name: &str) -> Self {
        Self {
            name: name.to_string(),
            labels: Default::default(),
        }
    }

    /// Set label to Counter
    /// Labels represents a collection of label name -> value mappings. 
    ///
    /// ### Example
    /// ```
    /// use std::collections::HashMap;
    /// use substreams_sink_prometheus::Counter;
    /// let mut counter = Counter::from("counter_name");
    /// let labels = HashMap::from([("label1".to_string(), "value1".to_string())]);
    /// counter.with(labels);
    /// ```
    #[inline]
    pub fn with(mut self, labels: HashMap<String, String>) -> Self {
        self.labels = labels;
        self
    }

    /// Increments the Counter by 1.
    ///
    /// ### Example
    /// ```
    /// use substreams_sink_prometheus::{PrometheusOperations, Counter};
    /// let mut prom_ops: PrometheusOperations = Default::default();
    /// prom_ops.push(Counter::from("counter_name").inc());
    /// ```
    #[inline]
    #[must_use]
    pub fn inc(&mut self) -> PrometheusOperation {
        let op = CounterOp {
            value: 1.0,
            operation: counter_op::Operation::Inc.into(),
        };
        PrometheusOperation {
            name: self.name.to_owned(),
            labels: self.labels.to_owned(),
            operation: Some(prometheus_operation::Operation::Counter(op)),
        }
    }

    /// Adds an arbitrary value to a Counter. (Returns an error if the value is < 0.)
    ///
    /// ### Example
    /// ```
    /// use substreams_sink_prometheus::{PrometheusOperations, Counter};
    /// let mut prom_ops: PrometheusOperations = Default::default();
    /// prom_ops.push(Counter::from("counter_name").add(123.456));
    /// ```
    #[inline]
    #[must_use]
    pub fn add(&mut self, value: f64) -> PrometheusOperation {
        let op = CounterOp {
            value,
            operation: counter_op::Operation::Add.into(),
        };
        PrometheusOperation {
            name: self.name.to_owned(),
            labels: self.labels.to_owned(),
            operation: Some(prometheus_operation::Operation::Counter(op)),
        }
    }

    /// Remove metrics for the given label values
    ///
    /// ### Example
    /// ```
    /// use std::collections::HashMap;
    /// use substreams_sink_prometheus::{PrometheusOperations, Counter};
    /// let mut prom_ops: PrometheusOperations = Default::default();
    /// let labels = HashMap::from([("label1".to_string(), "value1".to_string())]);
    /// prom_ops.push(Counter::from("counter_name").remove(labels));
    /// ```
    #[inline]
    #[must_use]
    pub fn remove(&mut self, labels: HashMap<String, String>) -> PrometheusOperation {
        let op = CounterOp {
            value: f64::NAN,
            operation: counter_op::Operation::Remove.into(),
        };
        PrometheusOperation {
            name: self.name.to_owned(),
            labels,
            operation: Some(prometheus_operation::Operation::Counter(op)),
        }
    }

    /// Reset counter values
    ///
    /// ### Example
    /// ```
    /// use substreams_sink_prometheus::{PrometheusOperations, Counter};
    /// let mut prom_ops: PrometheusOperations = Default::default();
    /// prom_ops.push(Counter::from("counter_name").reset());
    /// ```
    #[inline]
    #[must_use]
    pub fn reset(&mut self) -> PrometheusOperation {
        let op = CounterOp {
            value: f64::NAN,
            operation: counter_op::Operation::Reset.into(),
        };
        PrometheusOperation {
            name: self.name.to_owned(),
            labels: Default::default(),
            operation: Some(prometheus_operation::Operation::Counter(op)),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::PrometheusOperations;
    use std::collections::HashMap;

    #[test]
    fn test_counter() {
        let mut prom_ops: PrometheusOperations = Default::default();
        let labels = HashMap::from([("label1".to_string(), "value1".to_string())]);
        let mut counter = Counter::from("a").with(labels.clone());

        prom_ops.push(counter.inc());
        prom_ops.push(counter.add(123.456));
        prom_ops.push(counter.reset());
        prom_ops.push(counter.remove(labels));
        assert_eq!(prom_ops.operations.len(), 4);
    }
}
