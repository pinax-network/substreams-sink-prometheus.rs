#[cfg(test)]
mod tests {
    use crate::{Counter, PrometheusOperations};
    use std::collections::HashMap;

    #[test]
    fn test_labels_counter_1() {
        let mut prom_ops: PrometheusOperations = Default::default();
        let mut labels = HashMap::new();
        labels.insert("label1".to_string(), "value1".to_string());
        let mut counter = Counter::from("custom_counter").with(labels);
        prom_ops.push(counter.inc());
    }

    #[test]
    fn test_labels_counter_2() {
        let mut prom_ops: PrometheusOperations = Default::default();
        let mut labels = HashMap::new();
        labels.insert("label1".to_string(), "value1".to_string());
        let counter = Counter::from("custom_counter");
        prom_ops.push(counter.with(labels).inc());
    }

    #[test]
    fn test_labels_counter_3() {
        let mut prom_ops: PrometheusOperations = Default::default();
        let labels = HashMap::from([("label1".to_string(), "value1".to_string())]);
        let counter = Counter::from("custom_counter");
        prom_ops.push(counter.with(labels).inc());
    }
}
