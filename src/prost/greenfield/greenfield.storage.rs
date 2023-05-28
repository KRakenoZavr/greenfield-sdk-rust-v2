// @generated
/// Approval is the signature information returned by the Primary Storage Provider (SP) to the user
/// after allowing them to create a bucket or object, which is then used for verification on the chain
/// to ensure agreement between the Primary SP and the user."
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Approval {
    /// expired_height is the block height at which the signature expires.
    #[prost(uint64, tag = "1")]
    pub expired_height: u64,
    /// The signature needs to conform to the EIP 712 specification.
    #[prost(bytes = "vec", tag = "2")]
    pub sig: ::prost::alloc::vec::Vec<u8>,
}
/// SecondarySpSignDoc used to generate seal signature of secondary SP
/// If the secondary SP only signs the checksum to declare the object pieces are saved,
/// it might be reused by the primary SP to fake it's declaration.
/// Then the primary SP can challenge and slash the secondary SP.
/// So the id of the object is needed to prevent this.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SecondarySpSignDoc {
    #[prost(string, tag = "1")]
    pub sp_address: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub object_id: ::prost::alloc::string::String,
    #[prost(bytes = "vec", tag = "3")]
    pub checksum: ::prost::alloc::vec::Vec<u8>,
}
/// SourceType represents the source of resource creation, which can
/// from Greenfield native or from a cross-chain transfer from BSC
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum SourceType {
    Origin = 0,
    BscCrossChain = 1,
    MirrorPending = 2,
}
impl SourceType {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            SourceType::Origin => "SOURCE_TYPE_ORIGIN",
            SourceType::BscCrossChain => "SOURCE_TYPE_BSC_CROSS_CHAIN",
            SourceType::MirrorPending => "SOURCE_TYPE_MIRROR_PENDING",
        }
    }
}
/// BucketStatus represents the status of a bucket. After a user successfully
/// sends a CreateBucket transaction onto the chain, the status is set to 'Created'.
/// When a Discontinue Object transaction is received on chain, the status is set to 'Discontinued'.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum BucketStatus {
    Created = 0,
    Discontinued = 1,
}
impl BucketStatus {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            BucketStatus::Created => "BUCKET_STATUS_CREATED",
            BucketStatus::Discontinued => "BUCKET_STATUS_DISCONTINUED",
        }
    }
}
/// RedundancyType represents the redundancy algorithm type for object data,
/// which can be either multi-replica or erasure coding.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum RedundancyType {
    RedundancyEcType = 0,
    RedundancyReplicaType = 1,
}
impl RedundancyType {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            RedundancyType::RedundancyEcType => "REDUNDANCY_EC_TYPE",
            RedundancyType::RedundancyReplicaType => "REDUNDANCY_REPLICA_TYPE",
        }
    }
}
/// ObjectStatus represents the creation status of an object. After a user successfully
/// sends a CreateObject transaction onto the chain, the status is set to 'Created'.
/// After the Primary Service Provider successfully sends a Seal Object transaction onto
/// the chain, the status is set to 'Sealed'. When a Discontinue Object transaction is
/// received on chain, the status is set to 'Discontinued'.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum ObjectStatus {
    Created = 0,
    Sealed = 1,
    Discontinued = 2,
}
impl ObjectStatus {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            ObjectStatus::Created => "OBJECT_STATUS_CREATED",
            ObjectStatus::Sealed => "OBJECT_STATUS_SEALED",
            ObjectStatus::Discontinued => "OBJECT_STATUS_DISCONTINUED",
        }
    }
}
/// VisibilityType is the resources public status.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum VisibilityType {
    Unspecified = 0,
    PublicRead = 1,
    Private = 2,
    /// If the bucket Visibility is inherit, it's finally set to private. If the object Visibility is inherit, it's the same as bucket.
    Inherit = 3,
}
impl VisibilityType {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            VisibilityType::Unspecified => "VISIBILITY_TYPE_UNSPECIFIED",
            VisibilityType::PublicRead => "VISIBILITY_TYPE_PUBLIC_READ",
            VisibilityType::Private => "VISIBILITY_TYPE_PRIVATE",
            VisibilityType::Inherit => "VISIBILITY_TYPE_INHERIT",
        }
    }
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct BucketInfo {
    /// owner is the account address of bucket creator, it is also the bucket owner.
    #[prost(string, tag = "1")]
    pub owner: ::prost::alloc::string::String,
    /// bucket_name is a globally unique name of bucket
    #[prost(string, tag = "2")]
    pub bucket_name: ::prost::alloc::string::String,
    /// visibility defines the highest permissions for bucket. When a bucket is public, everyone can get storage objects in it.
    #[prost(enumeration = "VisibilityType", tag = "3")]
    pub visibility: i32,
    /// id is the unique identification for bucket.
    #[prost(string, tag = "4")]
    pub id: ::prost::alloc::string::String,
    /// source_type defines which chain the user should send the bucket management transactions to
    #[prost(enumeration = "SourceType", tag = "5")]
    pub source_type: i32,
    /// create_at define the block timestamp when the bucket created.
    #[prost(int64, tag = "6")]
    pub create_at: i64,
    /// payment_address is the address of the payment account
    #[prost(string, tag = "7")]
    pub payment_address: ::prost::alloc::string::String,
    /// primary_sp_address is the address of the primary sp. Objects belongs to this bucket will never
    /// leave this SP, unless you explicitly shift them to another SP.
    #[prost(string, tag = "8")]
    pub primary_sp_address: ::prost::alloc::string::String,
    /// charged_read_quota defines the traffic quota for read in bytes per month.
    /// The available read data for each user is the sum of the free read data provided by SP and
    /// the ChargeReadQuota specified here.
    #[prost(uint64, tag = "9")]
    pub charged_read_quota: u64,
    /// billing info of the bucket
    #[prost(message, optional, tag = "10")]
    pub billing_info: ::core::option::Option<BillingInfo>,
    /// bucket_status define the status of the bucket.
    #[prost(enumeration = "BucketStatus", tag = "11")]
    pub bucket_status: i32,
}
/// BillingInfo is the billing information of the bucket
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct BillingInfo {
    /// the time of the payment price, used to calculate the charge rate of the bucket
    #[prost(int64, tag = "1")]
    pub price_time: i64,
    /// the total size of the objects in the bucket, used to calculate the charge rate of the bucket
    #[prost(uint64, tag = "2")]
    pub total_charge_size: u64,
    /// secondary sp objects size statistics
    #[prost(message, repeated, tag = "3")]
    pub secondary_sp_objects_size: ::prost::alloc::vec::Vec<SecondarySpObjectsSize>,
}
/// secondary sp objects size statistics
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SecondarySpObjectsSize {
    /// address is the address of the secondary sp
    #[prost(string, tag = "1")]
    pub sp_address: ::prost::alloc::string::String,
    /// size is the total size of the objects in the secondary sp
    #[prost(uint64, tag = "2")]
    pub total_charge_size: u64,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ObjectInfo {
    #[prost(string, tag = "1")]
    pub owner: ::prost::alloc::string::String,
    /// bucket_name is the name of the bucket
    #[prost(string, tag = "2")]
    pub bucket_name: ::prost::alloc::string::String,
    /// object_name is the name of object
    #[prost(string, tag = "3")]
    pub object_name: ::prost::alloc::string::String,
    /// id is the unique identifier of object
    #[prost(string, tag = "4")]
    pub id: ::prost::alloc::string::String,
    /// payloadSize is the total size of the object payload
    #[prost(uint64, tag = "5")]
    pub payload_size: u64,
    /// visibility defines the highest permissions for object. When an object is public, everyone can access it.
    #[prost(enumeration = "VisibilityType", tag = "6")]
    pub visibility: i32,
    /// content_type define the format of the object which should be a standard MIME type.
    #[prost(string, tag = "7")]
    pub content_type: ::prost::alloc::string::String,
    /// create_at define the block timestamp when the object is created
    #[prost(int64, tag = "8")]
    pub create_at: i64,
    /// object_status define the upload status of the object.
    #[prost(enumeration = "ObjectStatus", tag = "9")]
    pub object_status: i32,
    /// redundancy_type define the type of the redundancy which can be multi-replication or EC.
    #[prost(enumeration = "RedundancyType", tag = "10")]
    pub redundancy_type: i32,
    /// source_type define the source of the object.
    #[prost(enumeration = "SourceType", tag = "11")]
    pub source_type: i32,
    /// checksums define the root hash of the pieces which stored in a SP.
    /// add omit tag to omit the field when converting to NFT metadata
    #[prost(bytes = "vec", repeated, tag = "12")]
    pub checksums: ::prost::alloc::vec::Vec<::prost::alloc::vec::Vec<u8>>,
    /// secondary_sp_addresses define the addresses of secondary_sps
    #[prost(string, repeated, tag = "13")]
    pub secondary_sp_addresses: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GroupInfo {
    /// owner is the owner of the group. It can not changed once it created.
    #[prost(string, tag = "1")]
    pub owner: ::prost::alloc::string::String,
    /// group_name is the name of group which is unique under an account.
    #[prost(string, tag = "2")]
    pub group_name: ::prost::alloc::string::String,
    /// source_type
    #[prost(enumeration = "SourceType", tag = "3")]
    pub source_type: i32,
    /// id is the unique identifier of group
    #[prost(string, tag = "4")]
    pub id: ::prost::alloc::string::String,
    /// extra is used to store extra info for the group
    #[prost(string, tag = "5")]
    pub extra: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Trait {
    #[prost(string, tag = "1")]
    pub trait_type: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub value: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct BucketMetaData {
    /// description
    #[prost(string, tag = "1")]
    pub description: ::prost::alloc::string::String,
    /// externalUrl a link to external site to view NFT
    #[prost(string, tag = "2")]
    pub external_url: ::prost::alloc::string::String,
    /// name of bucket NFT
    #[prost(string, tag = "3")]
    pub bucket_name: ::prost::alloc::string::String,
    /// image is the link to image
    #[prost(string, tag = "4")]
    pub image: ::prost::alloc::string::String,
    /// attributes
    #[prost(message, repeated, tag = "5")]
    pub attributes: ::prost::alloc::vec::Vec<Trait>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ObjectMetaData {
    /// description
    #[prost(string, tag = "1")]
    pub description: ::prost::alloc::string::String,
    /// externalUrl a link to external site to view NFT
    #[prost(string, tag = "2")]
    pub external_url: ::prost::alloc::string::String,
    /// name of object NFT
    #[prost(string, tag = "3")]
    pub object_name: ::prost::alloc::string::String,
    /// image is the link to image
    #[prost(string, tag = "4")]
    pub image: ::prost::alloc::string::String,
    /// attributes
    #[prost(message, repeated, tag = "5")]
    pub attributes: ::prost::alloc::vec::Vec<Trait>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GroupMetaData {
    /// description
    #[prost(string, tag = "1")]
    pub description: ::prost::alloc::string::String,
    /// externalUrl a link to external site to view NFT
    #[prost(string, tag = "2")]
    pub external_url: ::prost::alloc::string::String,
    /// name of group NFT
    #[prost(string, tag = "3")]
    pub group_name: ::prost::alloc::string::String,
    /// image is the link to image
    #[prost(string, tag = "4")]
    pub image: ::prost::alloc::string::String,
    /// attributes
    #[prost(message, repeated, tag = "5")]
    pub attributes: ::prost::alloc::vec::Vec<Trait>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Ids {
    /// ids of the objects or buckets
    #[prost(string, repeated, tag = "1")]
    pub id: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DeleteInfo {
    #[prost(message, optional, tag = "1")]
    pub bucket_ids: ::core::option::Option<Ids>,
    #[prost(message, optional, tag = "2")]
    pub object_ids: ::core::option::Option<Ids>,
    #[prost(message, optional, tag = "3")]
    pub group_ids: ::core::option::Option<Ids>,
}
/// EventCreateBucket is emitted on MsgCreateBucket
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct EventCreateBucket {
    /// owner define the account address of bucket owner
    #[prost(string, tag = "1")]
    pub owner: ::prost::alloc::string::String,
    /// bucket_name is a globally unique name of bucket
    #[prost(string, tag = "2")]
    pub bucket_name: ::prost::alloc::string::String,
    /// visibility defines the highest permissions for bucket. When a bucket is public, everyone can get the object under it.
    #[prost(enumeration = "VisibilityType", tag = "3")]
    pub visibility: i32,
    /// create_at define the block timestamp when the bucket has been created
    #[prost(int64, tag = "4")]
    pub create_at: i64,
    /// bucket_id is the unique u256 for bucket. Not global, only unique in buckets.
    #[prost(string, tag = "5")]
    pub bucket_id: ::prost::alloc::string::String,
    /// source_type define the source of the bucket. CrossChain or Greenfield origin
    #[prost(enumeration = "SourceType", tag = "6")]
    pub source_type: i32,
    /// read_quota defines the charged traffic quota for read, not include free quota which provided by each storage provider
    #[prost(uint64, tag = "7")]
    pub charged_read_quota: u64,
    /// payment_address is the address of the payment account
    #[prost(string, tag = "8")]
    pub payment_address: ::prost::alloc::string::String,
    /// primary_sp_address is the operator address of the primary sp.
    #[prost(string, tag = "9")]
    pub primary_sp_address: ::prost::alloc::string::String,
    /// status define the status of the bucket.
    #[prost(enumeration = "BucketStatus", tag = "10")]
    pub status: i32,
}
/// EventDeleteBucket is emitted on MsgDeleteBucket
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct EventDeleteBucket {
    /// operator define the account address of operator who delete the bucket
    #[prost(string, tag = "1")]
    pub operator: ::prost::alloc::string::String,
    /// owner define the account address of the bucket owner
    #[prost(string, tag = "2")]
    pub owner: ::prost::alloc::string::String,
    /// bucket_name define the name of the deleted bucket
    #[prost(string, tag = "3")]
    pub bucket_name: ::prost::alloc::string::String,
    /// bucket_id define an u256 id for bucket
    #[prost(string, tag = "4")]
    pub bucket_id: ::prost::alloc::string::String,
    /// primary_sp_address define the account address of primary sp
    #[prost(string, tag = "5")]
    pub primary_sp_address: ::prost::alloc::string::String,
}
/// EventUpdateBucketInfo is emitted on MsgUpdateBucketInfo
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct EventUpdateBucketInfo {
    /// operator define the account address of operator who update the bucket
    #[prost(string, tag = "1")]
    pub operator: ::prost::alloc::string::String,
    /// bucket_name define the name of the bucket
    #[prost(string, tag = "2")]
    pub bucket_name: ::prost::alloc::string::String,
    /// bucket_id define an u256 id for bucket
    #[prost(string, tag = "3")]
    pub bucket_id: ::prost::alloc::string::String,
    /// charged_read_quota_before define the read quota before updated
    #[prost(uint64, tag = "4")]
    pub charged_read_quota_before: u64,
    /// charged_read_quota_after define the read quota after updated
    #[prost(uint64, tag = "5")]
    pub charged_read_quota_after: u64,
    /// payment_address_before define the payment address before updated
    #[prost(string, tag = "6")]
    pub payment_address_before: ::prost::alloc::string::String,
    /// payment_address_after define the payment address after updated
    #[prost(string, tag = "7")]
    pub payment_address_after: ::prost::alloc::string::String,
    /// visibility defines the highest permission of object.
    #[prost(enumeration = "VisibilityType", tag = "8")]
    pub visibility: i32,
}
/// EventDiscontinueBucket is emitted on MsgDiscontinueBucket
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct EventDiscontinueBucket {
    /// bucket_id define id of the bucket
    #[prost(string, tag = "1")]
    pub bucket_id: ::prost::alloc::string::String,
    /// bucket_name define the name of the bucket
    #[prost(string, tag = "2")]
    pub bucket_name: ::prost::alloc::string::String,
    /// the reason
    #[prost(string, tag = "3")]
    pub reason: ::prost::alloc::string::String,
    /// the timestamp after which the metadata will be deleted
    #[prost(int64, tag = "4")]
    pub delete_at: i64,
}
/// EventCreateObject is emitted on MsgCreateObject
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct EventCreateObject {
    /// creator define the account address of msg creator
    #[prost(string, tag = "1")]
    pub creator: ::prost::alloc::string::String,
    /// owner define the account address of object owner
    #[prost(string, tag = "2")]
    pub owner: ::prost::alloc::string::String,
    /// bucket_name define the name of bucket
    #[prost(string, tag = "3")]
    pub bucket_name: ::prost::alloc::string::String,
    /// object_name define the name of object
    #[prost(string, tag = "4")]
    pub object_name: ::prost::alloc::string::String,
    /// bucket_id define an u256 id for object
    #[prost(string, tag = "6")]
    pub bucket_id: ::prost::alloc::string::String,
    /// object_id define an u256 id for object
    #[prost(string, tag = "7")]
    pub object_id: ::prost::alloc::string::String,
    /// primary_sp_address define the account address of primary sp
    #[prost(string, tag = "8")]
    pub primary_sp_address: ::prost::alloc::string::String,
    /// payload_size define the size of payload data which you want upload
    #[prost(uint64, tag = "9")]
    pub payload_size: u64,
    /// visibility defines the highest permission of object.
    #[prost(enumeration = "VisibilityType", tag = "10")]
    pub visibility: i32,
    /// content_type define the content type of the payload data
    #[prost(string, tag = "11")]
    pub content_type: ::prost::alloc::string::String,
    /// create_at define the block timestamp when the object created
    #[prost(int64, tag = "12")]
    pub create_at: i64,
    /// status define the status of the object. INIT or IN_SERVICE or others
    #[prost(enumeration = "ObjectStatus", tag = "13")]
    pub status: i32,
    /// redundancy_type define the type of redundancy. Replication or EC
    #[prost(enumeration = "RedundancyType", tag = "14")]
    pub redundancy_type: i32,
    /// source_type define the source of the object.  CrossChain or Greenfield origin
    #[prost(enumeration = "SourceType", tag = "15")]
    pub source_type: i32,
    /// checksums define the total checksums of the object which generated by redundancy
    #[prost(bytes = "vec", repeated, tag = "16")]
    pub checksums: ::prost::alloc::vec::Vec<::prost::alloc::vec::Vec<u8>>,
}
/// EventSealObject is emitted on MsgSealObject
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct EventCancelCreateObject {
    /// operator define the account address of operator who cancel create object
    #[prost(string, tag = "1")]
    pub operator: ::prost::alloc::string::String,
    /// bucket_name define the name of the bucket
    #[prost(string, tag = "2")]
    pub bucket_name: ::prost::alloc::string::String,
    /// object_name define the name of the object
    #[prost(string, tag = "3")]
    pub object_name: ::prost::alloc::string::String,
    /// primary_sp_address define the operator account address of the sp
    #[prost(string, tag = "4")]
    pub primary_sp_address: ::prost::alloc::string::String,
    /// id define an u256 id for object
    #[prost(string, tag = "6")]
    pub object_id: ::prost::alloc::string::String,
}
/// EventSealObject is emitted on MsgSealObject
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct EventSealObject {
    /// operator define the account address of operator who seal object
    #[prost(string, tag = "1")]
    pub operator: ::prost::alloc::string::String,
    /// bucket_name define the name of the bucket
    #[prost(string, tag = "2")]
    pub bucket_name: ::prost::alloc::string::String,
    /// object_name define the name of the object
    #[prost(string, tag = "3")]
    pub object_name: ::prost::alloc::string::String,
    /// id define an u256 id for object
    #[prost(string, tag = "5")]
    pub object_id: ::prost::alloc::string::String,
    /// status define the status of the object. INIT or IN_SERVICE or others
    #[prost(enumeration = "ObjectStatus", tag = "6")]
    pub status: i32,
    /// secondary_sp_address define all the operator address of the secondary sps
    #[prost(string, repeated, tag = "7")]
    pub secondary_sp_addresses: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
/// EventCopyObject is emitted on MsgCopyObject
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct EventCopyObject {
    /// operator define the account address of operator who copy the object
    #[prost(string, tag = "1")]
    pub operator: ::prost::alloc::string::String,
    /// src_bucket_name define the name of the src bucket
    #[prost(string, tag = "2")]
    pub src_bucket_name: ::prost::alloc::string::String,
    /// src_object_name define the name of the src object
    #[prost(string, tag = "3")]
    pub src_object_name: ::prost::alloc::string::String,
    /// dst_bucket_name define the name of the dst bucket
    #[prost(string, tag = "4")]
    pub dst_bucket_name: ::prost::alloc::string::String,
    /// dst_object_name define the name of the dst object
    #[prost(string, tag = "5")]
    pub dst_object_name: ::prost::alloc::string::String,
    /// src_object_id define the u256 id for src object
    #[prost(string, tag = "6")]
    pub src_object_id: ::prost::alloc::string::String,
    /// dst_object_id define the u256 id for dst object
    #[prost(string, tag = "7")]
    pub dst_object_id: ::prost::alloc::string::String,
}
/// EventDeleteObject is emitted on MsgDeleteObject
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct EventDeleteObject {
    /// operator define the account address of operator who delete the object
    #[prost(string, tag = "1")]
    pub operator: ::prost::alloc::string::String,
    /// bucket_name define the name of the bucket
    #[prost(string, tag = "2")]
    pub bucket_name: ::prost::alloc::string::String,
    /// object_name define the name of the object
    #[prost(string, tag = "3")]
    pub object_name: ::prost::alloc::string::String,
    /// id define an u256 id for object
    #[prost(string, tag = "4")]
    pub object_id: ::prost::alloc::string::String,
    /// primary_sp_address define the operator account address of the sp
    #[prost(string, tag = "5")]
    pub primary_sp_address: ::prost::alloc::string::String,
    /// secondary_sp_address define all the operator address of the secondary sps
    #[prost(string, repeated, tag = "6")]
    pub secondary_sp_addresses: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
/// EventRejectSealObject is emitted on MsgRejectSealObject
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct EventRejectSealObject {
    /// operator define the account address of operator who reject seal object
    #[prost(string, tag = "1")]
    pub operator: ::prost::alloc::string::String,
    /// bucket_name define the name of the bucket
    #[prost(string, tag = "2")]
    pub bucket_name: ::prost::alloc::string::String,
    /// object_name define the name of the object
    #[prost(string, tag = "3")]
    pub object_name: ::prost::alloc::string::String,
    /// id define an u256 id for object
    #[prost(string, tag = "4")]
    pub object_id: ::prost::alloc::string::String,
}
/// EventDiscontinueObject is emitted on MsgDiscontinueObject
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct EventDiscontinueObject {
    /// bucket_name define the name of the bucket
    #[prost(string, tag = "1")]
    pub bucket_name: ::prost::alloc::string::String,
    /// object_id defines id of the object
    #[prost(string, tag = "2")]
    pub object_id: ::prost::alloc::string::String,
    /// the reason
    #[prost(string, tag = "3")]
    pub reason: ::prost::alloc::string::String,
    /// the timestamp after which the metadata will be deleted
    #[prost(int64, tag = "4")]
    pub delete_at: i64,
}
/// EventUpdateObjectInfo is emitted on MsgUpdateObjectInfo
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct EventUpdateObjectInfo {
    /// operator define the account address of operator who update the bucket
    #[prost(string, tag = "1")]
    pub operator: ::prost::alloc::string::String,
    /// bucket_name define the name of the bucket
    #[prost(string, tag = "2")]
    pub bucket_name: ::prost::alloc::string::String,
    /// object_name define the name of the object
    #[prost(string, tag = "3")]
    pub object_name: ::prost::alloc::string::String,
    /// object_id define an u256 id for object
    #[prost(string, tag = "4")]
    pub object_id: ::prost::alloc::string::String,
    /// visibility defines the highest permission of object.
    #[prost(enumeration = "VisibilityType", tag = "5")]
    pub visibility: i32,
}
/// EventCreateGroup is emitted on MsgCreateGroup
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct EventCreateGroup {
    /// owner define the account address of group owner
    #[prost(string, tag = "1")]
    pub owner: ::prost::alloc::string::String,
    /// group_name define the name of the group
    #[prost(string, tag = "2")]
    pub group_name: ::prost::alloc::string::String,
    /// id define an u256 id for group
    #[prost(string, tag = "3")]
    pub group_id: ::prost::alloc::string::String,
    /// source_type define the source of the group. CrossChain or Greenfield origin
    #[prost(enumeration = "SourceType", tag = "4")]
    pub source_type: i32,
    /// members define the all the address of the members.
    #[prost(string, repeated, tag = "5")]
    pub members: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    /// extra defines extra info for the group
    #[prost(string, tag = "6")]
    pub extra: ::prost::alloc::string::String,
}
/// EventDeleteGroup is emitted on MsgDeleteGroup
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct EventDeleteGroup {
    /// owner define the account address of group owner
    #[prost(string, tag = "2")]
    pub owner: ::prost::alloc::string::String,
    /// group_name define the name of the group
    #[prost(string, tag = "3")]
    pub group_name: ::prost::alloc::string::String,
    /// id define an u256 id for group
    #[prost(string, tag = "4")]
    pub group_id: ::prost::alloc::string::String,
}
/// EventLeaveGroup is emitted on MsgLeaveGroup
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct EventLeaveGroup {
    /// member_address define the address of the member who leave the group
    #[prost(string, tag = "1")]
    pub member_address: ::prost::alloc::string::String,
    /// owner define the account address of group owner
    #[prost(string, tag = "2")]
    pub owner: ::prost::alloc::string::String,
    /// group_name define the name of the group
    #[prost(string, tag = "3")]
    pub group_name: ::prost::alloc::string::String,
    /// id define an u256 id for group
    #[prost(string, tag = "4")]
    pub group_id: ::prost::alloc::string::String,
}
/// EventUpdateGroupMember is emitted on MsgUpdateGroupMember
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct EventUpdateGroupMember {
    /// operator define the account address of operator who update the group member
    #[prost(string, tag = "1")]
    pub operator: ::prost::alloc::string::String,
    /// owner define the account address of group owner
    #[prost(string, tag = "2")]
    pub owner: ::prost::alloc::string::String,
    /// group_name define the name of the group
    #[prost(string, tag = "3")]
    pub group_name: ::prost::alloc::string::String,
    /// id define an u256 id for group
    #[prost(string, tag = "4")]
    pub group_id: ::prost::alloc::string::String,
    /// members_to_add defines all the members to be added to the group
    #[prost(string, repeated, tag = "5")]
    pub members_to_add: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    /// members_to_add defines all the members to be deleted from the group
    #[prost(string, repeated, tag = "6")]
    pub members_to_delete: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
/// EventUpdateGroupExtra is emitted on MsgUpdateGroupExtra
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct EventUpdateGroupExtra {
    /// operator define the account address of operator who update the group member
    #[prost(string, tag = "1")]
    pub operator: ::prost::alloc::string::String,
    /// owner define the account address of group owner
    #[prost(string, tag = "2")]
    pub owner: ::prost::alloc::string::String,
    /// group_name define the name of the group
    #[prost(string, tag = "3")]
    pub group_name: ::prost::alloc::string::String,
    /// id define an u256 id for group
    #[prost(string, tag = "4")]
    pub group_id: ::prost::alloc::string::String,
    /// extra defines extra info for the group to update
    #[prost(string, tag = "5")]
    pub extra: ::prost::alloc::string::String,
}
/// EventMirrorBucket is emitted on MirrorBucket
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct EventMirrorBucket {
    /// operator define the account address of operator who mirror the bucket
    #[prost(string, tag = "1")]
    pub operator: ::prost::alloc::string::String,
    /// bucket_name defines the name of the bucket
    #[prost(string, tag = "2")]
    pub bucket_name: ::prost::alloc::string::String,
    /// bucket_id define an u256 id for bucket
    #[prost(string, tag = "4")]
    pub bucket_id: ::prost::alloc::string::String,
}
/// EventMirrorBucketResult is emitted on receiving ack package from destination chain
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct EventMirrorBucketResult {
    /// status define the status of the result
    #[prost(uint32, tag = "1")]
    pub status: u32,
    /// bucket_name defines the name of the bucket
    #[prost(string, tag = "2")]
    pub bucket_name: ::prost::alloc::string::String,
    /// bucket_id define an u256 id for bucket
    #[prost(string, tag = "4")]
    pub bucket_id: ::prost::alloc::string::String,
}
/// EventMirrorObject is emitted on MirrorObject
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct EventMirrorObject {
    /// operator define the account address of operator who delete the object
    #[prost(string, tag = "1")]
    pub operator: ::prost::alloc::string::String,
    /// bucket_name define the name of the bucket
    #[prost(string, tag = "2")]
    pub bucket_name: ::prost::alloc::string::String,
    /// object_name define the name of the object
    #[prost(string, tag = "3")]
    pub object_name: ::prost::alloc::string::String,
    /// object_id define an u256 id for object
    #[prost(string, tag = "4")]
    pub object_id: ::prost::alloc::string::String,
}
/// EventMirrorObjectResult is emitted on receiving ack package from destination chain
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct EventMirrorObjectResult {
    /// status define the status of the result
    #[prost(uint32, tag = "1")]
    pub status: u32,
    /// bucket_name define the name of the bucket
    #[prost(string, tag = "2")]
    pub bucket_name: ::prost::alloc::string::String,
    /// object_name define the name of the object
    #[prost(string, tag = "3")]
    pub object_name: ::prost::alloc::string::String,
    /// object_id define an u256 id for object
    #[prost(string, tag = "4")]
    pub object_id: ::prost::alloc::string::String,
}
/// EventMirrorGroup is emitted on MirrorGroup
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct EventMirrorGroup {
    /// owner define the account address of group owner
    #[prost(string, tag = "1")]
    pub owner: ::prost::alloc::string::String,
    /// group_name define the name of the group
    #[prost(string, tag = "2")]
    pub group_name: ::prost::alloc::string::String,
    /// group_id define an u256 id for group
    #[prost(string, tag = "3")]
    pub group_id: ::prost::alloc::string::String,
}
/// EventMirrorGroupResult is emitted on receiving ack package from destination chain
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct EventMirrorGroupResult {
    /// status define the status of the result
    #[prost(uint32, tag = "1")]
    pub status: u32,
    /// group_name define the name of the group
    #[prost(string, tag = "2")]
    pub group_name: ::prost::alloc::string::String,
    /// group_id define an u256 id for group
    #[prost(string, tag = "3")]
    pub group_id: ::prost::alloc::string::String,
}
/// EventStalePolicyCleanup is emitted when specified block height's stale policies need to be Garbage collected
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct EventStalePolicyCleanup {
    #[prost(int64, tag = "1")]
    pub block_num: i64,
    #[prost(message, optional, tag = "2")]
    pub delete_info: ::core::option::Option<DeleteInfo>,
}
/// Params defines the parameters for the module.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Params {
    #[prost(message, optional, tag = "1")]
    pub versioned_params: ::core::option::Option<VersionedParams>,
    /// max_payload_size is the maximum size of the payload, default: 2G
    #[prost(uint64, tag = "2")]
    pub max_payload_size: u64,
    /// relayer fee for the mirror bucket tx
    #[prost(string, tag = "3")]
    pub mirror_bucket_relayer_fee: ::prost::alloc::string::String,
    /// relayer fee for the ACK or FAIL_ACK package of the mirror bucket tx
    #[prost(string, tag = "4")]
    pub mirror_bucket_ack_relayer_fee: ::prost::alloc::string::String,
    /// relayer fee for the mirror object tx
    #[prost(string, tag = "5")]
    pub mirror_object_relayer_fee: ::prost::alloc::string::String,
    /// Relayer fee for the ACK or FAIL_ACK package of the mirror object tx
    #[prost(string, tag = "6")]
    pub mirror_object_ack_relayer_fee: ::prost::alloc::string::String,
    /// relayer fee for the mirror object tx
    #[prost(string, tag = "7")]
    pub mirror_group_relayer_fee: ::prost::alloc::string::String,
    /// Relayer fee for the ACK or FAIL_ACK package of the mirror object tx
    #[prost(string, tag = "8")]
    pub mirror_group_ack_relayer_fee: ::prost::alloc::string::String,
    /// The maximum number of buckets that can be created per account
    #[prost(uint32, tag = "9")]
    pub max_buckets_per_account: u32,
    /// The window to count the discontinued objects or buckets
    #[prost(uint64, tag = "10")]
    pub discontinue_counting_window: u64,
    /// The max objects can be requested in a window
    #[prost(uint64, tag = "11")]
    pub discontinue_object_max: u64,
    /// The max buckets can be requested in a window
    #[prost(uint64, tag = "12")]
    pub discontinue_bucket_max: u64,
    /// The object will be deleted after the confirm period in seconds
    #[prost(int64, tag = "13")]
    pub discontinue_confirm_period: i64,
    /// The max delete objects in each end block
    #[prost(uint64, tag = "14")]
    pub discontinue_deletion_max: u64,
    /// The max number for deleting policy in each end block
    #[prost(uint64, tag = "15")]
    pub stale_policy_cleanup_max: u64,
}
/// VersionedParams defines the parameters for the storage module with multi version, each version store with different timestamp.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct VersionedParams {
    /// max_segment_size is the maximum size of a segment. default: 16M
    #[prost(uint64, tag = "1")]
    pub max_segment_size: u64,
    /// redundant_data_check_num is the num of data chunks of EC redundancy algorithm
    #[prost(uint32, tag = "2")]
    pub redundant_data_chunk_num: u32,
    /// redundant_data_check_num is the num of parity chunks of EC redundancy algorithm
    #[prost(uint32, tag = "3")]
    pub redundant_parity_chunk_num: u32,
    /// min_charge_size is the minimum charge size of the payload, objects smaller than this size will be charged as this size
    #[prost(uint64, tag = "4")]
    pub min_charge_size: u64,
}
/// GenesisState defines the bridge module's genesis state.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GenesisState {
    /// this line is used by starport scaffolding # genesis/proto/state
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
/// QueryVersionedParamsRequest is request type for the Query/Params RPC method with timestamp.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryParamsByTimestampRequest {
    /// the timestamp of the block time you want to query
    #[prost(int64, tag = "1")]
    pub timestamp: i64,
}
/// QueryVersionedParamsResponse is response type for the Query/Params RPC method with timestamp.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryParamsByTimestampResponse {
    /// params holds all the parameters of this module.
    #[prost(message, optional, tag = "1")]
    pub params: ::core::option::Option<Params>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryHeadBucketRequest {
    #[prost(string, tag = "1")]
    pub bucket_name: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryHeadBucketByIdRequest {
    #[prost(string, tag = "1")]
    pub bucket_id: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryHeadBucketResponse {
    #[prost(message, optional, tag = "1")]
    pub bucket_info: ::core::option::Option<BucketInfo>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryHeadObjectRequest {
    #[prost(string, tag = "1")]
    pub bucket_name: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub object_name: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryHeadObjectByIdRequest {
    #[prost(string, tag = "1")]
    pub object_id: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryHeadObjectResponse {
    #[prost(message, optional, tag = "1")]
    pub object_info: ::core::option::Option<ObjectInfo>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryListBucketsRequest {
    #[prost(message, optional, tag = "1")]
    pub pagination: ::core::option::Option<super::super::cosmos::base::query::v1beta1::PageRequest>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryListBucketsResponse {
    #[prost(message, repeated, tag = "1")]
    pub bucket_infos: ::prost::alloc::vec::Vec<BucketInfo>,
    #[prost(message, optional, tag = "2")]
    pub pagination:
        ::core::option::Option<super::super::cosmos::base::query::v1beta1::PageResponse>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryListObjectsRequest {
    #[prost(message, optional, tag = "1")]
    pub pagination: ::core::option::Option<super::super::cosmos::base::query::v1beta1::PageRequest>,
    #[prost(string, tag = "2")]
    pub bucket_name: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryListObjectsByBucketIdRequest {
    #[prost(message, optional, tag = "1")]
    pub pagination: ::core::option::Option<super::super::cosmos::base::query::v1beta1::PageRequest>,
    #[prost(string, tag = "2")]
    pub bucket_id: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryListObjectsResponse {
    #[prost(message, repeated, tag = "1")]
    pub object_infos: ::prost::alloc::vec::Vec<ObjectInfo>,
    #[prost(message, optional, tag = "2")]
    pub pagination:
        ::core::option::Option<super::super::cosmos::base::query::v1beta1::PageResponse>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryNftRequest {
    #[prost(string, tag = "1")]
    pub token_id: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryBucketNftResponse {
    #[prost(message, optional, tag = "1")]
    pub meta_data: ::core::option::Option<BucketMetaData>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryObjectNftResponse {
    #[prost(message, optional, tag = "1")]
    pub meta_data: ::core::option::Option<ObjectMetaData>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryGroupNftResponse {
    #[prost(message, optional, tag = "1")]
    pub meta_data: ::core::option::Option<GroupMetaData>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryPolicyForAccountRequest {
    #[prost(string, tag = "1")]
    pub resource: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub principal_address: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryPolicyForAccountResponse {
    #[prost(message, optional, tag = "1")]
    pub policy: ::core::option::Option<super::permission::Policy>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryVerifyPermissionRequest {
    #[prost(string, tag = "1")]
    pub operator: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub bucket_name: ::prost::alloc::string::String,
    #[prost(string, tag = "3")]
    pub object_name: ::prost::alloc::string::String,
    #[prost(enumeration = "super::permission::ActionType", tag = "4")]
    pub action_type: i32,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryVerifyPermissionResponse {
    #[prost(enumeration = "super::permission::Effect", tag = "1")]
    pub effect: i32,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryHeadGroupRequest {
    #[prost(string, tag = "1")]
    pub group_owner: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub group_name: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryHeadGroupResponse {
    #[prost(message, optional, tag = "1")]
    pub group_info: ::core::option::Option<GroupInfo>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryListGroupRequest {
    #[prost(message, optional, tag = "1")]
    pub pagination: ::core::option::Option<super::super::cosmos::base::query::v1beta1::PageRequest>,
    #[prost(string, tag = "2")]
    pub group_owner: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryListGroupResponse {
    #[prost(message, optional, tag = "1")]
    pub pagination:
        ::core::option::Option<super::super::cosmos::base::query::v1beta1::PageResponse>,
    #[prost(message, repeated, tag = "2")]
    pub group_infos: ::prost::alloc::vec::Vec<GroupInfo>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryHeadGroupMemberRequest {
    #[prost(string, tag = "1")]
    pub member: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub group_owner: ::prost::alloc::string::String,
    #[prost(string, tag = "3")]
    pub group_name: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryHeadGroupMemberResponse {
    #[prost(message, optional, tag = "1")]
    pub group_member: ::core::option::Option<super::permission::GroupMember>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryPolicyForGroupRequest {
    #[prost(string, tag = "1")]
    pub resource: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub principal_group_id: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryPolicyForGroupResponse {
    #[prost(message, optional, tag = "1")]
    pub policy: ::core::option::Option<super::permission::Policy>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryPolicyByIdRequest {
    #[prost(string, tag = "1")]
    pub policy_id: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryPolicyByIdResponse {
    #[prost(message, optional, tag = "1")]
    pub policy: ::core::option::Option<super::permission::Policy>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgCreateBucket {
    /// creator defines the account address of bucket creator, it is also the bucket owner.
    #[prost(string, tag = "1")]
    pub creator: ::prost::alloc::string::String,
    /// bucket_name defines a globally unique name of bucket
    #[prost(string, tag = "2")]
    pub bucket_name: ::prost::alloc::string::String,
    /// visibility means the bucket is private or public. if private, only bucket owner or grantee can read it,
    /// otherwise every greenfield user can read it.
    #[prost(enumeration = "VisibilityType", tag = "3")]
    pub visibility: i32,
    /// payment_address defines an account address specified by bucket owner to pay the read fee. Default: creator
    #[prost(string, tag = "4")]
    pub payment_address: ::prost::alloc::string::String,
    /// primary_sp_address defines the address of primary sp.
    #[prost(string, tag = "6")]
    pub primary_sp_address: ::prost::alloc::string::String,
    /// primary_sp_approval defines the approval info of the primary SP which indicates that primary sp confirm the user's request.
    #[prost(message, optional, tag = "7")]
    pub primary_sp_approval: ::core::option::Option<Approval>,
    /// charged_read_quota defines the read data that users are charged for, measured in bytes.
    /// The available read data for each user is the sum of the free read data provided by SP and
    /// the ChargeReadQuota specified here.
    #[prost(uint64, tag = "8")]
    pub charged_read_quota: u64,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgCreateBucketResponse {
    #[prost(string, tag = "1")]
    pub bucket_id: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgDeleteBucket {
    /// creator defines the account address of the grantee who has the DeleteBucket permission of the bucket to be deleted.
    #[prost(string, tag = "1")]
    pub operator: ::prost::alloc::string::String,
    /// bucket_name defines the name of the bucket to be deleted.
    #[prost(string, tag = "2")]
    pub bucket_name: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgDeleteBucketResponse {}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgDiscontinueBucket {
    /// operator is the sp who wants to stop serving the bucket.
    #[prost(string, tag = "1")]
    pub operator: ::prost::alloc::string::String,
    /// bucket_name defines the name of the bucket where the object which to be discontinued is stored.
    #[prost(string, tag = "2")]
    pub bucket_name: ::prost::alloc::string::String,
    /// the reason for the request.
    #[prost(string, tag = "3")]
    pub reason: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgDiscontinueBucketResponse {}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgCreateObject {
    /// creator defines the account address of object uploader
    #[prost(string, tag = "1")]
    pub creator: ::prost::alloc::string::String,
    /// bucket_name defines the name of the bucket where the object is stored.
    #[prost(string, tag = "2")]
    pub bucket_name: ::prost::alloc::string::String,
    /// object_name defines the name of object
    #[prost(string, tag = "3")]
    pub object_name: ::prost::alloc::string::String,
    /// payload_size defines size of the object's payload
    #[prost(uint64, tag = "4")]
    pub payload_size: u64,
    /// visibility means the object is private or public. if private, only object owner or grantee can access it,
    /// otherwise every greenfield user can access it.
    #[prost(enumeration = "VisibilityType", tag = "5")]
    pub visibility: i32,
    /// content_type defines a standard MIME type describing the format of the object.
    #[prost(string, tag = "6")]
    pub content_type: ::prost::alloc::string::String,
    /// primary_sp_approval defines the approval info of the primary SP which indicates that primary sp confirm the user's request.
    #[prost(message, optional, tag = "7")]
    pub primary_sp_approval: ::core::option::Option<Approval>,
    /// expect_checksums defines a list of hashes which was generate by redundancy algorithm.
    #[prost(bytes = "vec", repeated, tag = "8")]
    pub expect_checksums: ::prost::alloc::vec::Vec<::prost::alloc::vec::Vec<u8>>,
    /// redundancy_type can be ec or replica
    #[prost(enumeration = "RedundancyType", tag = "9")]
    pub redundancy_type: i32,
    /// expect_secondarySPs defines a list of StorageProvider address, which is optional
    #[prost(string, repeated, tag = "10")]
    pub expect_secondary_sp_addresses: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgCreateObjectResponse {
    #[prost(string, tag = "1")]
    pub object_id: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgSealObject {
    /// operator defines the account address of primary SP
    #[prost(string, tag = "1")]
    pub operator: ::prost::alloc::string::String,
    /// bucket_name defines the name of the bucket where the object is stored.
    #[prost(string, tag = "2")]
    pub bucket_name: ::prost::alloc::string::String,
    /// object_name defines the name of object to be sealed.
    #[prost(string, tag = "3")]
    pub object_name: ::prost::alloc::string::String,
    /// secondary_sp_addresses defines a list of storage provider which store the redundant data.
    #[prost(string, repeated, tag = "4")]
    pub secondary_sp_addresses: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    /// secondary_sp_signatures defines the signature of the secondary sp that can
    /// acknowledge that the payload data has received and stored.
    #[prost(bytes = "vec", repeated, tag = "5")]
    pub secondary_sp_signatures: ::prost::alloc::vec::Vec<::prost::alloc::vec::Vec<u8>>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgSealObjectResponse {}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgRejectSealObject {
    /// operator defines the account address of the object owner
    #[prost(string, tag = "1")]
    pub operator: ::prost::alloc::string::String,
    /// bucket_name defines the name of the bucket where the object is stored.
    #[prost(string, tag = "2")]
    pub bucket_name: ::prost::alloc::string::String,
    /// object_name defines the name of unsealed object to be reject.
    #[prost(string, tag = "3")]
    pub object_name: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgRejectSealObjectResponse {}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgCopyObject {
    /// operator defines the account address of the operator who has the CopyObject permission of the object to be deleted.
    #[prost(string, tag = "1")]
    pub operator: ::prost::alloc::string::String,
    /// src_bucket_name defines the name of the bucket where the object to be copied is located
    #[prost(string, tag = "2")]
    pub src_bucket_name: ::prost::alloc::string::String,
    /// dst_bucket_name defines the name of the bucket where the object is copied to.
    #[prost(string, tag = "3")]
    pub dst_bucket_name: ::prost::alloc::string::String,
    /// src_object_name defines the name of the object which to be copied
    #[prost(string, tag = "4")]
    pub src_object_name: ::prost::alloc::string::String,
    /// dst_object_name defines the name of the object which is copied to
    #[prost(string, tag = "5")]
    pub dst_object_name: ::prost::alloc::string::String,
    /// primary_sp_approval defines the approval info of the primary SP which indicates that primary sp confirm the user's request.
    #[prost(message, optional, tag = "6")]
    pub dst_primary_sp_approval: ::core::option::Option<Approval>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgCopyObjectResponse {
    #[prost(string, tag = "1")]
    pub object_id: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgDeleteObject {
    /// operator defines the account address of the operator who has the DeleteObject permission of the object to be deleted.
    #[prost(string, tag = "1")]
    pub operator: ::prost::alloc::string::String,
    /// bucket_name defines the name of the bucket where the object which to be deleted is stored.
    #[prost(string, tag = "2")]
    pub bucket_name: ::prost::alloc::string::String,
    /// object_name defines the name of the object which to be deleted.
    #[prost(string, tag = "3")]
    pub object_name: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgDeleteObjectResponse {}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgDiscontinueObject {
    /// operator is the sp who wants to stop serving the objects.
    #[prost(string, tag = "1")]
    pub operator: ::prost::alloc::string::String,
    /// bucket_name defines the name of the bucket where the object which to be discontinued is stored.
    #[prost(string, tag = "2")]
    pub bucket_name: ::prost::alloc::string::String,
    /// object_ids are the ids of object info.
    #[prost(string, repeated, tag = "3")]
    pub object_ids: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    /// the reason for the request.
    #[prost(string, tag = "4")]
    pub reason: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgDiscontinueObjectResponse {}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgCreateGroup {
    /// owner defines the account address of group owner who create the group
    #[prost(string, tag = "1")]
    pub creator: ::prost::alloc::string::String,
    /// group_name defines the name of the group. it's not globally unique.
    #[prost(string, tag = "2")]
    pub group_name: ::prost::alloc::string::String,
    /// member_request defines a list of member which to be add or remove
    #[prost(string, repeated, tag = "3")]
    pub members: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    /// extra defines extra info for the group
    #[prost(string, tag = "4")]
    pub extra: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgCreateGroupResponse {
    #[prost(string, tag = "1")]
    pub group_id: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgDeleteGroup {
    /// operator defines the account address of the operator who has the DeleteGroup permission of the group to be deleted.
    #[prost(string, tag = "1")]
    pub operator: ::prost::alloc::string::String,
    /// group_name defines the name of the group which to be deleted
    #[prost(string, tag = "2")]
    pub group_name: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgDeleteGroupResponse {}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgUpdateGroupMember {
    /// operator defines the account address of the operator who has the UpdateGroupMember permission of the group.
    #[prost(string, tag = "1")]
    pub operator: ::prost::alloc::string::String,
    /// group_owner defines the account address of the group owner
    #[prost(string, tag = "2")]
    pub group_owner: ::prost::alloc::string::String,
    /// group_name defines the name of the group which to be updated
    #[prost(string, tag = "3")]
    pub group_name: ::prost::alloc::string::String,
    /// members_to_add defines a list of members account address which will be add to the group
    #[prost(string, repeated, tag = "4")]
    pub members_to_add: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    /// members_to_delete defines a list of members account address which will be remove from the group
    #[prost(string, repeated, tag = "5")]
    pub members_to_delete: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgUpdateGroupMemberResponse {}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgUpdateGroupExtra {
    /// operator defines the account address of the operator who has the UpdateGroupMember permission of the group.
    #[prost(string, tag = "1")]
    pub operator: ::prost::alloc::string::String,
    /// group_owner defines the account address of the group owner
    #[prost(string, tag = "2")]
    pub group_owner: ::prost::alloc::string::String,
    /// group_name defines the name of the group which to be updated
    #[prost(string, tag = "3")]
    pub group_name: ::prost::alloc::string::String,
    /// extra defines extra info for the group to update
    #[prost(string, tag = "4")]
    pub extra: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgUpdateGroupExtraResponse {}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgLeaveGroup {
    /// member defines the account address of the member who want to leave the group
    #[prost(string, tag = "1")]
    pub member: ::prost::alloc::string::String,
    /// group_owner defines the owner of the group you want to leave
    #[prost(string, tag = "2")]
    pub group_owner: ::prost::alloc::string::String,
    /// group_name defines the name of the group you want to leave
    #[prost(string, tag = "3")]
    pub group_name: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgLeaveGroupResponse {}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgUpdateBucketInfo {
    /// operator defines the account address of the operator
    #[prost(string, tag = "1")]
    pub operator: ::prost::alloc::string::String,
    /// bucket_name defines the name of bucket which you'll update
    #[prost(string, tag = "2")]
    pub bucket_name: ::prost::alloc::string::String,
    /// charged_read_quota defines the traffic quota that you read from primary sp
    /// if read_quota is nil, it means don't change the read_quota
    #[prost(message, optional, tag = "3")]
    pub charged_read_quota: ::core::option::Option<super::common::UInt64Value>,
    /// payment_address defines the account address of the payment account
    /// if payment_address is empty, it means don't change the payment_address
    #[prost(string, tag = "4")]
    pub payment_address: ::prost::alloc::string::String,
    /// visibility means the bucket is private or public. if private, only bucket owner or grantee can read it,
    /// otherwise every greenfield user can read it.
    #[prost(enumeration = "VisibilityType", tag = "5")]
    pub visibility: i32,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgUpdateBucketInfoResponse {}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgCancelCreateObject {
    /// operator defines the account address of the operator
    #[prost(string, tag = "1")]
    pub operator: ::prost::alloc::string::String,
    /// bucket_name defines the name of the bucket
    #[prost(string, tag = "2")]
    pub bucket_name: ::prost::alloc::string::String,
    /// object_name defines the name of the object
    #[prost(string, tag = "3")]
    pub object_name: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgCancelCreateObjectResponse {}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgPutPolicy {
    /// operator defines the granter who grant the permission to another principal
    #[prost(string, tag = "1")]
    pub operator: ::prost::alloc::string::String,
    /// Principal defines the roles that can grant permissions. Currently, it can be account or group.
    #[prost(message, optional, tag = "2")]
    pub principal: ::core::option::Option<super::permission::Principal>,
    /// resource defines a greenfield standard resource name that can be generated by GRN structure
    #[prost(string, tag = "3")]
    pub resource: ::prost::alloc::string::String,
    /// statements defines a list of individual statement which describe the detail rules of policy
    #[prost(message, repeated, tag = "4")]
    pub statements: ::prost::alloc::vec::Vec<super::permission::Statement>,
    /// expiration_time defines the whole expiration time of all the statements.
    /// Notices: Its priority is higher than the expiration time inside the Statement
    #[prost(message, optional, tag = "7")]
    pub expiration_time: ::core::option::Option<::prost_types::Timestamp>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgPutPolicyResponse {
    #[prost(string, tag = "4")]
    pub policy_id: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgDeletePolicy {
    /// operator defines the granter who grant the permission to another principal
    #[prost(string, tag = "1")]
    pub operator: ::prost::alloc::string::String,
    /// Principal defines the roles that can grant permissions. Currently, it can be account or group.
    #[prost(message, optional, tag = "2")]
    pub principal: ::core::option::Option<super::permission::Principal>,
    /// resource defines a greenfield standard resource name that can be generated by GRN structure
    #[prost(string, tag = "3")]
    pub resource: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgDeletePolicyResponse {
    #[prost(string, tag = "4")]
    pub policy_id: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgMirrorObject {
    /// operator defines the account address of the operator who has the DeleteObject permission of the object to be deleted.
    #[prost(string, tag = "1")]
    pub operator: ::prost::alloc::string::String,
    /// id defines the unique u256 for object.
    #[prost(string, tag = "2")]
    pub id: ::prost::alloc::string::String,
    /// bucket_name defines the name of the bucket where the object is stored
    #[prost(string, tag = "3")]
    pub bucket_name: ::prost::alloc::string::String,
    /// object_name defines the name of object
    #[prost(string, tag = "4")]
    pub object_name: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgMirrorObjectResponse {}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgMirrorBucket {
    /// creator defines the account address of the grantee who has the DeleteBucket permission of the bucket to be deleted.
    #[prost(string, tag = "1")]
    pub operator: ::prost::alloc::string::String,
    /// id defines the unique u256 for bucket.
    #[prost(string, tag = "2")]
    pub id: ::prost::alloc::string::String,
    /// bucket_name defines a globally unique name of bucket
    #[prost(string, tag = "3")]
    pub bucket_name: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgUpdateObjectInfoResponse {}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgUpdateObjectInfo {
    /// operator defines the account address of the operator
    #[prost(string, tag = "1")]
    pub operator: ::prost::alloc::string::String,
    /// bucket_name is the name of the bucket
    #[prost(string, tag = "2")]
    pub bucket_name: ::prost::alloc::string::String,
    /// object_name defines the name of bucket which you'll update
    #[prost(string, tag = "3")]
    pub object_name: ::prost::alloc::string::String,
    /// visibility means the object is private or public. if private, only bucket owner or grantee can read it,
    /// otherwise every greenfield user can read it.
    #[prost(enumeration = "VisibilityType", tag = "4")]
    pub visibility: i32,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgMirrorBucketResponse {}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgMirrorGroup {
    /// operator defines the account address of the operator who is the owner of the group
    #[prost(string, tag = "1")]
    pub operator: ::prost::alloc::string::String,
    /// id defines the unique u256 for group.
    #[prost(string, tag = "2")]
    pub id: ::prost::alloc::string::String,
    /// group_name defines the name of the group
    #[prost(string, tag = "3")]
    pub group_name: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgMirrorGroupResponse {}
/// MsgUpdateParams is the Msg/UpdateParams request type.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgUpdateParams {
    /// authority is the address that controls the module (defaults to x/gov unless overwritten).
    #[prost(string, tag = "1")]
    pub authority: ::prost::alloc::string::String,
    /// params defines the x/storage parameters to update.
    ///
    /// NOTE: All parameters must be supplied.
    #[prost(message, optional, tag = "2")]
    pub params: ::core::option::Option<Params>,
}
/// MsgUpdateParamsResponse defines the response structure for executing a
/// MsgUpdateParams message.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgUpdateParamsResponse {}
include!("greenfield.storage.tonic.rs");
// @@protoc_insertion_point(module)
