// @generated
/// DepositAuthorization defines authorization for sp deposit.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DepositAuthorization {
    /// max_deposit specifies the maximum amount of tokens can be deposit to a storage provider. If it is
    /// empty, there is no spend limit and any amount of coins can be deposit.
    #[prost(message, optional, tag = "1")]
    pub max_deposit: ::core::option::Option<super::super::cosmos::base::v1beta1::Coin>,
    /// sp_address is the operator address of storage provider.
    #[prost(string, tag = "2")]
    pub sp_address: ::prost::alloc::string::String,
}
/// Description defines a storage provider description.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Description {
    /// moniker defines a human-readable name for the storage provider
    #[prost(string, tag = "1")]
    pub moniker: ::prost::alloc::string::String,
    /// identity defines an optional identity signature (ex. UPort or Keybase).
    #[prost(string, tag = "2")]
    pub identity: ::prost::alloc::string::String,
    /// website defines an optional website link.
    #[prost(string, tag = "3")]
    pub website: ::prost::alloc::string::String,
    /// security_contact defines an optional email for security contact.
    #[prost(string, tag = "4")]
    pub security_contact: ::prost::alloc::string::String,
    /// details define other optional details.
    #[prost(string, tag = "5")]
    pub details: ::prost::alloc::string::String,
}
/// StorageProvider defines the meta info of storage provider
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct StorageProvider {
    /// operator_address defines the account address of the storage provider's operator; It also is the unique index key of sp.
    #[prost(string, tag = "1")]
    pub operator_address: ::prost::alloc::string::String,
    /// funding_address defines one of the storage provider's accounts which is used to deposit and reward.
    #[prost(string, tag = "2")]
    pub funding_address: ::prost::alloc::string::String,
    /// seal_address defines one of the storage provider's accounts which is used to SealObject
    #[prost(string, tag = "3")]
    pub seal_address: ::prost::alloc::string::String,
    /// approval_address defines one of the storage provider's accounts which is used to approve use's createBucket/createObject request
    #[prost(string, tag = "4")]
    pub approval_address: ::prost::alloc::string::String,
    /// gc_address defines one of the storage provider's accounts which is used for gc purpose.
    #[prost(string, tag = "5")]
    pub gc_address: ::prost::alloc::string::String,
    /// total_deposit defines the number of tokens deposited by this storage provider for staking.
    #[prost(string, tag = "6")]
    pub total_deposit: ::prost::alloc::string::String,
    /// status defines the current service status of this storage provider
    #[prost(enumeration = "Status", tag = "7")]
    pub status: i32,
    /// endpoint define the storage provider's network service address
    #[prost(string, tag = "8")]
    pub endpoint: ::prost::alloc::string::String,
    /// description defines the description terms for the storage provider.
    #[prost(message, optional, tag = "9")]
    pub description: ::core::option::Option<Description>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RewardInfo {
    #[prost(string, tag = "1")]
    pub address: ::prost::alloc::string::String,
    #[prost(message, optional, tag = "2")]
    pub amount: ::core::option::Option<super::super::cosmos::base::v1beta1::Coin>,
}
/// storage price of a specific sp
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SpStoragePrice {
    /// sp address
    #[prost(string, tag = "1")]
    pub sp_address: ::prost::alloc::string::String,
    /// update time, unix timestamp in seconds
    #[prost(int64, tag = "2")]
    pub update_time_sec: i64,
    /// read price, in bnb wei per charge byte
    #[prost(string, tag = "3")]
    pub read_price: ::prost::alloc::string::String,
    /// free read quota, in byte
    #[prost(uint64, tag = "4")]
    pub free_read_quota: u64,
    /// store price, in bnb wei per charge byte
    #[prost(string, tag = "5")]
    pub store_price: ::prost::alloc::string::String,
}
/// global secondary sp store price, the price for all secondary sps
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SecondarySpStorePrice {
    /// update time, unix timestamp in seconds
    #[prost(int64, tag = "1")]
    pub update_time_sec: i64,
    /// store price, in bnb wei per charge byte
    #[prost(string, tag = "2")]
    pub store_price: ::prost::alloc::string::String,
}
/// Status is the status of a storage provider.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum Status {
    InService = 0,
    InJailed = 1,
    GracefulExiting = 2,
    OutOfService = 3,
}
impl Status {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            Status::InService => "STATUS_IN_SERVICE",
            Status::InJailed => "STATUS_IN_JAILED",
            Status::GracefulExiting => "STATUS_GRACEFUL_EXITING",
            Status::OutOfService => "STATUS_OUT_OF_SERVICE",
        }
    }
}
/// EventCreateStorageProvider is emitted when there is a storage provider created
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct EventCreateStorageProvider {
    /// sp_address is the operator address of the storage provider
    #[prost(string, tag = "1")]
    pub sp_address: ::prost::alloc::string::String,
    /// funding_address is the funding account address of the storage provider
    #[prost(string, tag = "2")]
    pub funding_address: ::prost::alloc::string::String,
    /// seal_address is the account address for SealObject Tx
    #[prost(string, tag = "3")]
    pub seal_address: ::prost::alloc::string::String,
    /// approval_address is the account address for approve create bucket/object signature
    #[prost(string, tag = "4")]
    pub approval_address: ::prost::alloc::string::String,
    /// gc_address defines one of the storage provider's accounts which is used for gc purpose
    #[prost(string, tag = "5")]
    pub gc_address: ::prost::alloc::string::String,
    /// endpoint is the domain name address used by SP to provide storage services
    #[prost(string, tag = "6")]
    pub endpoint: ::prost::alloc::string::String,
    /// total_deposit is the token coin that the storage provider deposit to the storage module
    #[prost(message, optional, tag = "7")]
    pub total_deposit: ::core::option::Option<super::super::cosmos::base::v1beta1::Coin>,
    /// status defines the status of the storage provider
    #[prost(enumeration = "Status", tag = "8")]
    pub status: i32,
    /// description defines the description terms for the storage provider
    #[prost(message, optional, tag = "9")]
    pub description: ::core::option::Option<Description>,
}
/// EventEditStorageProvider is emitted when SP's metadata is edited.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct EventEditStorageProvider {
    /// sp_address is the operator address of the storage provider
    #[prost(string, tag = "1")]
    pub sp_address: ::prost::alloc::string::String,
    /// endpoint is the service endpoint of the storage provider
    #[prost(string, tag = "2")]
    pub endpoint: ::prost::alloc::string::String,
    /// description defines the description terms for the storage provider
    #[prost(message, optional, tag = "3")]
    pub description: ::core::option::Option<Description>,
    /// seal_address is the account address for SealObject Tx
    #[prost(string, tag = "4")]
    pub seal_address: ::prost::alloc::string::String,
    /// approval_address is the account address for approve create bucket/object signature
    #[prost(string, tag = "5")]
    pub approval_address: ::prost::alloc::string::String,
    /// gc_address defines one of the storage provider's accounts which is used for gc purpose
    #[prost(string, tag = "6")]
    pub gc_address: ::prost::alloc::string::String,
}
/// EventDeposit is emitted when sp deposit tokens.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct EventDeposit {
    /// funding_address is the funding account address of the storage provider
    #[prost(string, tag = "1")]
    pub funding_address: ::prost::alloc::string::String,
    /// deposit is the token coin deposited this message
    #[prost(string, tag = "2")]
    pub deposit: ::prost::alloc::string::String,
    /// total_deposit is the total token coins this storage provider deposited
    #[prost(string, tag = "3")]
    pub total_deposit: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct EventSpStoragePriceUpdate {
    /// sp address
    #[prost(string, tag = "1")]
    pub sp_address: ::prost::alloc::string::String,
    /// update time, in unix timestamp
    #[prost(int64, tag = "2")]
    pub update_time_sec: i64,
    /// read price, in bnb wei per charge byte
    #[prost(string, tag = "3")]
    pub read_price: ::prost::alloc::string::String,
    /// free read quota, in byte
    #[prost(uint64, tag = "4")]
    pub free_read_quota: u64,
    /// store price, in bnb wei per charge byte
    #[prost(string, tag = "5")]
    pub store_price: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct EventSecondarySpStorePriceUpdate {
    /// update time, in unix timestamp
    #[prost(int64, tag = "1")]
    pub update_time_sec: i64,
    /// store price, in bnb wei per charge byte
    #[prost(string, tag = "2")]
    pub store_price: ::prost::alloc::string::String,
}
/// Params defines the parameters for the module.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Params {
    /// deposit_denom defines the staking coin denomination.
    #[prost(string, tag = "1")]
    pub deposit_denom: ::prost::alloc::string::String,
    /// min_deposit defines the minimum deposit amount for storage providers.
    #[prost(string, tag = "2")]
    pub min_deposit: ::prost::alloc::string::String,
    /// the ratio of the store price of the secondary sp to the primary sp, the default value is 80%
    #[prost(string, tag = "3")]
    pub secondary_sp_store_price_ratio: ::prost::alloc::string::String,
}
/// GenesisState defines the sp module's genesis state.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GenesisState {
    #[prost(message, optional, tag = "1")]
    pub params: ::core::option::Option<Params>,
    /// this used by starport scaffolding # genesis/proto/state
    #[prost(message, repeated, tag = "2")]
    pub storage_providers: ::prost::alloc::vec::Vec<StorageProvider>,
    #[prost(message, repeated, tag = "3")]
    pub sp_storage_price_list: ::prost::alloc::vec::Vec<SpStoragePrice>,
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
/// this line is used by starport scaffolding # 3
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryStorageProvidersRequest {
    /// pagination defines an optional pagination for the request.
    #[prost(message, optional, tag = "1")]
    pub pagination: ::core::option::Option<super::super::cosmos::base::query::v1beta1::PageRequest>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryStorageProvidersResponse {
    #[prost(message, repeated, tag = "1")]
    pub sps: ::prost::alloc::vec::Vec<StorageProvider>,
    /// pagination defines the pagination in the response.
    #[prost(message, optional, tag = "2")]
    pub pagination:
        ::core::option::Option<super::super::cosmos::base::query::v1beta1::PageResponse>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryGetSpStoragePriceByTimeRequest {
    /// operator address of sp
    #[prost(string, tag = "1")]
    pub sp_addr: ::prost::alloc::string::String,
    /// unix timestamp in seconds. If it's 0, it will return the latest price.
    #[prost(int64, tag = "2")]
    pub timestamp: i64,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryGetSpStoragePriceByTimeResponse {
    #[prost(message, optional, tag = "1")]
    pub sp_storage_price: ::core::option::Option<SpStoragePrice>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryGetSecondarySpStorePriceByTimeRequest {
    /// unix timestamp in seconds. If it's 0, it will return the latest price.
    #[prost(int64, tag = "1")]
    pub timestamp: i64,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryGetSecondarySpStorePriceByTimeResponse {
    #[prost(message, optional, tag = "1")]
    pub secondary_sp_store_price: ::core::option::Option<SecondarySpStorePrice>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryStorageProviderRequest {
    #[prost(string, tag = "1")]
    pub sp_address: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryStorageProviderResponse {
    #[prost(message, optional, tag = "1")]
    pub storage_provider: ::core::option::Option<StorageProvider>,
}
/// MsgCreateStorageProvider defines message for creating a new storage provider.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgCreateStorageProvider {
    /// creator is the msg signer
    #[prost(string, tag = "1")]
    pub creator: ::prost::alloc::string::String,
    /// description defines the description terms for the validator.
    #[prost(message, optional, tag = "2")]
    pub description: ::core::option::Option<Description>,
    /// sp_address defines the address of the sp's operator; It also is the unqiue index key of sp.
    #[prost(string, tag = "3")]
    pub sp_address: ::prost::alloc::string::String,
    /// fund_address is the account address of the storage provider for deposit, remuneration.
    #[prost(string, tag = "4")]
    pub funding_address: ::prost::alloc::string::String,
    /// seal_address is the account address of the storage provider for sealObject
    #[prost(string, tag = "5")]
    pub seal_address: ::prost::alloc::string::String,
    /// approval_address is the account address of the storage provider for ack CreateBucket/Object.
    #[prost(string, tag = "6")]
    pub approval_address: ::prost::alloc::string::String,
    /// gc_address defines one of the storage provider's accounts which is used for gc purpose.
    #[prost(string, tag = "7")]
    pub gc_address: ::prost::alloc::string::String,
    /// endpoint is the service address of the storage provider
    #[prost(string, tag = "8")]
    pub endpoint: ::prost::alloc::string::String,
    /// deposit define the deposit token
    #[prost(message, optional, tag = "9")]
    pub deposit: ::core::option::Option<super::super::cosmos::base::v1beta1::Coin>,
    /// read price, in bnb wei per charge byte
    #[prost(string, tag = "10")]
    pub read_price: ::prost::alloc::string::String,
    /// free read quota, in byte
    #[prost(uint64, tag = "11")]
    pub free_read_quota: u64,
    /// store price, in bnb wei per charge byte
    #[prost(string, tag = "12")]
    pub store_price: ::prost::alloc::string::String,
}
/// MsgCreateStorageProviderResponse defines the Msg/CreateStorageProvider response type.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgCreateStorageProviderResponse {}
/// MsgDeposit defines a SDK message for deposit token for sp.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgDeposit {
    /// creator is the msg signer, it should be sp's fund address
    #[prost(string, tag = "1")]
    pub creator: ::prost::alloc::string::String,
    /// sp_address is the operator address of sp
    #[prost(string, tag = "2")]
    pub sp_address: ::prost::alloc::string::String,
    /// deposit is a mount of token which used to deposit for SP
    #[prost(message, optional, tag = "3")]
    pub deposit: ::core::option::Option<super::super::cosmos::base::v1beta1::Coin>,
}
/// MsgDepositResponse defines the Msg/Deposit response type.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgDepositResponse {}
/// MsgEditStorageProvider defines a SDK message for editing an existing sp.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgEditStorageProvider {
    #[prost(string, tag = "1")]
    pub sp_address: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub endpoint: ::prost::alloc::string::String,
    #[prost(message, optional, tag = "3")]
    pub description: ::core::option::Option<Description>,
    /// seal_address is the account address of the storage provider for sealObject
    #[prost(string, tag = "4")]
    pub seal_address: ::prost::alloc::string::String,
    /// approval_address is the account address of the storage provider for ack CreateBucket/Object
    #[prost(string, tag = "5")]
    pub approval_address: ::prost::alloc::string::String,
    /// gc_address defines one of the storage provider's accounts which is used for gc purpose
    #[prost(string, tag = "6")]
    pub gc_address: ::prost::alloc::string::String,
}
/// MsgEditStorageProviderResponse defines the Msg/EditStorageProvider response type.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgEditStorageProviderResponse {}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgUpdateSpStoragePrice {
    /// sp address
    #[prost(string, tag = "1")]
    pub sp_address: ::prost::alloc::string::String,
    /// read price, in bnb wei per charge byte
    #[prost(string, tag = "2")]
    pub read_price: ::prost::alloc::string::String,
    /// free read quota, in byte
    #[prost(uint64, tag = "3")]
    pub free_read_quota: u64,
    /// store price, in bnb wei per charge byte
    #[prost(string, tag = "4")]
    pub store_price: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgUpdateSpStoragePriceResponse {}
/// MsgUpdateParams is the Msg/UpdateParams request type.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgUpdateParams {
    /// authority is the address that controls the module (defaults to x/gov unless overwritten).
    #[prost(string, tag = "1")]
    pub authority: ::prost::alloc::string::String,
    /// params defines the x/sp parameters to update.
    ///
    /// NOTE: All parameters must be supplied.
    #[prost(message, optional, tag = "2")]
    pub params: ::core::option::Option<Params>,
}
/// MsgUpdateParamsResponse defines the response structure for executing a
/// MsgUpdateParams message.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgUpdateParamsResponse {}
include!("greenfield.sp.tonic.rs");
// @@protoc_insertion_point(module)
