/*
 * Radix Core API
 *
 * This API is exposed by the Babylon Radix node to give clients access to the Radix Engine, Mempool and State in the node.  The default configuration is intended for use by node-runners on a private network, and is not intended to be exposed publicly. Very heavy load may impact the node's function. The node exposes a configuration flag which allows disabling certain endpoints which may be problematic, but monitoring is advised. This configuration parameter is `api.core.flags.enable_unbounded_endpoints` / `RADIXDLT_CORE_API_FLAGS_ENABLE_UNBOUNDED_ENDPOINTS`.  This API exposes queries against the node's current state (see `/lts/state/` or `/state/`), and streams of transaction history (under `/lts/stream/` or `/stream`).  If you require queries against snapshots of historical ledger state, you may also wish to consider using the [Gateway API](https://docs-babylon.radixdlt.com/).  ## Integration and forward compatibility guarantees  Integrators (such as exchanges) are recommended to use the `/lts/` endpoints - they have been designed to be clear and simple for integrators wishing to create and monitor transactions involving fungible transfers to/from accounts.  All endpoints under `/lts/` have high guarantees of forward compatibility in future node versions. We may add new fields, but existing fields will not be changed. Assuming the integrating code uses a permissive JSON parser which ignores unknown fields, any additions will not affect existing code.  Other endpoints may be changed with new node versions carrying protocol-updates, although any breaking changes will be flagged clearly in the corresponding release notes.  All responses may have additional fields added, so clients are advised to use JSON parsers which ignore unknown fields on JSON objects. 
 *
 * The version of the OpenAPI document: v1.2.3
 * 
 * Generated by: https://openapi-generator.tech
 */

use crate::models;
use serde::{Deserialize, Serialize};

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct StateValidatorResponse {
    /// The Bech32m-encoded human readable version of the component address
    #[serde(rename = "address")]
    pub address: String,
    /// A summarized state of the ledger at which the query was performed.
    #[serde(rename = "at_ledger_state")]
    pub at_ledger_state: models::LedgerStateSummary,
    /// Any descendent nodes owned directly or indirectly by the component
    #[serde(rename = "descendent_nodes")]
    pub descendent_nodes: Vec<models::StateComponentDescendentNode>,
    #[serde(rename = "owner_role")]
    pub owner_role: models::Substate,
    #[serde(rename = "protocol_update_readiness_signal")]
    pub protocol_update_readiness_signal: models::Substate,
    #[serde(rename = "state")]
    pub state: models::Substate,
    /// Any vaults owned directly or indirectly by the component
    #[serde(rename = "vaults")]
    pub vaults: Vec<models::VaultBalance>,
}

impl StateValidatorResponse {
    pub fn new(address: String, at_ledger_state: models::LedgerStateSummary, descendent_nodes: Vec<models::StateComponentDescendentNode>, owner_role: models::Substate, protocol_update_readiness_signal: models::Substate, state: models::Substate, vaults: Vec<models::VaultBalance>) -> StateValidatorResponse {
        StateValidatorResponse {
            address,
            at_ledger_state,
            descendent_nodes,
            owner_role,
            protocol_update_readiness_signal,
            state,
            vaults,
        }
    }
}

