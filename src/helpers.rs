use crate::pb::{
    prometheus_operation::Metric, prometheus_operation::Type, PrometheusOperation,
    PrometheusOperations,
};

impl PrometheusOperations {
    /// Set sets the Gauge to an arbitrary value.
    ///
    /// ### Example
    /// ```
    /// use substreams_sink_prometheus::PrometheusOperations;
    /// let mut prom_ops: PrometheusOperations = Default::default();
    /// prom_ops.push_set("gauge_name".to_owned(), Some("Gauge Help".to_owned()), vec!["some_key"], 123.456);
    /// ```
    pub fn push_set(&mut self, name: String, help: Option<String>, labels: Vec<&str>, value: f64) {
        self.operations.push(PrometheusOperation {
            name,
            help,
            labels: vec_to_string(labels),
            value,
            metric: Metric::Gauge.into(),
            r#type: Type::Set.into(),
        })
    }

    /// Inc increments the Counter by 1.
    ///
    /// ### Example
    /// ```
    /// use substreams_sink_prometheus::PrometheusOperations;
    /// let mut prom_ops: PrometheusOperations = Default::default();
    /// prom_ops.push_counter_inc("counter_name".to_owned(), Some("Counter Help".to_owned()), vec!["some_key"]);
    /// ```
    pub fn push_counter_inc(&mut self, name: String, help: Option<String>, labels: Vec<&str>) {
        self.operations.push(PrometheusOperation {
            name,
            help,
            labels: vec_to_string(labels),
            value: f64::default(),
            metric: Metric::Counter.into(),
            r#type: Type::Inc.into(),
        })
    }

    /// Inc increments the Gauge by 1. Use Add to increment it by arbitrary values.
    ///
    /// ### Example
    /// ```
    /// use substreams_sink_prometheus::PrometheusOperations;
    /// let mut prom_ops: PrometheusOperations = Default::default();
    /// prom_ops.push_gauge_inc("gauge_name".to_owned(), Some("Gauge Help".to_owned()), vec!["some_key"]);
    /// ```
    pub fn push_gauge_inc(&mut self, name: String, help: Option<String>, labels: Vec<&str>) {
        self.operations.push(PrometheusOperation {
            name,
            help,
            labels: vec_to_string(labels),
            value: f64::default(),
            metric: Metric::Gauge.into(),
            r#type: Type::Inc.into(),
        })
    }

    /// Dec decrements the Gauge by 1. Use Sub to decrement it by arbitrary values.
    ///
    /// ### Example
    /// ```
    /// use substreams_sink_prometheus::PrometheusOperations;
    /// let mut prom_ops: PrometheusOperations = Default::default();
    /// prom_ops.push_dec("gauge_name".to_owned(), Some("Gauge Help".to_owned()), vec!["some_key"]);
    /// ```
    pub fn push_dec(&mut self, name: String, help: Option<String>, labels: Vec<&str>) {
        self.operations.push(PrometheusOperation {
            name,
            help,
            labels: vec_to_string(labels),
            value: f64::default(),
            metric: Metric::Gauge.into(),
            r#type: Type::Dec.into(),
        })
    }

    /// Add adds the given value to the Counter. (Returns an error if the value is < 0.)
    ///
    /// ### Example
    /// ```
    /// use substreams_sink_prometheus::PrometheusOperations;
    /// let mut prom_ops: PrometheusOperations = Default::default();
    /// prom_ops.push_counter_add("counter_name".to_owned(), Some("Counter Help".to_owned()), vec!["some_key"], 123.456);
    /// ```
    pub fn push_counter_add(
        &mut self,
        name: String,
        help: Option<String>,
        labels: Vec<&str>,
        value: f64,
    ) {
        self.operations.push(PrometheusOperation {
            name,
            help,
            labels: vec_to_string(labels),
            value,
            metric: Metric::Counter.into(),
            r#type: Type::Add.into(),
        })
    }

    /// Add adds the given value to the Gauge. (The value can be negative, resulting in a decrease of the Gauge.)
    ///
    /// ### Example
    /// ```
    /// use substreams_sink_prometheus::PrometheusOperations;
    /// let mut prom_ops: PrometheusOperations = Default::default();
    /// prom_ops.push_gauge_add("gauge_name".to_owned(), Some("Gauge Help".to_owned()), vec!["some_key"], 123.456);
    /// ```
    pub fn push_gauge_add(
        &mut self,
        name: String,
        help: Option<String>,
        labels: Vec<&str>,
        value: f64,
    ) {
        self.operations.push(PrometheusOperation {
            name,
            help,
            labels: vec_to_string(labels),
            value,
            metric: Metric::Gauge.into(),
            r#type: Type::Add.into(),
        })
    }

    /// Sub subtracts the given value from the Gauge. (The value can be negative, resulting in an increase of the Gauge.)
    ///
    /// ### Example
    /// ```
    /// use substreams_sink_prometheus::PrometheusOperations;
    /// let mut prom_ops: PrometheusOperations = Default::default();
    /// prom_ops.push_sub("gauge_name".to_owned(), Some("Gauge Help".to_owned()), vec!["some_key"], 123.456);
    /// ```
    pub fn push_sub(&mut self, name: String, help: Option<String>, labels: Vec<&str>, value: f64) {
        self.operations.push(PrometheusOperation {
            name,
            help,
            labels: vec_to_string(labels),
            value,
            metric: Metric::Gauge.into(),
            r#type: Type::Sub.into(),
        })
    }

    /// SetToCurrentTime sets the Gauge to the current Unix time in seconds.
    ///
    /// ### Example
    /// ```
    /// use substreams_sink_prometheus::PrometheusOperations;
    /// let mut prom_ops: PrometheusOperations = Default::default();
    /// prom_ops.push_set_to_current_time("gauge_name".to_owned(), Some("Gauge Help".to_owned()), vec!["some_key"]);
    /// ```
    pub fn push_set_to_current_time(
        &mut self,
        name: String,
        help: Option<String>,
        labels: Vec<&str>,
    ) {
        self.operations.push(PrometheusOperation {
            name,
            help,
            labels: vec_to_string(labels),
            value: f64::default(),
            metric: Metric::Gauge.into(),
            r#type: Type::Dec.into(),
        })
    }
}

fn vec_to_string(vec: Vec<&str>) -> Vec<String> {
    vec.iter().map(|s| s.to_string()).collect()
}
