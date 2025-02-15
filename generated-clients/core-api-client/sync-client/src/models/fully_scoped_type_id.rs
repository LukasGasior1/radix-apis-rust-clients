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

/// FullyScopedTypeId : An identifier for a type in the context of a schema in an entity's schema partition.  Note - this type provides a schema context even for well-known types where this context is effectively irrelevant. 
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct FullyScopedTypeId {
    /// Bech32m-encoded human readable version of the entity's address (ie the entity's node id)
    #[serde(rename = "entity_address")]
    pub entity_address: String,
    #[serde(rename = "local_type_id")]
    pub local_type_id: Box<models::LocalTypeId>,
    /// The hex-encoded schema hash, capturing the identity of an SBOR schema.
    #[serde(rename = "schema_hash")]
    pub schema_hash: String,
}

impl FullyScopedTypeId {
    /// An identifier for a type in the context of a schema in an entity's schema partition.  Note - this type provides a schema context even for well-known types where this context is effectively irrelevant. 
    pub fn new(entity_address: String, local_type_id: models::LocalTypeId, schema_hash: String) -> FullyScopedTypeId {
        FullyScopedTypeId {
            entity_address,
            local_type_id: Box::new(local_type_id),
            schema_hash,
        }
    }
}

