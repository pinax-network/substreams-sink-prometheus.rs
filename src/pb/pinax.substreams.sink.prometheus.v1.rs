// @generated
/// Reference <https://pkg.go.dev/github.com/prometheus/client_golang/prometheus#Gauge>
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PrometheusOperations {
    #[prost(message, repeated, tag="1")]
    pub operations: ::prost::alloc::vec::Vec<PrometheusOperation>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PrometheusOperation {
    #[prost(string, repeated, tag="1")]
    pub labels: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    #[prost(double, tag="2")]
    pub value: f64,
    #[prost(enumeration="prometheus_operation::Type", tag="4")]
    pub r#type: i32,
}
/// Nested message and enum types in `PrometheusOperation`.
pub mod prometheus_operation {
    /// uint64 ordinal = 3;
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum Type {
        /// Protobuf default should not be used, this is used so that the consume can ensure that the value was actually specified
        Unset = 0,
        /// Set sets the Gauge to an arbitrary value. 
        ///
        /// float
        Set = 1,
        /// Unsets the Gauge.
        Delete = 2,
        /// Inc increments the Gauge by 1. Use Add to increment it by arbitrary values.
        Inc = 3,
        /// Dec decrements the Gauge by 1. Use Sub to decrement it by arbitrary values.
        Dec = 4,
        /// Add adds the given value to the Gauge. (The value can be negative, resulting in a decrease of the Gauge.)
        ///
        /// float
        Add = 5,
        /// Sub subtracts the given value from the Gauge. (The value can be negative, resulting in an increase of the Gauge.)
        ///
        /// float
        Sub = 6,
        /// SetToCurrentTime sets the Gauge to the current Unix time in seconds.
        SetToCurrentTime = 7,
    }
}
/// Encoded file descriptor set for the `pinax.substreams.sink.prometheus.v1` package
pub const FILE_DESCRIPTOR_SET: &[u8] = &[
    0x0a, 0xed, 0x0f, 0x0a, 0x10, 0x70, 0x72, 0x6f, 0x6d, 0x65, 0x74, 0x68, 0x65, 0x75, 0x73, 0x2e,
    0x70, 0x72, 0x6f, 0x74, 0x6f, 0x12, 0x23, 0x70, 0x69, 0x6e, 0x61, 0x78, 0x2e, 0x73, 0x75, 0x62,
    0x73, 0x74, 0x72, 0x65, 0x61, 0x6d, 0x73, 0x2e, 0x73, 0x69, 0x6e, 0x6b, 0x2e, 0x70, 0x72, 0x6f,
    0x6d, 0x65, 0x74, 0x68, 0x65, 0x75, 0x73, 0x2e, 0x76, 0x31, 0x22, 0x70, 0x0a, 0x14, 0x50, 0x72,
    0x6f, 0x6d, 0x65, 0x74, 0x68, 0x65, 0x75, 0x73, 0x4f, 0x70, 0x65, 0x72, 0x61, 0x74, 0x69, 0x6f,
    0x6e, 0x73, 0x12, 0x58, 0x0a, 0x0a, 0x6f, 0x70, 0x65, 0x72, 0x61, 0x74, 0x69, 0x6f, 0x6e, 0x73,
    0x18, 0x01, 0x20, 0x03, 0x28, 0x0b, 0x32, 0x38, 0x2e, 0x70, 0x69, 0x6e, 0x61, 0x78, 0x2e, 0x73,
    0x75, 0x62, 0x73, 0x74, 0x72, 0x65, 0x61, 0x6d, 0x73, 0x2e, 0x73, 0x69, 0x6e, 0x6b, 0x2e, 0x70,
    0x72, 0x6f, 0x6d, 0x65, 0x74, 0x68, 0x65, 0x75, 0x73, 0x2e, 0x76, 0x31, 0x2e, 0x50, 0x72, 0x6f,
    0x6d, 0x65, 0x74, 0x68, 0x65, 0x75, 0x73, 0x4f, 0x70, 0x65, 0x72, 0x61, 0x74, 0x69, 0x6f, 0x6e,
    0x52, 0x0a, 0x6f, 0x70, 0x65, 0x72, 0x61, 0x74, 0x69, 0x6f, 0x6e, 0x73, 0x22, 0xfb, 0x01, 0x0a,
    0x13, 0x50, 0x72, 0x6f, 0x6d, 0x65, 0x74, 0x68, 0x65, 0x75, 0x73, 0x4f, 0x70, 0x65, 0x72, 0x61,
    0x74, 0x69, 0x6f, 0x6e, 0x12, 0x16, 0x0a, 0x06, 0x6c, 0x61, 0x62, 0x65, 0x6c, 0x73, 0x18, 0x01,
    0x20, 0x03, 0x28, 0x09, 0x52, 0x06, 0x6c, 0x61, 0x62, 0x65, 0x6c, 0x73, 0x12, 0x14, 0x0a, 0x05,
    0x76, 0x61, 0x6c, 0x75, 0x65, 0x18, 0x02, 0x20, 0x01, 0x28, 0x01, 0x52, 0x05, 0x76, 0x61, 0x6c,
    0x75, 0x65, 0x12, 0x51, 0x0a, 0x04, 0x74, 0x79, 0x70, 0x65, 0x18, 0x04, 0x20, 0x01, 0x28, 0x0e,
    0x32, 0x3d, 0x2e, 0x70, 0x69, 0x6e, 0x61, 0x78, 0x2e, 0x73, 0x75, 0x62, 0x73, 0x74, 0x72, 0x65,
    0x61, 0x6d, 0x73, 0x2e, 0x73, 0x69, 0x6e, 0x6b, 0x2e, 0x70, 0x72, 0x6f, 0x6d, 0x65, 0x74, 0x68,
    0x65, 0x75, 0x73, 0x2e, 0x76, 0x31, 0x2e, 0x50, 0x72, 0x6f, 0x6d, 0x65, 0x74, 0x68, 0x65, 0x75,
    0x73, 0x4f, 0x70, 0x65, 0x72, 0x61, 0x74, 0x69, 0x6f, 0x6e, 0x2e, 0x54, 0x79, 0x70, 0x65, 0x52,
    0x04, 0x74, 0x79, 0x70, 0x65, 0x22, 0x63, 0x0a, 0x04, 0x54, 0x79, 0x70, 0x65, 0x12, 0x09, 0x0a,
    0x05, 0x55, 0x4e, 0x53, 0x45, 0x54, 0x10, 0x00, 0x12, 0x07, 0x0a, 0x03, 0x53, 0x45, 0x54, 0x10,
    0x01, 0x12, 0x0a, 0x0a, 0x06, 0x44, 0x45, 0x4c, 0x45, 0x54, 0x45, 0x10, 0x02, 0x12, 0x07, 0x0a,
    0x03, 0x49, 0x4e, 0x43, 0x10, 0x03, 0x12, 0x07, 0x0a, 0x03, 0x44, 0x45, 0x43, 0x10, 0x04, 0x12,
    0x07, 0x0a, 0x03, 0x41, 0x44, 0x44, 0x10, 0x05, 0x12, 0x07, 0x0a, 0x03, 0x53, 0x55, 0x42, 0x10,
    0x06, 0x12, 0x17, 0x0a, 0x13, 0x53, 0x45, 0x54, 0x5f, 0x54, 0x4f, 0x5f, 0x43, 0x55, 0x52, 0x52,
    0x45, 0x4e, 0x54, 0x5f, 0x54, 0x49, 0x4d, 0x45, 0x10, 0x07, 0x42, 0x3d, 0x5a, 0x3b, 0x67, 0x69,
    0x74, 0x68, 0x75, 0x62, 0x2e, 0x63, 0x6f, 0x6d, 0x2f, 0x70, 0x69, 0x6e, 0x61, 0x78, 0x2d, 0x6e,
    0x65, 0x74, 0x77, 0x6f, 0x72, 0x6b, 0x2f, 0x73, 0x75, 0x62, 0x73, 0x74, 0x72, 0x65, 0x61, 0x6d,
    0x73, 0x2d, 0x73, 0x69, 0x6e, 0x6b, 0x2d, 0x70, 0x72, 0x6f, 0x6d, 0x65, 0x74, 0x68, 0x65, 0x75,
    0x73, 0x2f, 0x70, 0x62, 0x3b, 0x70, 0x62, 0x6b, 0x76, 0x4a, 0xfc, 0x0b, 0x0a, 0x06, 0x12, 0x04,
    0x00, 0x00, 0x22, 0x01, 0x0a, 0x08, 0x0a, 0x01, 0x0c, 0x12, 0x03, 0x00, 0x00, 0x12, 0x0a, 0x08,
    0x0a, 0x01, 0x02, 0x12, 0x03, 0x02, 0x00, 0x2c, 0x0a, 0x08, 0x0a, 0x01, 0x08, 0x12, 0x03, 0x04,
    0x00, 0x52, 0x0a, 0x09, 0x0a, 0x02, 0x08, 0x0b, 0x12, 0x03, 0x04, 0x00, 0x52, 0x0a, 0x5f, 0x0a,
    0x02, 0x04, 0x00, 0x12, 0x04, 0x07, 0x00, 0x09, 0x01, 0x1a, 0x53, 0x20, 0x52, 0x65, 0x66, 0x65,
    0x72, 0x65, 0x6e, 0x63, 0x65, 0x20, 0x68, 0x74, 0x74, 0x70, 0x73, 0x3a, 0x2f, 0x2f, 0x70, 0x6b,
    0x67, 0x2e, 0x67, 0x6f, 0x2e, 0x64, 0x65, 0x76, 0x2f, 0x67, 0x69, 0x74, 0x68, 0x75, 0x62, 0x2e,
    0x63, 0x6f, 0x6d, 0x2f, 0x70, 0x72, 0x6f, 0x6d, 0x65, 0x74, 0x68, 0x65, 0x75, 0x73, 0x2f, 0x63,
    0x6c, 0x69, 0x65, 0x6e, 0x74, 0x5f, 0x67, 0x6f, 0x6c, 0x61, 0x6e, 0x67, 0x2f, 0x70, 0x72, 0x6f,
    0x6d, 0x65, 0x74, 0x68, 0x65, 0x75, 0x73, 0x23, 0x47, 0x61, 0x75, 0x67, 0x65, 0x0a, 0x0a, 0x0a,
    0x0a, 0x03, 0x04, 0x00, 0x01, 0x12, 0x03, 0x07, 0x08, 0x1c, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x00,
    0x02, 0x00, 0x12, 0x03, 0x08, 0x02, 0x2e, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x00, 0x04,
    0x12, 0x03, 0x08, 0x02, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x00, 0x06, 0x12, 0x03,
    0x08, 0x0b, 0x1e, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x00, 0x01, 0x12, 0x03, 0x08, 0x1f,
    0x29, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x00, 0x03, 0x12, 0x03, 0x08, 0x2c, 0x2d, 0x0a,
    0x0a, 0x0a, 0x02, 0x04, 0x01, 0x12, 0x04, 0x0b, 0x00, 0x22, 0x01, 0x0a, 0x0a, 0x0a, 0x03, 0x04,
    0x01, 0x01, 0x12, 0x03, 0x0b, 0x08, 0x1b, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x01, 0x02, 0x00, 0x12,
    0x03, 0x0c, 0x02, 0x1d, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x00, 0x04, 0x12, 0x03, 0x0c,
    0x02, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x00, 0x05, 0x12, 0x03, 0x0c, 0x0b, 0x11,
    0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x00, 0x01, 0x12, 0x03, 0x0c, 0x12, 0x18, 0x0a, 0x0c,
    0x0a, 0x05, 0x04, 0x01, 0x02, 0x00, 0x03, 0x12, 0x03, 0x0c, 0x1b, 0x1c, 0x0a, 0x0b, 0x0a, 0x04,
    0x04, 0x01, 0x02, 0x01, 0x12, 0x03, 0x0d, 0x02, 0x13, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02,
    0x01, 0x05, 0x12, 0x03, 0x0d, 0x02, 0x08, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x01, 0x01,
    0x12, 0x03, 0x0d, 0x09, 0x0e, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x01, 0x03, 0x12, 0x03,
    0x0d, 0x11, 0x12, 0x0a, 0x23, 0x0a, 0x04, 0x04, 0x01, 0x04, 0x00, 0x12, 0x04, 0x0f, 0x02, 0x20,
    0x03, 0x1a, 0x15, 0x20, 0x75, 0x69, 0x6e, 0x74, 0x36, 0x34, 0x20, 0x6f, 0x72, 0x64, 0x69, 0x6e,
    0x61, 0x6c, 0x20, 0x3d, 0x20, 0x33, 0x3b, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x04, 0x00,
    0x01, 0x12, 0x03, 0x0f, 0x07, 0x0b, 0x0a, 0x87, 0x01, 0x0a, 0x06, 0x04, 0x01, 0x04, 0x00, 0x02,
    0x00, 0x12, 0x03, 0x11, 0x04, 0x0e, 0x1a, 0x78, 0x20, 0x50, 0x72, 0x6f, 0x74, 0x6f, 0x62, 0x75,
    0x66, 0x20, 0x64, 0x65, 0x66, 0x61, 0x75, 0x6c, 0x74, 0x20, 0x73, 0x68, 0x6f, 0x75, 0x6c, 0x64,
    0x20, 0x6e, 0x6f, 0x74, 0x20, 0x62, 0x65, 0x20, 0x75, 0x73, 0x65, 0x64, 0x2c, 0x20, 0x74, 0x68,
    0x69, 0x73, 0x20, 0x69, 0x73, 0x20, 0x75, 0x73, 0x65, 0x64, 0x20, 0x73, 0x6f, 0x20, 0x74, 0x68,
    0x61, 0x74, 0x20, 0x74, 0x68, 0x65, 0x20, 0x63, 0x6f, 0x6e, 0x73, 0x75, 0x6d, 0x65, 0x20, 0x63,
    0x61, 0x6e, 0x20, 0x65, 0x6e, 0x73, 0x75, 0x72, 0x65, 0x20, 0x74, 0x68, 0x61, 0x74, 0x20, 0x74,
    0x68, 0x65, 0x20, 0x76, 0x61, 0x6c, 0x75, 0x65, 0x20, 0x77, 0x61, 0x73, 0x20, 0x61, 0x63, 0x74,
    0x75, 0x61, 0x6c, 0x6c, 0x79, 0x20, 0x73, 0x70, 0x65, 0x63, 0x69, 0x66, 0x69, 0x65, 0x64, 0x0a,
    0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x01, 0x04, 0x00, 0x02, 0x00, 0x01, 0x12, 0x03, 0x11, 0x04, 0x09,
    0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x01, 0x04, 0x00, 0x02, 0x00, 0x02, 0x12, 0x03, 0x11, 0x0c, 0x0d,
    0x0a, 0x44, 0x0a, 0x06, 0x04, 0x01, 0x04, 0x00, 0x02, 0x01, 0x12, 0x03, 0x13, 0x04, 0x0c, 0x1a,
    0x2c, 0x20, 0x53, 0x65, 0x74, 0x20, 0x73, 0x65, 0x74, 0x73, 0x20, 0x74, 0x68, 0x65, 0x20, 0x47,
    0x61, 0x75, 0x67, 0x65, 0x20, 0x74, 0x6f, 0x20, 0x61, 0x6e, 0x20, 0x61, 0x72, 0x62, 0x69, 0x74,
    0x72, 0x61, 0x72, 0x79, 0x20, 0x76, 0x61, 0x6c, 0x75, 0x65, 0x2e, 0x20, 0x0a, 0x22, 0x07, 0x20,
    0x66, 0x6c, 0x6f, 0x61, 0x74, 0x0a, 0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x01, 0x04, 0x00, 0x02, 0x01,
    0x01, 0x12, 0x03, 0x13, 0x04, 0x07, 0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x01, 0x04, 0x00, 0x02, 0x01,
    0x02, 0x12, 0x03, 0x13, 0x0a, 0x0b, 0x0a, 0x22, 0x0a, 0x06, 0x04, 0x01, 0x04, 0x00, 0x02, 0x02,
    0x12, 0x03, 0x15, 0x04, 0x0f, 0x1a, 0x13, 0x20, 0x55, 0x6e, 0x73, 0x65, 0x74, 0x73, 0x20, 0x74,
    0x68, 0x65, 0x20, 0x47, 0x61, 0x75, 0x67, 0x65, 0x2e, 0x0a, 0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x01,
    0x04, 0x00, 0x02, 0x02, 0x01, 0x12, 0x03, 0x15, 0x04, 0x0a, 0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x01,
    0x04, 0x00, 0x02, 0x02, 0x02, 0x12, 0x03, 0x15, 0x0d, 0x0e, 0x0a, 0x5c, 0x0a, 0x06, 0x04, 0x01,
    0x04, 0x00, 0x02, 0x03, 0x12, 0x03, 0x17, 0x04, 0x0c, 0x1a, 0x4d, 0x20, 0x49, 0x6e, 0x63, 0x20,
    0x69, 0x6e, 0x63, 0x72, 0x65, 0x6d, 0x65, 0x6e, 0x74, 0x73, 0x20, 0x74, 0x68, 0x65, 0x20, 0x47,
    0x61, 0x75, 0x67, 0x65, 0x20, 0x62, 0x79, 0x20, 0x31, 0x2e, 0x20, 0x55, 0x73, 0x65, 0x20, 0x41,
    0x64, 0x64, 0x20, 0x74, 0x6f, 0x20, 0x69, 0x6e, 0x63, 0x72, 0x65, 0x6d, 0x65, 0x6e, 0x74, 0x20,
    0x69, 0x74, 0x20, 0x62, 0x79, 0x20, 0x61, 0x72, 0x62, 0x69, 0x74, 0x72, 0x61, 0x72, 0x79, 0x20,
    0x76, 0x61, 0x6c, 0x75, 0x65, 0x73, 0x2e, 0x0a, 0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x01, 0x04, 0x00,
    0x02, 0x03, 0x01, 0x12, 0x03, 0x17, 0x04, 0x07, 0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x01, 0x04, 0x00,
    0x02, 0x03, 0x02, 0x12, 0x03, 0x17, 0x0a, 0x0b, 0x0a, 0x5c, 0x0a, 0x06, 0x04, 0x01, 0x04, 0x00,
    0x02, 0x04, 0x12, 0x03, 0x19, 0x04, 0x0c, 0x1a, 0x4d, 0x20, 0x44, 0x65, 0x63, 0x20, 0x64, 0x65,
    0x63, 0x72, 0x65, 0x6d, 0x65, 0x6e, 0x74, 0x73, 0x20, 0x74, 0x68, 0x65, 0x20, 0x47, 0x61, 0x75,
    0x67, 0x65, 0x20, 0x62, 0x79, 0x20, 0x31, 0x2e, 0x20, 0x55, 0x73, 0x65, 0x20, 0x53, 0x75, 0x62,
    0x20, 0x74, 0x6f, 0x20, 0x64, 0x65, 0x63, 0x72, 0x65, 0x6d, 0x65, 0x6e, 0x74, 0x20, 0x69, 0x74,
    0x20, 0x62, 0x79, 0x20, 0x61, 0x72, 0x62, 0x69, 0x74, 0x72, 0x61, 0x72, 0x79, 0x20, 0x76, 0x61,
    0x6c, 0x75, 0x65, 0x73, 0x2e, 0x0a, 0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x01, 0x04, 0x00, 0x02, 0x04,
    0x01, 0x12, 0x03, 0x19, 0x04, 0x07, 0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x01, 0x04, 0x00, 0x02, 0x04,
    0x02, 0x12, 0x03, 0x19, 0x0a, 0x0b, 0x0a, 0x83, 0x01, 0x0a, 0x06, 0x04, 0x01, 0x04, 0x00, 0x02,
    0x05, 0x12, 0x03, 0x1b, 0x04, 0x0c, 0x1a, 0x6b, 0x20, 0x41, 0x64, 0x64, 0x20, 0x61, 0x64, 0x64,
    0x73, 0x20, 0x74, 0x68, 0x65, 0x20, 0x67, 0x69, 0x76, 0x65, 0x6e, 0x20, 0x76, 0x61, 0x6c, 0x75,
    0x65, 0x20, 0x74, 0x6f, 0x20, 0x74, 0x68, 0x65, 0x20, 0x47, 0x61, 0x75, 0x67, 0x65, 0x2e, 0x20,
    0x28, 0x54, 0x68, 0x65, 0x20, 0x76, 0x61, 0x6c, 0x75, 0x65, 0x20, 0x63, 0x61, 0x6e, 0x20, 0x62,
    0x65, 0x20, 0x6e, 0x65, 0x67, 0x61, 0x74, 0x69, 0x76, 0x65, 0x2c, 0x20, 0x72, 0x65, 0x73, 0x75,
    0x6c, 0x74, 0x69, 0x6e, 0x67, 0x20, 0x69, 0x6e, 0x20, 0x61, 0x20, 0x64, 0x65, 0x63, 0x72, 0x65,
    0x61, 0x73, 0x65, 0x20, 0x6f, 0x66, 0x20, 0x74, 0x68, 0x65, 0x20, 0x47, 0x61, 0x75, 0x67, 0x65,
    0x2e, 0x29, 0x0a, 0x22, 0x07, 0x20, 0x66, 0x6c, 0x6f, 0x61, 0x74, 0x0a, 0x0a, 0x0e, 0x0a, 0x07,
    0x04, 0x01, 0x04, 0x00, 0x02, 0x05, 0x01, 0x12, 0x03, 0x1b, 0x04, 0x07, 0x0a, 0x0e, 0x0a, 0x07,
    0x04, 0x01, 0x04, 0x00, 0x02, 0x05, 0x02, 0x12, 0x03, 0x1b, 0x0a, 0x0b, 0x0a, 0x8b, 0x01, 0x0a,
    0x06, 0x04, 0x01, 0x04, 0x00, 0x02, 0x06, 0x12, 0x03, 0x1d, 0x04, 0x0c, 0x1a, 0x73, 0x20, 0x53,
    0x75, 0x62, 0x20, 0x73, 0x75, 0x62, 0x74, 0x72, 0x61, 0x63, 0x74, 0x73, 0x20, 0x74, 0x68, 0x65,
    0x20, 0x67, 0x69, 0x76, 0x65, 0x6e, 0x20, 0x76, 0x61, 0x6c, 0x75, 0x65, 0x20, 0x66, 0x72, 0x6f,
    0x6d, 0x20, 0x74, 0x68, 0x65, 0x20, 0x47, 0x61, 0x75, 0x67, 0x65, 0x2e, 0x20, 0x28, 0x54, 0x68,
    0x65, 0x20, 0x76, 0x61, 0x6c, 0x75, 0x65, 0x20, 0x63, 0x61, 0x6e, 0x20, 0x62, 0x65, 0x20, 0x6e,
    0x65, 0x67, 0x61, 0x74, 0x69, 0x76, 0x65, 0x2c, 0x20, 0x72, 0x65, 0x73, 0x75, 0x6c, 0x74, 0x69,
    0x6e, 0x67, 0x20, 0x69, 0x6e, 0x20, 0x61, 0x6e, 0x20, 0x69, 0x6e, 0x63, 0x72, 0x65, 0x61, 0x73,
    0x65, 0x20, 0x6f, 0x66, 0x20, 0x74, 0x68, 0x65, 0x20, 0x47, 0x61, 0x75, 0x67, 0x65, 0x2e, 0x29,
    0x0a, 0x22, 0x07, 0x20, 0x66, 0x6c, 0x6f, 0x61, 0x74, 0x0a, 0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x01,
    0x04, 0x00, 0x02, 0x06, 0x01, 0x12, 0x03, 0x1d, 0x04, 0x07, 0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x01,
    0x04, 0x00, 0x02, 0x06, 0x02, 0x12, 0x03, 0x1d, 0x0a, 0x0b, 0x0a, 0x55, 0x0a, 0x06, 0x04, 0x01,
    0x04, 0x00, 0x02, 0x07, 0x12, 0x03, 0x1f, 0x04, 0x1c, 0x1a, 0x46, 0x20, 0x53, 0x65, 0x74, 0x54,
    0x6f, 0x43, 0x75, 0x72, 0x72, 0x65, 0x6e, 0x74, 0x54, 0x69, 0x6d, 0x65, 0x20, 0x73, 0x65, 0x74,
    0x73, 0x20, 0x74, 0x68, 0x65, 0x20, 0x47, 0x61, 0x75, 0x67, 0x65, 0x20, 0x74, 0x6f, 0x20, 0x74,
    0x68, 0x65, 0x20, 0x63, 0x75, 0x72, 0x72, 0x65, 0x6e, 0x74, 0x20, 0x55, 0x6e, 0x69, 0x78, 0x20,
    0x74, 0x69, 0x6d, 0x65, 0x20, 0x69, 0x6e, 0x20, 0x73, 0x65, 0x63, 0x6f, 0x6e, 0x64, 0x73, 0x2e,
    0x0a, 0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x01, 0x04, 0x00, 0x02, 0x07, 0x01, 0x12, 0x03, 0x1f, 0x04,
    0x17, 0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x01, 0x04, 0x00, 0x02, 0x07, 0x02, 0x12, 0x03, 0x1f, 0x1a,
    0x1b, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x01, 0x02, 0x02, 0x12, 0x03, 0x21, 0x02, 0x10, 0x0a, 0x0c,
    0x0a, 0x05, 0x04, 0x01, 0x02, 0x02, 0x06, 0x12, 0x03, 0x21, 0x02, 0x06, 0x0a, 0x0c, 0x0a, 0x05,
    0x04, 0x01, 0x02, 0x02, 0x01, 0x12, 0x03, 0x21, 0x07, 0x0b, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01,
    0x02, 0x02, 0x03, 0x12, 0x03, 0x21, 0x0e, 0x0f, 0x62, 0x06, 0x70, 0x72, 0x6f, 0x74, 0x6f, 0x33,
];
// @@protoc_insertion_point(module)