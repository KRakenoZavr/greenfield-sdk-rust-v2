// @generated
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum ResourceType {
    Unspecified = 0,
    Bucket = 1,
    Object = 2,
    Group = 3,
}
impl ResourceType {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            ResourceType::Unspecified => "RESOURCE_TYPE_UNSPECIFIED",
            ResourceType::Bucket => "RESOURCE_TYPE_BUCKET",
            ResourceType::Object => "RESOURCE_TYPE_OBJECT",
            ResourceType::Group => "RESOURCE_TYPE_GROUP",
        }
    }
}
// @@protoc_insertion_point(module)
