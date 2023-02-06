use crate::pb::{PrometheusOperation, PrometheusOperations, metric, operation};

impl PrometheusOperations {
    /// Increments the Counter by 1.
    ///
    /// ### Example
    /// ```
    /// use substreams_sink_prometheus::PrometheusOperations;
    /// let mut prom_ops: PrometheusOperations = Default::default();
    ///
    /// // INC Counter by 1
    /// prom_ops.push_counter_inc("counter_name", vec![], None);
    /// 
    /// // INC Counter using labels & help documentation
    /// prom_ops.push_counter_inc("custom_counter", vec!["custom_label"], Some("Counter documentation"));
    /// ```
    pub fn push_counter_inc(&mut self, name: &str, labels: Vec<&str>, help: Option<&str>) {
        self.operations.push(PrometheusOperation {
            name: name.to_owned(),
            metric: metric::Type::Counter.into(),
            r#type: operation::Type::Inc.into(),
            value: 1.0,
            labels: vec_to_string(labels),
            help: help.map(str::to_string)
        })
    }
    /// Adds an arbitrary value to a Counter. (Returns an error if the value is < 0.)
    ///
    /// ### Example
    /// ```
    /// use substreams_sink_prometheus::PrometheusOperations;
    /// let mut prom_ops: PrometheusOperations = Default::default();
    ///
    /// // ADD Counter by arbitrary value
    /// prom_ops.push_counter_add("counter_name", 123.456, vec![], None);
    /// 
    /// // ADD Counter using labels & help documentation
    /// prom_ops.push_counter_add("custom_counter", 50.0, vec!["custom_label"], Some("counter documentation"));
    /// ```
    pub fn push_counter_add(&mut self, name: &str, value: f64, labels: Vec<&str>, help: Option<&str>) {
        self.operations.push(PrometheusOperation {
            name: name.to_owned(),
            metric: metric::Type::Gauge.into(),
            r#type: operation::Type::Dec.into(),
            value,
            labels: vec_to_string(labels),
            help: help.map(str::to_string)
        })
    }

}

fn vec_to_string(v: Vec<&str>) -> Vec<String> {
    v.iter().map(|s| s.to_string()).collect()
}
