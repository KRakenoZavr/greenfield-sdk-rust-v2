// @generated
/// Wrapper message for `double`.
///
/// The JSON representation for `DoubleValue` is JSON number.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DoubleValue {
    /// The double value.
    #[prost(double, tag = "1")]
    pub value: f64,
}
/// Wrapper message for `float`.
///
/// The JSON representation for `FloatValue` is JSON number.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct FloatValue {
    /// The float value.
    #[prost(float, tag = "1")]
    pub value: f32,
}
/// Wrapper message for `int64`.
///
/// The JSON representation for `Int64Value` is JSON string.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Int64Value {
    /// The int64 value.
    #[prost(int64, tag = "1")]
    pub value: i64,
}
/// Wrapper message for `uint64`.
///
/// The JSON representation for `UInt64Value` is JSON string.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UInt64Value {
    /// The uint64 value.
    #[prost(uint64, tag = "1")]
    pub value: u64,
}
/// Wrapper message for `int32`.
///
/// The JSON representation for `Int32Value` is JSON number.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Int32Value {
    /// The int32 value.
    #[prost(int32, tag = "1")]
    pub value: i32,
}
/// Wrapper message for `uint32`.
///
/// The JSON representation for `UInt32Value` is JSON number.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UInt32Value {
    /// The uint32 value.
    #[prost(uint32, tag = "1")]
    pub value: u32,
}
/// Wrapper message for `bool`.
///
/// The JSON representation for `BoolValue` is JSON `true` and `false`.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct BoolValue {
    /// The bool value.
    #[prost(bool, tag = "1")]
    pub value: bool,
}
/// Wrapper message for `string`.
///
/// The JSON representation for `StringValue` is JSON string.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct StringValue {
    /// The string value.
    #[prost(string, tag = "1")]
    pub value: ::prost::alloc::string::String,
}
/// Wrapper message for `bytes`.
///
/// The JSON representation for `BytesValue` is JSON string.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct BytesValue {
    /// The bytes value.
    #[prost(bytes = "vec", tag = "1")]
    pub value: ::prost::alloc::vec::Vec<u8>,
}
// @@protoc_insertion_point(module)
