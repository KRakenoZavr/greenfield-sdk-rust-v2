// @generated
/// AutoSettleRecord is the record keeps the auto settle information.
/// The EndBlocker of payment module will scan the list of AutoSettleRecord
/// and settle the stream account if the timestamp is less than the current time.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AutoSettleRecord {
    /// timestamp is the unix timestamp when the stream account will be settled.
    #[prost(int64, tag = "1")]
    pub timestamp: i64,
    /// A stream account address
    #[prost(string, tag = "2")]
    pub addr: ::prost::alloc::string::String,
}
/// OutFlow defines the accumulative outflow stream rate in BNB
/// from a stream account to a Storage Provider
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct OutFlow {
    /// stream account address who receives the flow, usually SP(service provider)
    #[prost(string, tag = "1")]
    pub to_address: ::prost::alloc::string::String,
    /// flow rate
    #[prost(string, tag = "2")]
    pub rate: ::prost::alloc::string::String,
}
/// StreamAccountStatus defines the status of a stream account
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum StreamAccountStatus {
    /// STREAM_ACCOUNT_STATUS_ACTIVE defines the active status of a stream account.
    Active = 0,
    /// STREAM_ACCOUNT_STATUS_FROZEN defines the frozen status of a stream account.
    /// A frozen stream account cannot be used as payment address for buckets.
    /// It can be unfrozen by depositing more BNB to the stream account.
    Frozen = 1,
}
impl StreamAccountStatus {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            StreamAccountStatus::Active => "STREAM_ACCOUNT_STATUS_ACTIVE",
            StreamAccountStatus::Frozen => "STREAM_ACCOUNT_STATUS_FROZEN",
        }
    }
}
/// Stream Payment Record of a stream account
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct StreamRecord {
    /// account address
    #[prost(string, tag = "1")]
    pub account: ::prost::alloc::string::String,
    /// latest update timestamp of the stream record
    #[prost(int64, tag = "2")]
    pub crud_timestamp: i64,
    /// The per-second rate that an account's balance is changing.
    /// It is the sum of the account's inbound and outbound flow rates.
    #[prost(string, tag = "3")]
    pub netflow_rate: ::prost::alloc::string::String,
    /// The balance of the stream account at the latest CRUD timestamp.
    #[prost(string, tag = "4")]
    pub static_balance: ::prost::alloc::string::String,
    /// reserved balance of the stream account
    /// If the netflow rate is negative, the reserved balance is `netflow_rate * reserve_time`
    #[prost(string, tag = "5")]
    pub buffer_balance: ::prost::alloc::string::String,
    /// the locked balance of the stream account after it puts a new object and before the object is sealed
    #[prost(string, tag = "6")]
    pub lock_balance: ::prost::alloc::string::String,
    /// the status of the stream account
    #[prost(enumeration = "StreamAccountStatus", tag = "7")]
    pub status: i32,
    /// the unix timestamp when the stream account will be settled
    #[prost(int64, tag = "8")]
    pub settle_timestamp: i64,
    /// the accumulated outflow rates of the stream account
    #[prost(message, repeated, tag = "9")]
    pub out_flows: ::prost::alloc::vec::Vec<OutFlow>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct EventPaymentAccountUpdate {
    /// address of the payment account
    #[prost(string, tag = "1")]
    pub addr: ::prost::alloc::string::String,
    /// owner address of the payment account
    #[prost(string, tag = "2")]
    pub owner: ::prost::alloc::string::String,
    /// whether the payment account is refundable
    #[prost(bool, tag = "3")]
    pub refundable: bool,
}
/// Stream Payment Record of a stream account
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct EventStreamRecordUpdate {
    /// account address
    #[prost(string, tag = "1")]
    pub account: ::prost::alloc::string::String,
    /// latest update timestamp of the stream record
    #[prost(int64, tag = "2")]
    pub crud_timestamp: i64,
    /// The per-second rate that an account's balance is changing.
    /// It is the sum of the account's inbound and outbound flow rates.
    #[prost(string, tag = "3")]
    pub netflow_rate: ::prost::alloc::string::String,
    /// The balance of the stream account at the latest CRUD timestamp.
    #[prost(string, tag = "4")]
    pub static_balance: ::prost::alloc::string::String,
    /// reserved balance of the stream account
    /// If the netflow rate is negative, the reserved balance is `netflow_rate * reserve_time`
    #[prost(string, tag = "5")]
    pub buffer_balance: ::prost::alloc::string::String,
    /// the locked balance of the stream account after it puts a new object and before the object is sealed
    #[prost(string, tag = "6")]
    pub lock_balance: ::prost::alloc::string::String,
    /// the status of the stream account
    #[prost(enumeration = "StreamAccountStatus", tag = "7")]
    pub status: i32,
    /// the unix timestamp when the stream account will be settled
    #[prost(int64, tag = "8")]
    pub settle_timestamp: i64,
    /// the accumulated outflow rates of the stream account
    #[prost(message, repeated, tag = "9")]
    pub out_flows: ::prost::alloc::vec::Vec<OutFlow>,
}
/// EventForceSettle may be emitted on all Msgs and EndBlocker when a payment account's
/// balance or net outflow rate is changed
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct EventForceSettle {
    /// address of the payment account
    #[prost(string, tag = "1")]
    pub addr: ::prost::alloc::string::String,
    /// left balance of the payment account after force settlement
    /// if the balance is positive, it will go to the governance stream account
    /// if the balance is negative, it's the debt of the system, which will be paid by the governance stream account
    #[prost(string, tag = "2")]
    pub settled_balance: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct EventDeposit {
    /// from is the the address of the account to deposit from
    #[prost(string, tag = "1")]
    pub from: ::prost::alloc::string::String,
    /// to is the address of the stream account to deposit to
    #[prost(string, tag = "2")]
    pub to: ::prost::alloc::string::String,
    /// amount is the amount to deposit
    #[prost(string, tag = "3")]
    pub amount: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct EventWithdraw {
    /// to the address of the receive account
    #[prost(string, tag = "1")]
    pub to: ::prost::alloc::string::String,
    /// from is the address of the stream account to withdraw from
    #[prost(string, tag = "2")]
    pub from: ::prost::alloc::string::String,
    /// amount is the amount to withdraw
    #[prost(string, tag = "3")]
    pub amount: ::prost::alloc::string::String,
}
/// emit when upload/cancel/delete object, used for frontend to preview the fee changed
/// only emit in tx simulation
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct EventFeePreview {
    #[prost(string, tag = "1")]
    pub account: ::prost::alloc::string::String,
    #[prost(enumeration = "FeePreviewType", tag = "2")]
    pub fee_preview_type: i32,
    #[prost(string, tag = "3")]
    pub amount: ::prost::alloc::string::String,
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum FeePreviewType {
    PrelockedFee = 0,
    UnlockedFee = 1,
}
impl FeePreviewType {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            FeePreviewType::PrelockedFee => "FEE_PREVIEW_TYPE_PRELOCKED_FEE",
            FeePreviewType::UnlockedFee => "FEE_PREVIEW_TYPE_UNLOCKED_FEE",
        }
    }
}
/// Params defines the parameters for the module.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Params {
    /// Time duration which the buffer balance need to be reserved for NetOutFlow e.g. 6 month
    #[prost(uint64, tag = "1")]
    pub reserve_time: u64,
    /// The maximum number of payment accounts that can be created by one user
    #[prost(uint64, tag = "2")]
    pub payment_account_count_limit: u64,
    /// Time duration threshold of forced settlement.
    /// If dynamic balance is less than NetOutFlowRate * forcedSettleTime, the account can be forced settled.
    #[prost(uint64, tag = "3")]
    pub forced_settle_time: u64,
    /// the maximum number of accounts that will be forced settled in one block
    #[prost(uint64, tag = "4")]
    pub max_auto_force_settle_num: u64,
    /// The denom of fee charged in payment module
    #[prost(string, tag = "5")]
    pub fee_denom: ::prost::alloc::string::String,
    /// The tax rate to pay for validators in storage payment. The default value is 1%(0.01)
    #[prost(string, tag = "6")]
    pub validator_tax_rate: ::prost::alloc::string::String,
}
/// PaymentAccount defines a payment account
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PaymentAccount {
    /// the address of the payment account
    #[prost(string, tag = "1")]
    pub addr: ::prost::alloc::string::String,
    /// the owner address of the payment account
    #[prost(string, tag = "2")]
    pub owner: ::prost::alloc::string::String,
    /// whether the payment account is refundable
    #[prost(bool, tag = "3")]
    pub refundable: bool,
}
/// PaymentAccountCount defines the state struct which stores the number of payment accounts for an account
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PaymentAccountCount {
    /// owner is the account address
    #[prost(string, tag = "1")]
    pub owner: ::prost::alloc::string::String,
    /// count is the number of payment accounts for the account
    #[prost(uint64, tag = "2")]
    pub count: u64,
}
/// GenesisState defines the payment module's genesis state.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GenesisState {
    #[prost(message, optional, tag = "1")]
    pub params: ::core::option::Option<Params>,
    #[prost(message, repeated, tag = "2")]
    pub stream_record_list: ::prost::alloc::vec::Vec<StreamRecord>,
    #[prost(message, repeated, tag = "3")]
    pub payment_account_count_list: ::prost::alloc::vec::Vec<PaymentAccountCount>,
    #[prost(message, repeated, tag = "4")]
    pub payment_account_list: ::prost::alloc::vec::Vec<PaymentAccount>,
    /// this line is used by starport scaffolding # genesis/proto/state
    #[prost(message, repeated, tag = "5")]
    pub auto_settle_record_list: ::prost::alloc::vec::Vec<AutoSettleRecord>,
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
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryGetStreamRecordRequest {
    #[prost(string, tag = "1")]
    pub account: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryGetStreamRecordResponse {
    #[prost(message, optional, tag = "1")]
    pub stream_record: ::core::option::Option<StreamRecord>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryAllStreamRecordRequest {
    #[prost(message, optional, tag = "1")]
    pub pagination: ::core::option::Option<super::super::cosmos::base::query::v1beta1::PageRequest>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryAllStreamRecordResponse {
    #[prost(message, repeated, tag = "1")]
    pub stream_record: ::prost::alloc::vec::Vec<StreamRecord>,
    #[prost(message, optional, tag = "2")]
    pub pagination:
        ::core::option::Option<super::super::cosmos::base::query::v1beta1::PageResponse>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryGetPaymentAccountCountRequest {
    #[prost(string, tag = "1")]
    pub owner: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryGetPaymentAccountCountResponse {
    #[prost(message, optional, tag = "1")]
    pub payment_account_count: ::core::option::Option<PaymentAccountCount>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryAllPaymentAccountCountRequest {
    #[prost(message, optional, tag = "1")]
    pub pagination: ::core::option::Option<super::super::cosmos::base::query::v1beta1::PageRequest>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryAllPaymentAccountCountResponse {
    #[prost(message, repeated, tag = "1")]
    pub payment_account_count: ::prost::alloc::vec::Vec<PaymentAccountCount>,
    #[prost(message, optional, tag = "2")]
    pub pagination:
        ::core::option::Option<super::super::cosmos::base::query::v1beta1::PageResponse>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryGetPaymentAccountRequest {
    #[prost(string, tag = "1")]
    pub addr: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryGetPaymentAccountResponse {
    #[prost(message, optional, tag = "1")]
    pub payment_account: ::core::option::Option<PaymentAccount>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryAllPaymentAccountRequest {
    #[prost(message, optional, tag = "1")]
    pub pagination: ::core::option::Option<super::super::cosmos::base::query::v1beta1::PageRequest>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryAllPaymentAccountResponse {
    #[prost(message, repeated, tag = "1")]
    pub payment_account: ::prost::alloc::vec::Vec<PaymentAccount>,
    #[prost(message, optional, tag = "2")]
    pub pagination:
        ::core::option::Option<super::super::cosmos::base::query::v1beta1::PageResponse>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryDynamicBalanceRequest {
    #[prost(string, tag = "1")]
    pub account: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryDynamicBalanceResponse {
    /// dynamic balance is static balance + flowDelta
    #[prost(string, tag = "1")]
    pub dynamic_balance: ::prost::alloc::string::String,
    /// the stream record of the given account, if it does not exist, it will be default values
    #[prost(message, optional, tag = "2")]
    pub stream_record: ::core::option::Option<StreamRecord>,
    /// the timestamp of the current block
    #[prost(int64, tag = "3")]
    pub current_timestamp: i64,
    /// bank_balance is the BNB balance of the bank module
    #[prost(string, tag = "4")]
    pub bank_balance: ::prost::alloc::string::String,
    /// available_balance is bank balance + static balance
    #[prost(string, tag = "5")]
    pub available_balance: ::prost::alloc::string::String,
    /// locked_fee is buffer balance + locked balance
    #[prost(string, tag = "6")]
    pub locked_fee: ::prost::alloc::string::String,
    /// change_rate is the netflow rate of the given account
    #[prost(string, tag = "7")]
    pub change_rate: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryGetPaymentAccountsByOwnerRequest {
    #[prost(string, tag = "1")]
    pub owner: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryGetPaymentAccountsByOwnerResponse {
    #[prost(string, repeated, tag = "1")]
    pub payment_accounts: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryAllAutoSettleRecordRequest {
    #[prost(message, optional, tag = "1")]
    pub pagination: ::core::option::Option<super::super::cosmos::base::query::v1beta1::PageRequest>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryAllAutoSettleRecordResponse {
    #[prost(message, repeated, tag = "1")]
    pub auto_settle_record: ::prost::alloc::vec::Vec<AutoSettleRecord>,
    #[prost(message, optional, tag = "2")]
    pub pagination:
        ::core::option::Option<super::super::cosmos::base::query::v1beta1::PageResponse>,
}
/// MsgUpdateParams is the Msg/UpdateParams request type.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgUpdateParams {
    /// authority is the address that controls the module (defaults to x/gov unless overwritten).
    #[prost(string, tag = "1")]
    pub authority: ::prost::alloc::string::String,
    /// params defines the x/payment parameters to update.
    /// NOTE: All parameters must be supplied.
    #[prost(message, optional, tag = "2")]
    pub params: ::core::option::Option<Params>,
}
/// MsgUpdateParamsResponse defines the response structure for executing a MsgUpdateParams message.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgUpdateParamsResponse {}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgCreatePaymentAccount {
    /// creator is the address of the stream account that created the payment account
    #[prost(string, tag = "1")]
    pub creator: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgCreatePaymentAccountResponse {}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgDeposit {
    /// creator is the message signer for MsgDeposit and the address of the account to deposit from
    #[prost(string, tag = "1")]
    pub creator: ::prost::alloc::string::String,
    /// to is the address of the account to deposit to
    #[prost(string, tag = "2")]
    pub to: ::prost::alloc::string::String,
    /// amount is the amount to deposit
    #[prost(string, tag = "3")]
    pub amount: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgDepositResponse {}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgWithdraw {
    /// creator is the message signer for MsgWithdraw and the address of the receive account
    #[prost(string, tag = "1")]
    pub creator: ::prost::alloc::string::String,
    /// from is the address of the account to withdraw from
    #[prost(string, tag = "2")]
    pub from: ::prost::alloc::string::String,
    /// amount is the amount to withdraw
    #[prost(string, tag = "3")]
    pub amount: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgWithdrawResponse {}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgDisableRefund {
    /// owner is the message signer for MsgDisableRefund and the address of the payment account owner
    #[prost(string, tag = "1")]
    pub owner: ::prost::alloc::string::String,
    /// addr is the address of the payment account to disable refund
    #[prost(string, tag = "2")]
    pub addr: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgDisableRefundResponse {}
include!("greenfield.payment.tonic.rs");
// @@protoc_insertion_point(module)
