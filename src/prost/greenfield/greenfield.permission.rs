// @generated
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Statement {
    /// effect define the impact of permissions, which can be Allow/Deny
    #[prost(enumeration = "Effect", tag = "1")]
    pub effect: i32,
    /// action_type define the operation type you can act. greenfield defines a set of permission
    /// that you can specify in a permissionInfo. see ActionType enum for detail.
    #[prost(enumeration = "ActionType", repeated, tag = "2")]
    pub actions: ::prost::alloc::vec::Vec<i32>,
    /// CAN ONLY USED IN bucket level. Support fuzzy match and limit to 5.
    /// The sub-resource name must comply with the standard specified in the greenfield/types/grn.go file for Greenfield resource names.
    /// If the sub-resources include 'grn:o:{bucket_name}/*' in the statement, it indicates that specific permissions is granted to all objects within the specified bucket.
    /// If the sub-resources include 'grn:o:{bucket_name}/test_*' in the statement, it indicates that specific permissions is granted to all objects with the `test_` prefix within the specified bucket.
    /// If the sub-resources is empty, when you need to operate(excluding CreateObject) a specified subresource, it will be denied because it cannot match any subresource.
    #[prost(string, repeated, tag = "3")]
    pub resources: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    /// expiration_time defines how long the permission is valid. If not explicitly specified, it means it will not expire.
    #[prost(message, optional, tag = "4")]
    pub expiration_time: ::core::option::Option<::prost_types::Timestamp>,
    /// limit_size defines the total data size that is allowed to operate. If not explicitly specified, it means it will not limit.
    #[prost(message, optional, tag = "5")]
    pub limit_size: ::core::option::Option<super::common::UInt64Value>,
}
/// Principal define the roles that can grant permissions. Currently, it can be account or group.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Principal {
    #[prost(enumeration = "PrincipalType", tag = "1")]
    pub r#type: i32,
    /// When the type is an account, its value is sdk.AccAddress().String();
    /// when the type is a group, its value is math.Uint().String()
    #[prost(string, tag = "2")]
    pub value: ::prost::alloc::string::String,
}
/// ActionType defines the operations you can execute in greenfield storage network
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum ActionType {
    ActionUnspecified = 0,
    ActionUpdateBucketInfo = 1,
    ActionDeleteBucket = 2,
    ActionCreateObject = 3,
    ActionDeleteObject = 4,
    ActionCopyObject = 5,
    ActionGetObject = 6,
    ActionExecuteObject = 7,
    ActionListObject = 8,
    ActionUpdateGroupMember = 9,
    ActionDeleteGroup = 10,
    ActionUpdateObjectInfo = 11,
    ActionUpdateGroupExtra = 12,
    All = 99,
}
impl ActionType {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            ActionType::ActionUnspecified => "ACTION_UNSPECIFIED",
            ActionType::ActionUpdateBucketInfo => "ACTION_UPDATE_BUCKET_INFO",
            ActionType::ActionDeleteBucket => "ACTION_DELETE_BUCKET",
            ActionType::ActionCreateObject => "ACTION_CREATE_OBJECT",
            ActionType::ActionDeleteObject => "ACTION_DELETE_OBJECT",
            ActionType::ActionCopyObject => "ACTION_COPY_OBJECT",
            ActionType::ActionGetObject => "ACTION_GET_OBJECT",
            ActionType::ActionExecuteObject => "ACTION_EXECUTE_OBJECT",
            ActionType::ActionListObject => "ACTION_LIST_OBJECT",
            ActionType::ActionUpdateGroupMember => "ACTION_UPDATE_GROUP_MEMBER",
            ActionType::ActionDeleteGroup => "ACTION_DELETE_GROUP",
            ActionType::ActionUpdateObjectInfo => "ACTION_UPDATE_OBJECT_INFO",
            ActionType::ActionUpdateGroupExtra => "ACTION_UPDATE_GROUP_EXTRA",
            ActionType::All => "ACTION_TYPE_ALL",
        }
    }
}
/// Effect define the effect of the operation permission, include Allow or deny
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum Effect {
    Unspecified = 0,
    Allow = 1,
    Deny = 2,
}
impl Effect {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            Effect::Unspecified => "EFFECT_UNSPECIFIED",
            Effect::Allow => "EFFECT_ALLOW",
            Effect::Deny => "EFFECT_DENY",
        }
    }
}
/// PrincipalType refers to the identity type of system users or entities.
/// In Greenfield, it usually refers to accounts or groups.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum PrincipalType {
    Unspecified = 0,
    GnfdAccount = 1,
    GnfdGroup = 2,
}
impl PrincipalType {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            PrincipalType::Unspecified => "PRINCIPAL_TYPE_UNSPECIFIED",
            PrincipalType::GnfdAccount => "PRINCIPAL_TYPE_GNFD_ACCOUNT",
            PrincipalType::GnfdGroup => "PRINCIPAL_TYPE_GNFD_GROUP",
        }
    }
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Policy {
    /// id is an unique u256 sequence for each policy. It also be used as NFT tokenID
    #[prost(string, tag = "1")]
    pub id: ::prost::alloc::string::String,
    /// principal defines the accounts/group which the permission grants to
    #[prost(message, optional, tag = "2")]
    pub principal: ::core::option::Option<Principal>,
    /// resource_type defines the type of resource that grants permission for
    #[prost(enumeration = "super::resource::ResourceType", tag = "3")]
    pub resource_type: i32,
    /// resource_id defines the bucket/object/group id of the resource that grants permission for
    #[prost(string, tag = "4")]
    pub resource_id: ::prost::alloc::string::String,
    /// statements defines the details content of the permission, including effect/actions/sub-resources
    #[prost(message, repeated, tag = "5")]
    pub statements: ::prost::alloc::vec::Vec<Statement>,
    /// expiration_time defines the whole expiration time of all the statements.
    /// Notices: Its priority is higher than the expiration time inside the Statement
    #[prost(message, optional, tag = "6")]
    pub expiration_time: ::core::option::Option<::prost_types::Timestamp>,
}
/// PolicyGroup refers to a group of policies which grant permission to Group, which is limited to MaxGroupNum (default 10).
/// This means that a single resource can only grant permission to 10 groups. The reason for
/// this is to enable on-chain determination of whether an operator has permission within a limited time.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PolicyGroup {
    /// items define a pair of policy_id and group_id. Each resource can only grant its own permissions to a limited number of groups
    #[prost(message, repeated, tag = "1")]
    pub items: ::prost::alloc::vec::Vec<policy_group::Item>,
}
/// Nested message and enum types in `PolicyGroup`.
pub mod policy_group {
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Item {
        #[prost(string, tag = "1")]
        pub policy_id: ::prost::alloc::string::String,
        #[prost(string, tag = "2")]
        pub group_id: ::prost::alloc::string::String,
    }
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GroupMember {
    /// id is an unique u256 sequence for each group member. It also be used as NFT tokenID
    #[prost(string, tag = "1")]
    pub id: ::prost::alloc::string::String,
    /// group_id is the unique id of the group
    #[prost(string, tag = "2")]
    pub group_id: ::prost::alloc::string::String,
    /// member is the account address of the member
    #[prost(string, tag = "3")]
    pub member: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct EventPutPolicy {
    /// policy_id is an unique u256 sequence for each policy. It also be used as NFT tokenID
    #[prost(string, tag = "1")]
    pub policy_id: ::prost::alloc::string::String,
    /// principal defines the accounts/group which the permission grants to
    #[prost(message, optional, tag = "2")]
    pub principal: ::core::option::Option<Principal>,
    /// resource_type defines the type of resource that grants permission for
    #[prost(enumeration = "super::resource::ResourceType", tag = "3")]
    pub resource_type: i32,
    /// resource_id defines the bucket/object/group id of the resource that grants permission for
    #[prost(string, tag = "4")]
    pub resource_id: ::prost::alloc::string::String,
    /// statements defines the details content of the permission, include effect/actions/sub-resources
    #[prost(message, repeated, tag = "5")]
    pub statements: ::prost::alloc::vec::Vec<Statement>,
    /// expiration_time defines the whole expiration time of all the statements.
    /// Notices: Its priority is higher than the expiration time inside the Statement
    #[prost(message, optional, tag = "6")]
    pub expiration_time: ::core::option::Option<::prost_types::Timestamp>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct EventDeletePolicy {
    /// policy_id is an unique u256 sequence for each policy. It also be used as NFT tokenID
    #[prost(string, tag = "1")]
    pub policy_id: ::prost::alloc::string::String,
}
/// Params defines the parameters for the module.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Params {
    /// maximum_statements_num defines the maximum number of statements allowed in a policy
    #[prost(uint64, tag = "1")]
    pub maximum_statements_num: u64,
    /// maximum_group_num used to set the upper limit on the number of groups to which a resource can grant access permissions.
    /// By placing a cap on the number of group permissions, permission control policies can be made more robust and better
    /// enforced, thereby reducing the chances of DDos and other security incidents.
    #[prost(uint64, tag = "2")]
    pub maximum_group_num: u64,
}
/// GenesisState defines the permission module's genesis state.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GenesisState {
    #[prost(message, optional, tag = "1")]
    pub params: ::core::option::Option<Params>,
}
/// QueryParamsRequest is request type for the Query/Params RPC method.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryParamsRequest {}
/// QueryParamsResponse is response type for the Query/Params RPC method.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryParamsResponse {
    /// params holds all the parameters of this module.
    #[prost(message, optional, tag = "1")]
    pub params: ::core::option::Option<Params>,
}
/// MsgUpdateParams is the Msg/UpdateParams request type.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgUpdateParams {
    /// authority is the address that controls the module (defaults to x/gov unless overwritten).
    #[prost(string, tag = "1")]
    pub authority: ::prost::alloc::string::String,
    /// params defines the x/permission parameters to update.
    ///
    /// NOTE: All parameters must be supplied.
    #[prost(message, optional, tag = "2")]
    pub params: ::core::option::Option<Params>,
}
/// MsgUpdateParamsResponse defines the response structure for executing a
/// MsgUpdateParams message.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgUpdateParamsResponse {}
include!("greenfield.permission.tonic.rs");
// @@protoc_insertion_point(module)
