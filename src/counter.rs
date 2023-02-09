use crate::{Metrics, Operations, PrometheusOperation};

#[derive(Eq, Clone, Debug, PartialEq, PartialOrd, Ord, Default)]
pub struct Counter {
    pub name: String,
    pub labels: Vec<String>,
}

impl Counter {
    /// Create new Counter
    ///
    /// ### Example
    /// ```
    /// use substreams_sink_prometheus::Counter;
    /// let mut counter = Counter::new("counter_name");
    /// ```
    #[inline]
    #[must_use]
    pub fn new(name: &str) -> Self {
        Self {
            name: name.to_string(),
            labels: vec![],
        }
    }

    /// Set label to Counter
    ///
    /// ### Example
    /// ```
    /// use substreams_sink_prometheus::Counter;
    /// let mut counter = Counter::new("counter_name");
    /// counter.set_label("custom_label");
    /// ```
    #[inline]
    pub fn set_label(&mut self, label: &str) {
        self.labels = vec![label.to_owned()];
    }

    /// Set labels to Counter
    ///
    /// ### Example
    /// ```
    /// use substreams_sink_prometheus::Counter;
    /// let mut counter = Counter::new("counter_name");
    /// counter.set_labels(vec!["custom_label_1", "custom_label_2"]);
    /// ```
    #[inline]
    pub fn set_labels(&mut self, labels: Vec<&str>) {
        self.labels = labels.iter().map(|s| s.to_string()).collect();
    }

    /// Increments the Counter by 1.
    ///
    /// ### Example
    /// ```
    /// use substreams_sink_prometheus::{PrometheusOperations, Counter};
    /// let mut prom_ops: PrometheusOperations = Default::default();
    /// prom_ops.push(Counter::new("counter_name").inc());
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
    /// prom_ops.push(Counter::new("counter_name").add(123.456));
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

    /// SetToCurrentTime sets the Gauge to the current Unix time in seconds.
    ///
    /// ### Example
    /// ```
    /// use substreams_sink_prometheus::{PrometheusOperations, Counter};
    /// let mut prom_ops: PrometheusOperations = Default::default();
    /// prom_ops.push(Counter::new("counter_name").set_to_current_time());
    /// ```
    #[inline]
    #[must_use]
    pub fn set_to_current_time(&mut self) -> PrometheusOperation {
        PrometheusOperation {
            metric: Metrics::Counter.into(),
            operation: Operations::SetToCurrentTime.into(),
            name: self.name.to_owned(),
            value: f64::NAN,
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
        prom_ops.push(Counter::new("a").inc());
        prom_ops.push(Counter::new("b").add(123.456));
        assert_eq!(prom_ops.operations.len(), 2);
    }
}
