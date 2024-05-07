use cgp_core::prelude::*;
use hermes_cosmos_chain_components::encoding::components::CosmosEncodingComponents;
use hermes_cosmos_chain_components::types::tendermint::{
    ProtoTendermintClientState, ProtoTendermintConsensusState, TendermintClientState,
    TendermintConsensusState,
};
use hermes_encoding_components::impls::convert::{ConvertFrom, TryConvertFrom};
use hermes_protobuf_encoding_components::types::Any;
use hermes_wasm_client_components::impls::encoding::components::WasmEncodingComponents;
use hermes_wasm_client_components::types::client_state::{
    DecodeViaWasmClientState, ProtoWasmClientState, WasmClientState,
};
use hermes_wasm_client_components::types::consensus_state::{
    ProtoWasmConsensusState, WasmConsensusState,
};

use crate::sovereign::types::client_state::{ProtoSovereignClientState, SovereignClientState};
use crate::sovereign::types::consensus_state::{
    ProtoSovereignConsensusState, SovereignConsensusState,
};

pub struct SovereignConverterComponents;

delegate_components! {
    SovereignConverterComponents {
        [
            (TendermintClientState, ProtoTendermintClientState),
            (ProtoTendermintClientState, TendermintClientState),
            (TendermintConsensusState, ProtoTendermintConsensusState),
            (ProtoTendermintConsensusState, TendermintConsensusState),
            (TendermintClientState, Any),
            (Any, TendermintClientState),
            (TendermintConsensusState, Any),
            (Any, TendermintConsensusState),
        ]:
            CosmosEncodingComponents,

        [
            (WasmClientState, ProtoWasmClientState),
            (ProtoWasmClientState, WasmClientState),

            (WasmConsensusState, ProtoWasmConsensusState),
            (ProtoWasmConsensusState, WasmConsensusState),

            (WasmClientState, Any),
            (Any, WasmClientState),

            (WasmConsensusState, Any),
            (Any, WasmConsensusState),
        ]:
            WasmEncodingComponents,

        (SovereignClientState, ProtoSovereignClientState):
            ConvertFrom,

        (ProtoSovereignClientState, SovereignClientState):
            TryConvertFrom,

        (SovereignConsensusState, ProtoSovereignConsensusState):
            ConvertFrom,

        (ProtoSovereignConsensusState, SovereignConsensusState):
            TryConvertFrom,

        (Any, SovereignClientState):
            DecodeViaWasmClientState,

        (Any, SovereignConsensusState):
            DecodeViaWasmClientState,
    }
}