use crate::pb::{prometheus_operation::Type, PrometheusOperation, PrometheusOperations};

impl PrometheusOperations {
    pub fn push_new<K: AsRef<str>, V: AsRef<[u8]>>(&mut self, key: K, value: V, ordinal: u64) {
        self.operations.push(PrometheusOperation {
            key: key.as_ref().to_string(),
            value: value.as_ref().to_vec(),
            ordinal: ordinal,
            r#type: Type::Set.into(),
        })
    }
    pub fn push_delete<V: AsRef<str>>(&mut self, key: V, ordinal: u64) {
        self.operations.push(PrometheusOperation {
            key: key.as_ref().to_string(),
            value: vec![],
            ordinal: ordinal,
            r#type: Type::Delete.into(),
        })
    }
}