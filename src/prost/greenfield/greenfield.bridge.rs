// @generated
/// EventCrossTransferOut is emitted when a cross chain transfer out tx created
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct EventCrossTransferOut {
    /// From addres of the cross chain transfer tx
    #[prost(string, tag = "1")]
    pub from: ::prost::alloc::string::String,
    /// To addres of the cross chain transfer tx
    #[prost(string, tag = "2")]
    pub to: ::prost::alloc::string::String,
    /// Amount of the cross chain transfer tx
    #[prost(message, optional, tag = "3")]
    pub amount: ::core::option::Option<super::super::cosmos::base::v1beta1::Coin>,
    /// Relayer fee of the cross chain transfer tx
    #[prost(message, optional, tag = "4")]
    pub relayer_fee: ::core::option::Option<super::super::cosmos::base::v1beta1::Coin>,
    /// Sequence of the corresponding cross chain package
    #[prost(uint64, tag = "5")]
    pub sequence: u64,
}
/// EventCrossTransferOutRefund is emitted when a cross chain transfer out tx failed
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct EventCrossTransferOutRefund {
    /// Refund address of the failed cross chain transfer tx
    #[prost(string, tag = "1")]
    pub refund_address: ::prost::alloc::string::String,
    /// Amount of the failed cross chain transfer tx
    #[prost(message, optional, tag = "2")]
    pub amount: ::core::option::Option<super::super::cosmos::base::v1beta1::Coin>,
    /// Refund reason of the failed cross chain transfer tx
    #[prost(enumeration = "RefundReason", tag = "3")]
    pub refund_reason: i32,
    /// Sequence of the corresponding cross chain package
    #[prost(uint64, tag = "4")]
    pub sequence: u64,
}
/// EventCrossTransferIn is emitted when a cross chain transfer in tx happened
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct EventCrossTransferIn {
    /// Amount of the cross chain transfer tx
    #[prost(message, optional, tag = "1")]
    pub amount: ::core::option::Option<super::super::cosmos::base::v1beta1::Coin>,
    /// Receiver of the cross chain transfer tx
    #[prost(string, tag = "2")]
    pub receiver_address: ::prost::alloc::string::String,
    /// Refund of the cross chain transfer tx in BSC
    #[prost(string, tag = "3")]
    pub refund_address: ::prost::alloc::string::String,
    /// Sequence of the corresponding cross chain package
    #[prost(uint64, tag = "4")]
    pub sequence: u64,
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum RefundReason {
    Unknown = 0,
    InsufficientBalance = 1,
    FailAck = 2,
}
impl RefundReason {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            RefundReason::Unknown => "REFUND_REASON_UNKNOWN",
            RefundReason::InsufficientBalance => "REFUND_REASON_INSUFFICIENT_BALANCE",
            RefundReason::FailAck => "REFUND_REASON_FAIL_ACK",
        }
    }
}
/// Params defines the parameters for the module.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Params {
    /// Relayer fee for the cross chain transfer out tx
    #[prost(string, tag = "1")]
    pub transfer_out_relayer_fee: ::prost::alloc::string::String,
    /// Relayer fee for the ACK or FAIL_ACK package of the cross chain transfer out tx
    #[prost(string, tag = "2")]
    pub transfer_out_ack_relayer_fee: ::prost::alloc::string::String,
}
// this line is used by starport scaffolding # genesis/proto/import

/// GenesisState defines the bridge module's genesis state.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GenesisState {
    /// Params defines all the paramaters of the module.
    ///
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
/// MsgTransferOut is the Msg/TransferOut request type.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgTransferOut {
    /// from address
    #[prost(string, tag = "1")]
    pub from: ::prost::alloc::string::String,
    /// to address
    #[prost(string, tag = "2")]
    pub to: ::prost::alloc::string::String,
    /// transfer token amount
    #[prost(message, optional, tag = "3")]
    pub amount: ::core::option::Option<super::super::cosmos::base::v1beta1::Coin>,
}
/// MsgTransferOutResponse is the Msg/TransferOut response type.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgTransferOutResponse {}
/// MsgUpdateParams is the Msg/UpdateParams request type.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgUpdateParams {
    /// authority is the address that controls the module (defaults to x/gov unless overwritten).
    #[prost(string, tag = "1")]
    pub authority: ::prost::alloc::string::String,
    /// params defines the x/crosschain parameters to update.
    ///
    /// NOTE: All parameters must be supplied.
    #[prost(message, optional, tag = "2")]
    pub params: ::core::option::Option<Params>,
}
/// MsgUpdateParamsResponse defines the response structure for executing a
/// MsgUpdateParams message.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgUpdateParamsResponse {}
include!("greenfield.bridge.tonic.rs");
// @@protoc_insertion_point(module)
