use crate::pb::{Metrics, Metric};

impl Metrics {
    /// Sets the Gauge to an arbitrary value.
    ///
    /// ### Example
    /// ```
    /// use substreams_sink_prometheus::PrometheusOperations;
    /// let mut prom_ops: PrometheusOperations = Default::default();
    /// 
    /// // SET Gauge to an arbitrary value
    /// prom_ops.push_set("gauge_name", 123.456, vec![]);
    /// prom_ops.push_set("gauge_custom", 888.8, vec!["custom_label"]);
    /// ```
    pub fn push_set(&mut self, name: &str, value: f64, labels: Vec<&str>) {
        self.metrics.push(Metric {
            r#type: "GAUGE".to_owned(),
            operation: "SET".to_owned(),
            name: name.to_owned(),
            value,
            labels: vec_to_string(labels),
        });
    }

    /// Increments the Gauge by 1.
    ///
    /// ### Example
    /// ```
    /// use substreams_sink_prometheus::PrometheusOperations;
    /// let mut prom_ops: PrometheusOperations = Default::default();
    ///
    /// // INC Gauge by 1
    /// prom_ops.push_inc("gauge_name", vec![]);
    /// ```
    pub fn push_inc(&mut self, name: &str, labels: Vec<&str>) {
        self.metrics.push(Metric {
            r#type: "GAUGE".to_owned(),
            operation: "INC".to_owned(),
            name: name.to_owned(),
            value: 1.0,
            labels: vec_to_string(labels),
        });
    }

    /// Decrements the Gauge by 1.
    ///
    /// ### Example
    /// ```
    /// use substreams_sink_prometheus::PrometheusOperations;
    /// let mut prom_ops: PrometheusOperations = Default::default();
    ///
    /// // DEC Gauge by 1.
    /// prom_ops.push_dec("gauge_name", vec![]);
    /// ```
    pub fn push_dec(&mut self, name: &str, labels: Vec<&str>) {
        self.metrics.push(Metric {
            r#type: "GAUGE".to_owned(),
            operation: "SET".to_owned(),
            name: name.to_owned(),
            value: 1.0,
            labels: vec_to_string(labels),
        });
    }
    /// Adds an arbitrary value to a Gauge. (The value can be negative, resulting in a decrease of the Gauge.)
    ///
    /// ### Example
    /// ```
    /// use substreams_sink_prometheus::PrometheusOperations;
    /// let mut prom_ops: PrometheusOperations = Default::default();
    ///
    /// // ADD Gauge by arbitrary value
    /// prom_ops.push_add("gauge_name", 50.0, vec![]);
    /// prom_ops.push_add("gauge_name", -10.0, vec![]);
    /// ```
    pub fn push_add(&mut self, name: &str, value: f64, labels: Vec<&str>) {
        self.metrics.push(Metric {
            r#type: "GAUGE".to_owned(),
            operation: "ADD".to_owned(),
            name: name.to_owned(),
            value,
            labels: vec_to_string(labels),
        });
    }

    /// Subtracts arbitrary value from the Gauge. (The value can be negative, resulting in an increase of the Gauge.)
    ///
    /// ### Example
    /// ```
    /// use substreams_sink_prometheus::PrometheusOperations;
    /// let mut prom_ops: PrometheusOperations = Default::default();
    ///
    /// // SUB Gauge by arbitrary value
    /// prom_ops.push_sub("gauge_name", 25.0, vec![]);
    /// prom_ops.push_sub("gauge_name", -5.0, vec![]);
    /// ```
    pub fn push_sub(&mut self, name: &str, value: f64, labels: Vec<&str>) {
        self.metrics.push(Metric {
            r#type: "GAUGE".to_owned(),
            operation: "SUB".to_owned(),
            name: name.to_owned(),
            value,
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
    /// prom_ops.push_set_to_current_time("gauge_name", vec![]);
    /// ```
    pub fn push_set_to_current_time(&mut self, name: &str, labels: Vec<&str>) {
        self.metrics.push(Metric {
            r#type: "GAUGE".to_owned(),
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
