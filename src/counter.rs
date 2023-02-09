use std::collections::HashMap;

use crate::{Metrics, Operations, PrometheusOperation};

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
    ///
    /// ### Example
    /// ```
    /// use std::collections::HashMap;
    /// use substreams_sink_prometheus::Counter;
    /// let mut counter = Counter::from("counter_name");
    /// let mut labels = HashMap::new();
    /// labels.insert("label1".to_string(), "value1".to_string());
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
        PrometheusOperation {
            metric: Metrics::Counter.into(),
            operation: Operations::Inc.into(),
            name: self.name.to_owned(),
            value: 1.0,
            labels: self.labels.to_owned(),
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
        PrometheusOperation {
            metric: Metrics::Counter.into(),
            operation: Operations::Add.into(),
            name: self.name.to_owned(),
            value,
            labels: self.labels.to_owned(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::PrometheusOperations;

    #[test]
    fn test_counter() {
        let mut prom_ops: PrometheusOperations = Default::default();
        prom_ops.push(Counter::from("a").inc());
        prom_ops.push(Counter::from("b").add(123.456));
        assert_eq!(prom_ops.operations.len(), 2);
    }
}
