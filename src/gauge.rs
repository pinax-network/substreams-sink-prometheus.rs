use crate::pb::{PrometheusOperation, PrometheusOperations, metric, operation};

impl PrometheusOperations {
    /// Sets the Gauge to an arbitrary value.
    ///
    /// ### Example
    /// ```
    /// use substreams_sink_prometheus::PrometheusOperations;
    /// let mut prom_ops: PrometheusOperations = Default::default();
    /// 
    /// // SET Gauge to an arbitrary value
    /// prom_ops.push_set("gauge_name", 123.456, vec![], None);
    /// 
    /// // SET Gauge using labels & help documentation
    /// prom_ops.push_set("custom_gauge", 50.0, vec!["custom_label"], Some("Gauge documentation"));
    /// ```
    pub fn push_set(&mut self, name: &str, value: f64, labels: Vec<&str>, help: Option<&str>) {
        self.operations.push(PrometheusOperation {
            name: name.to_owned(),
            metric: metric::Type::Gauge.into(),
            r#type: operation::Type::Set.into(),
            value,
            labels: vec_to_string(labels),
            help: help.map(str::to_string)
        })
    }

    /// Increments the Gauge by 1.
    ///
    /// ### Example
    /// ```
    /// use substreams_sink_prometheus::PrometheusOperations;
    /// let mut prom_ops: PrometheusOperations = Default::default();
    ///
    /// // INC Gauge by 1
    /// prom_ops.push_inc("gauge_name", vec![], None);
    /// 
    /// // INC Gauge using labels & help documentation
    /// prom_ops.push_inc("custom_gauge", vec!["custom_label"], Some("Gauge documentation"));
    /// ```
    pub fn push_inc(&mut self, name: &str, labels: Vec<&str>, help: Option<&str>) {
        self.operations.push(PrometheusOperation {
            name: name.to_owned(),
            metric: metric::Type::Gauge.into(),
            r#type: operation::Type::Inc.into(),
            value: 1.0,
            labels: vec_to_string(labels),
            help: help.map(str::to_string)
        })
    }

    /// Decrements the Gauge by 1.
    ///
    /// ### Example
    /// ```
    /// use substreams_sink_prometheus::PrometheusOperations;
    /// let mut prom_ops: PrometheusOperations = Default::default();
    ///
    /// // DEC Gauge by 1.
    /// prom_ops.push_dec("gauge_name", vec![], None);
    /// 
    /// // DEC Gauge using labels & help documentation
    /// prom_ops.push_dec("custom_gauge", vec!["custom_label"], Some("Gauge documentation"));
    /// ```
    pub fn push_dec(&mut self, name: &str, labels: Vec<&str>, help: Option<&str>) {
        self.operations.push(PrometheusOperation {
            name: name.to_owned(),
            metric: metric::Type::Gauge.into(),
            r#type: operation::Type::Dec.into(),
            value: 1.0,
            labels: vec_to_string(labels),
            help: help.map(str::to_string)
        })
    }
    /// Adds an arbitrary value to a Gauge. (The value can be negative, resulting in a decrease of the Gauge.)
    ///
    /// ### Example
    /// ```
    /// use substreams_sink_prometheus::PrometheusOperations;
    /// let mut prom_ops: PrometheusOperations = Default::default();
    ///
    /// // ADD Gauge by arbitrary value
    /// prom_ops.push_add("gauge_name", 123.456, vec![], None);
    /// 
    /// // ADD Gauge using labels & help documentation
    /// prom_ops.push_add("custom_gauge", 50.0, vec!["custom_label"], Some("Gauge documentation"));
    /// ```
    pub fn push_add(&mut self, name: &str, value: f64, labels: Vec<&str>, help: Option<&str>) {
        self.operations.push(PrometheusOperation {
            name: name.to_owned(),
            metric: metric::Type::Gauge.into(),
            r#type: operation::Type::Dec.into(),
            value,
            labels: vec_to_string(labels),
            help: help.map(str::to_string)
        })
    }

    /// Subtracts arbitrary value from the Gauge. (The value can be negative, resulting in an increase of the Gauge.)
    ///
    /// ### Example
    /// ```
    /// use substreams_sink_prometheus::PrometheusOperations;
    /// let mut prom_ops: PrometheusOperations = Default::default();
    ///
    /// // SUB Gauge by arbitrary value
    /// prom_ops.push_sub("gauge_name", 123.456, vec![], None);
    /// 
    /// // SUB Gauge using labels & help documentation
    /// prom_ops.push_sub("custom_gauge", 50.0, vec!["custom_label"], Some("Gauge documentation"));
    /// ```
    pub fn push_sub(&mut self, name: &str, value: f64, labels: Vec<&str>, help: Option<&str>) {
        self.operations.push(PrometheusOperation {
            name: name.to_owned(),
            metric: metric::Type::Gauge.into(),
            r#type: operation::Type::Dec.into(),
            value,
            labels: vec_to_string(labels),
            help: help.map(str::to_string)
        })
    }

    /// SetToCurrentTime sets the Gauge to the current Unix time in seconds.
    ///
    /// ### Example
    /// ```
    /// use substreams_sink_prometheus::PrometheusOperations;
    /// let mut prom_ops: PrometheusOperations = Default::default();
    ///
    /// // Set Gauge to the current Unix time in seconds.
    /// prom_ops.push_set_to_current_time("gauge_name", vec![], None);
    /// 
    /// // Set Gauge using labels & help documentation
    /// prom_ops.push_set_to_current_time("custom_gauge", vec!["custom_label"], Some("Gauge documentation"));
    /// ```
    pub fn push_set_to_current_time(&mut self, name: &str, labels: Vec<&str>, help: Option<&str>) {
        self.operations.push(PrometheusOperation {
            name: name.to_owned(),
            metric: metric::Type::Gauge.into(),
            r#type: operation::Type::SetToCurrentTime.into(),
            value: Default::default(),
            labels: vec_to_string(labels),
            help: help.map(str::to_string)
        })
    }
}

fn vec_to_string(v: Vec<&str>) -> Vec<String> {
    v.iter().map(|s| s.to_string()).collect()
}
