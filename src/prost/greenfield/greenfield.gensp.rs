// @generated
/// GenesisState defines the gensp module's genesis state.
/// GenesisState defines the raw genesis transaction in JSON.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GenesisState {
    /// gen_txs defines the genesis transactions.
    #[prost(bytes = "vec", repeated, tag = "1")]
    pub gensp_txs: ::prost::alloc::vec::Vec<::prost::alloc::vec::Vec<u8>>,
}
/// Params defines the parameters for the module.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Params {}
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
include!("greenfield.gensp.tonic.rs");
// @@protoc_insertion_point(module)
