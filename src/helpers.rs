use crate::pb::{prometheus_operation::Type, PrometheusOperation, PrometheusOperations};

impl PrometheusOperations {
    /// Set sets the Gauge to an arbitrary value. 
    /// 
    /// ### Example
    /// ```
    /// use substreams_sink_prometheus::PrometheusOperations;
    /// let mut prom_ops: PrometheusOperations = Default::default();
    /// prom_ops.push_set(vec!["some_key"], 123.456);
    /// ```
    pub fn push_set(&mut self, labels: Vec<&str>, value: f64) {
        self.operations.push(PrometheusOperation {
            labels: vec_to_string(labels),
            value,
            r#type: Type::Set.into(),
        })
    }

    /// Unsets the Gauge.
    ///
    /// This is a no-op if the Gauge is already unset.
    /// This is not the same as setting the Gauge to 0.
    /// Unset is used to remove the Gauge from the registry.
    /// If you want to set the Gauge to 0, use Set.
    /// If you want to remove the Gauge from the registry, use Delete.
    /// 
    /// ### Example
    ///
    /// ```
    /// use substreams_sink_prometheus::PrometheusOperations;
    /// let mut prom_ops: PrometheusOperations = Default::default();
    /// prom_ops.push_delete(vec!["some_key"]);
    /// ```
    pub fn push_delete(&mut self, labels: Vec<&str>) {
        self.operations.push(PrometheusOperation {
            labels: vec_to_string(labels),
            value: f64::default(),
            r#type: Type::Delete.into(),
        })
    }

    /// Inc increments the Gauge by 1. Use Add to increment it by arbitrary values.
    /// 
    /// ### Example
    /// ```
    /// use substreams_sink_prometheus::PrometheusOperations;
    /// let mut prom_ops: PrometheusOperations = Default::default();
    /// prom_ops.push_inc(vec!["some_key"]);
    /// ```
    pub fn push_inc(&mut self, labels: Vec<&str>) {
        self.operations.push(PrometheusOperation {
            labels: vec_to_string(labels),
            value: f64::default(),
            r#type: Type::Inc.into(),
        })
    }

    /// Dec decrements the Gauge by 1. Use Sub to decrement it by arbitrary values.
    /// 
    /// ### Example
    /// ```
    /// use substreams_sink_prometheus::PrometheusOperations;
    /// let mut prom_ops: PrometheusOperations = Default::default();
    /// prom_ops.push_dec(vec!["some_key"]);
    /// ```
    pub fn push_dec(&mut self, labels: Vec<&str>) {
        self.operations.push(PrometheusOperation {
            labels: vec_to_string(labels),
            value: f64::default(),
            r#type: Type::Dec.into(),
        })
    }

    /// Add adds the given value to the Gauge. (The value can be negative, resulting in a decrease of the Gauge.)
    /// 
    /// ### Example
    /// ```
    /// use substreams_sink_prometheus::PrometheusOperations;
    /// let mut prom_ops: PrometheusOperations = Default::default();
    /// prom_ops.push_add(vec!["some_key"], 123.456);
    /// ```
    pub fn push_add(&mut self, labels: Vec<&str>, value: f64) {
        self.operations.push(PrometheusOperation {
            labels: vec_to_string(labels),
            value,
            r#type: Type::Add.into(),
        })
    }

    /// Sub subtracts the given value from the Gauge. (The value can be negative, resulting in an increase of the Gauge.)
    /// 
    /// ### Example
    /// ```
    /// use substreams_sink_prometheus::PrometheusOperations;
    /// let mut prom_ops: PrometheusOperations = Default::default();
    /// prom_ops.push_sub(vec!["some_key"], 123.456);
    /// ```
    pub fn push_sub(&mut self, labels: Vec<&str>, value: f64) {
        self.operations.push(PrometheusOperation {
            labels: vec_to_string(labels),
            value,
            r#type: Type::Sub.into(),
        })
    }

    /// SetToCurrentTime sets the Gauge to the current Unix time in seconds.
    /// 
    /// ### Example
    /// ```
    /// use substreams_sink_prometheus::PrometheusOperations;
    /// let mut prom_ops: PrometheusOperations = Default::default();
    /// prom_ops.push_set_to_current_time(vec!["some_key"]);
    /// ```
    pub fn push_set_to_current_time(&mut self, labels: Vec<&str>) {
        self.operations.push(PrometheusOperation {
            labels: vec_to_string(labels),
            value: f64::default(),
            r#type: Type::Dec.into(),
        })
    }
}

fn vec_to_string(vec: Vec<&str>) -> Vec<String> {
    vec.iter().map(|s| s.to_string()).collect()
}