use crate::{PrometheusOperation, Operations, Metrics};

#[derive(Debug, Clone, PartialEq)]
pub struct Counter {
    pub name: String,
    pub labels: Vec<String>
}

impl Counter {
    /// Increments the Counter by 1.
    ///
    /// ### Example
    /// ```
    /// use substreams_sink_prometheus::{PrometheusOperations, Counter};
    /// let mut prom_ops: PrometheusOperations = Default::default();
    ///
    /// // INC Counter by 1
    /// prom_ops.operations.push(Counter{
    ///     name: "custom_counter".to_owned(),
    ///     labels: vec!["custom_label".to_owned()]
    /// }.inc());
    /// ```
    pub fn inc(&mut self) -> PrometheusOperation {
        PrometheusOperation {
            metric: Metrics::Counter.into(), 
            operation: Operations::Inc.into(),
            name: self.name.to_owned(),
            value: 1.0,
            labels: self.labels.to_owned(),
        }.clone()
    }

    /// Adds an arbitrary value to a Counter. (Returns an error if the value is < 0.)
    ///
    /// ### Example
    /// ```
    /// use substreams_sink_prometheus::{PrometheusOperations, Counter};
    /// let mut prom_ops: PrometheusOperations = Default::default();
    ///
    /// // ADD Counter by arbitrary value
    /// prom_ops.operations.push(Counter{
    ///     name: "custom_counter".to_owned(),
    ///     labels: vec!["custom_label".to_owned()]
    /// }.add(123.456));
    /// ```
    pub fn add(&mut self, value: f64) -> PrometheusOperation {
        PrometheusOperation {
            metric: Metrics::Counter.into(),
            operation: Operations::Add.into(),
            name: self.name.to_owned(),
            value: value,
            labels: self.labels.to_owned(),
        }.clone()
    }

    /// SetToCurrentTime sets the Gauge to the current Unix time in seconds.
    ///
    /// ### Example
    /// ```
    /// use substreams_sink_prometheus::{PrometheusOperations, Counter};
    /// let mut prom_ops: PrometheusOperations = Default::default();
    ///
    /// // Set Gauge to the current Unix time in seconds.
    /// prom_ops.operations.push(Counter{
    ///     name: "custom_counter".to_owned(),
    ///     labels: vec!["custom_label".to_owned()]
    /// }.set_to_current_time());
    /// ```
    pub fn set_to_current_time(&mut self) -> PrometheusOperation {
        PrometheusOperation {
            metric: Metrics::Counter.into(),
            operation: Operations::SetToCurrentTime.into(),
            name: self.name.to_owned(),
            value: f64::NAN,
            labels: self.labels.to_owned(),
        }.clone()
    }
}

#[cfg(test)]
mod tests {
    use crate::PrometheusOperations;
    use super::*;

    #[test]
    fn test_counter() {

        let mut prom_ops: PrometheusOperations = Default::default();

        prom_ops.operations.push(Counter{
            name: "custom_counter".to_owned(),
            labels: vec!["custom_label".to_owned()]
        }.inc());

        assert_eq!(prom_ops.operations.len(), 1);
    }

    #[test]
    fn test_counter_push() {
        let mut prom_ops: PrometheusOperations = Default::default();
        let mut counter = Counter{
            name: "custom_counter".to_owned(),
            labels: vec!["custom_label".to_owned()]
        };
        prom_ops.operations.push(counter.inc());
        prom_ops.operations.push(counter.add(123.456));

        assert_eq!(prom_ops.operations.len(), 2);
    }

}