/*
 * Radix Core API
 *
 * This API is exposed by the Babylon Radix node to give clients access to the Radix Engine, Mempool and State in the node.  The default configuration is intended for use by node-runners on a private network, and is not intended to be exposed publicly. Very heavy load may impact the node's function. The node exposes a configuration flag which allows disabling certain endpoints which may be problematic, but monitoring is advised. This configuration parameter is `api.core.flags.enable_unbounded_endpoints` / `RADIXDLT_CORE_API_FLAGS_ENABLE_UNBOUNDED_ENDPOINTS`.  This API exposes queries against the node's current state (see `/lts/state/` or `/state/`), and streams of transaction history (under `/lts/stream/` or `/stream`).  If you require queries against snapshots of historical ledger state, you may also wish to consider using the [Gateway API](https://docs-babylon.radixdlt.com/).  ## Integration and forward compatibility guarantees  Integrators (such as exchanges) are recommended to use the `/lts/` endpoints - they have been designed to be clear and simple for integrators wishing to create and monitor transactions involving fungible transfers to/from accounts.  All endpoints under `/lts/` have high guarantees of forward compatibility in future node versions. We may add new fields, but existing fields will not be changed. Assuming the integrating code uses a permissive JSON parser which ignores unknown fields, any additions will not affect existing code.  Other endpoints may be changed with new node versions carrying protocol-updates, although any breaking changes will be flagged clearly in the corresponding release notes.  All responses may have additional fields added, so clients are advised to use JSON parsers which ignore unknown fields on JSON objects. 
 *
 * The version of the OpenAPI document: v1.3.0
 * 
 * Generated by: https://openapi-generator.tech
 */

use crate::models;
use serde::{Deserialize, Serialize};

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct LimitParameters {
    /// A decimal string-encoded 64-bit unsigned integer, representing the configured maximum call depth allowed during transaction execution. 
    #[serde(rename = "max_call_depth")]
    pub max_call_depth: String,
    /// A decimal string-encoded 64-bit unsigned integer, representing the configured maximum byte size of a single emitted event. 
    #[serde(rename = "max_event_size")]
    pub max_event_size: String,
    /// A decimal string-encoded 64-bit unsigned integer, representing the configured maximum byte size of all substates kept on the heap during a single transaction's execution. 
    #[serde(rename = "max_heap_substate_total_bytes")]
    pub max_heap_substate_total_bytes: String,
    /// A decimal string-encoded 64-bit unsigned integer, representing the configured maximum byte size of a single call's input parameters. 
    #[serde(rename = "max_invoke_input_size")]
    pub max_invoke_input_size: String,
    /// A decimal string-encoded 64-bit unsigned integer, representing the configured maximum byte size of a single logged line. 
    #[serde(rename = "max_log_size")]
    pub max_log_size: String,
    /// A decimal string-encoded 64-bit unsigned integer, representing the configured maximum count of events emitted during a single transaction's execution. 
    #[serde(rename = "max_number_of_events")]
    pub max_number_of_events: String,
    /// A decimal string-encoded 64-bit unsigned integer, representing the configured maximum count of log lines emitted during a single transaction's execution. 
    #[serde(rename = "max_number_of_logs")]
    pub max_number_of_logs: String,
    /// A decimal string-encoded 64-bit unsigned integer, representing the configured maximum byte size of a single panic message. 
    #[serde(rename = "max_panic_message_size")]
    pub max_panic_message_size: String,
    /// A decimal string-encoded 64-bit unsigned integer, representing the configured maximum byte size of a Substate's key in the low-level Substate database. 
    #[serde(rename = "max_substate_key_size")]
    pub max_substate_key_size: String,
    /// A decimal string-encoded 64-bit unsigned integer, representing the configured maximum byte size of a Substate's value in the low-level Substate database. 
    #[serde(rename = "max_substate_value_size")]
    pub max_substate_value_size: String,
    /// A decimal string-encoded 64-bit unsigned integer, representing the configured maximum byte size of all substates kept in the track during a single transaction's execution. 
    #[serde(rename = "max_track_substate_total_bytes")]
    pub max_track_substate_total_bytes: String,
}

impl LimitParameters {
    pub fn new(max_call_depth: String, max_event_size: String, max_heap_substate_total_bytes: String, max_invoke_input_size: String, max_log_size: String, max_number_of_events: String, max_number_of_logs: String, max_panic_message_size: String, max_substate_key_size: String, max_substate_value_size: String, max_track_substate_total_bytes: String) -> LimitParameters {
        LimitParameters {
            max_call_depth,
            max_event_size,
            max_heap_substate_total_bytes,
            max_invoke_input_size,
            max_log_size,
            max_number_of_events,
            max_number_of_logs,
            max_panic_message_size,
            max_substate_key_size,
            max_substate_value_size,
            max_track_substate_total_bytes,
        }
    }
}

