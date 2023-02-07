use crate::{Metrics, Metric};

impl Metrics {
    /// Increments the Counter by 1.
    ///
    /// ### Example
    /// ```
    /// use substreams_sink_prometheus::PrometheusOperations;
    /// let mut prom_ops: PrometheusOperations = Default::default();
    ///
    /// // INC Counter by 1
    /// prom_ops.push_counter_inc("custom_counter", vec!["custom_label"]);
    /// ```
    pub fn push_counter_inc(&mut self, name: &str, labels: Vec<&str>) {
        self.metrics.push(Metric {
            r#type: "COUNTER".to_owned(), 
            operation: "INC".to_owned(),
            name: name.to_owned(),
            value: 1.0,
            labels: vec_to_string(labels),
        });
    }
    /// Adds an arbitrary value to a Counter. (Returns an error if the value is < 0.)
    ///
    /// ### Example
    /// ```
    /// use substreams_sink_prometheus::PrometheusOperations;
    /// let mut prom_ops: PrometheusOperations = Default::default();
    ///
    /// // ADD Counter by arbitrary value
    /// prom_ops.push_counter_add("counter_name", 123.456, vec!["custom_label"]);
    /// ```
    pub fn push_counter_add(&mut self, name: &str, value: f64, labels: Vec<&str>) {
        self.metrics.push(Metric {
            r#type: "COUNTER".to_owned(),
            operation: "ADD".to_owned(),
            name: name.to_owned(),
            value: value,
            labels: vec_to_string(labels),
        });
    }

    /// SetToCurrentTime sets the Gauge to the current Unix time in seconds.
    ///
    /// ### Example
    /// ```
    /// use substreams_sink_prometheus::PrometheusOperations;
    /// let mut prom_ops: PrometheusOperations = Default::default();
    ///
    /// // Set Gauge to the current Unix time in seconds.
    /// prom_ops.push_set_to_current_time("gauge_name", vec!["custom_label"]);
    /// ```
    pub fn push_counter_set_to_current_time(&mut self, name: &str, labels: Vec<&str>) {
        self.metrics.push(Metric {
            r#type: "COUNTER".to_owned(),
            operation: "SET_TO_CURRENT_TIME".to_owned(),
            name: name.to_owned(),
            value: f64::NAN,
            labels: vec_to_string(labels),
        });
    }
}

fn vec_to_string(v: Vec<&str>) -> Vec<String> {
    v.iter().map(|s| s.to_string()).collect()
}
