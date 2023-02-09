use crate::{PrometheusOperations, PrometheusOperation};

impl PrometheusOperations {
    pub fn push(&mut self, operation: PrometheusOperation) { 
        self.operations.push(operation);
    }

    pub fn extend(&mut self, operations: Vec<PrometheusOperation>) { 
        self.operations.extend(operations);
    }
}


#[cfg(test)]
mod tests {
    use crate::{PrometheusOperations, Counter};

    #[test]
    fn test_push() {
        let mut prom_ops: PrometheusOperations = Default::default();
        let mut counter = Counter::new("custom_counter");
        prom_ops.push(counter.inc());
        prom_ops.push(counter.inc());

        assert_eq!(prom_ops.operations.len(), 2);
    }

    #[test]
    fn test_extend() {
        let mut prom_ops: PrometheusOperations = Default::default();
        let mut counter = Counter::new("custom_counter");
        prom_ops.extend(vec![counter.inc(), counter.inc()]);

        assert_eq!(prom_ops.operations.len(), 2);
    }

}