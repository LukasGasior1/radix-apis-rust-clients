/*
 * Radix Core API - Babylon (Bottlenose)
 *
 * This API is exposed by the Babylon Radix node to give clients access to the Radix Engine, Mempool and State in the node.  The default configuration is intended for use by node-runners on a private network, and is not intended to be exposed publicly. Very heavy load may impact the node's function. The node exposes a configuration flag which allows disabling certain endpoints which may be problematic, but monitoring is advised. This configuration parameter is `api.core.flags.enable_unbounded_endpoints` / `RADIXDLT_CORE_API_FLAGS_ENABLE_UNBOUNDED_ENDPOINTS`.  This API exposes queries against the node's current state (see `/lts/state/` or `/state/`), and streams of transaction history (under `/lts/stream/` or `/stream`).  If you require queries against snapshots of historical ledger state, you may also wish to consider using the [Gateway API](https://docs-babylon.radixdlt.com/).  ## Integration and forward compatibility guarantees  Integrators (such as exchanges) are recommended to use the `/lts/` endpoints - they have been designed to be clear and simple for integrators wishing to create and monitor transactions involving fungible transfers to/from accounts.  All endpoints under `/lts/` have high guarantees of forward compatibility in future node versions. We may add new fields, but existing fields will not be changed. Assuming the integrating code uses a permissive JSON parser which ignores unknown fields, any additions will not affect existing code.  Other endpoints may be changed with new node versions carrying protocol-updates, although any breaking changes will be flagged clearly in the corresponding release notes.  All responses may have additional fields added, so clients are advised to use JSON parsers which ignore unknown fields on JSON objects. 
 *
 * The version of the OpenAPI document: v1.2.1
 * 
 * Generated by: https://openapi-generator.tech
 */

use crate::models;
use serde::{Deserialize, Serialize};

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct LtsEntityFungibleBalanceChanges {
    /// The Bech32m-encoded human readable version of the entity's address
    #[serde(rename = "entity_address")]
    pub entity_address: String,
    /// If present, this field indicates the entity contributed to the payment of the fee. The change in balance will always be negative. NOTE: This property is deprecated but kept for backwards compatibility. This entry is duplicated in  `fee_balance_changes`. 
    #[serde(rename = "fee_balance_change", skip_serializing_if = "Option::is_none")]
    pub fee_balance_change: Option<Box<models::LtsFungibleResourceBalanceChange>>,
    /// If present, this field indicates fee-related balance changes, for example:  - Payment of the fee (including tip and royalty) - Distribution of royalties - Distribution of the fee and tip to the consensus-manager, for distributing to the relevant   validator/s at end of epoch  See https://www.radixdlt.com/blog/how-fees-work-in-babylon for further information on how fee payment works at Babylon. 
    #[serde(rename = "fee_balance_changes")]
    pub fee_balance_changes: Vec<models::LtsFeeFungibleResourceBalanceChange>,
    #[serde(rename = "non_fee_balance_changes")]
    pub non_fee_balance_changes: Vec<models::LtsFungibleResourceBalanceChange>,
}

impl LtsEntityFungibleBalanceChanges {
    pub fn new(entity_address: String, fee_balance_changes: Vec<models::LtsFeeFungibleResourceBalanceChange>, non_fee_balance_changes: Vec<models::LtsFungibleResourceBalanceChange>) -> LtsEntityFungibleBalanceChanges {
        LtsEntityFungibleBalanceChanges {
            entity_address,
            fee_balance_change: None,
            fee_balance_changes,
            non_fee_balance_changes,
        }
    }
}

