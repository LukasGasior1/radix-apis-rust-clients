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
pub struct TransactionPreviewResponse {
    /// A summarized state of the ledger on top of which the preview was performed.
    #[serde(rename = "at_ledger_state")]
    pub at_ledger_state: models::LedgerStateSummary,
    /// The hex-sbor-encoded receipt.  This field is deprecated and will be removed from the API with the release of the next  protocol update, cuttlefish. This field was provided primarily for use with the Radix  Engine Toolkit and its execution summary functionality. If you still wish to use this  functionality update your Radix Engine Toolkit and use the receipt provided in the  `radix_engine_toolkit_receipt` field of this response. 
    #[serde(rename = "encoded_receipt")]
    pub encoded_receipt: String,
    #[serde(rename = "instruction_resource_changes")]
    pub instruction_resource_changes: Vec<models::InstructionResourceChanges>,
    #[serde(rename = "logs")]
    pub logs: Vec<models::TransactionPreviewResponseLogsInner>,
    /// An optional field which is only provided if the `radix_engine_toolkit_receipt` flag is set to true when requesting a transaction preview from the API.  This receipt is primarily intended for use with the toolkit and may contain information  that is already available in the receipt provided in the `receipt` field of this  response.  A typical client of this API is not expected to use this receipt. The primary clients  this receipt is intended for is the Radix wallet or any client that needs to perform  execution summaries on their transactions. 
    #[serde(rename = "radix_engine_toolkit_receipt", skip_serializing_if = "Option::is_none")]
    pub radix_engine_toolkit_receipt: Option<serde_json::Value>,
    #[serde(rename = "receipt")]
    pub receipt: models::TransactionReceipt,
}

impl TransactionPreviewResponse {
    pub fn new(at_ledger_state: models::LedgerStateSummary, encoded_receipt: String, instruction_resource_changes: Vec<models::InstructionResourceChanges>, logs: Vec<models::TransactionPreviewResponseLogsInner>, receipt: models::TransactionReceipt) -> TransactionPreviewResponse {
        TransactionPreviewResponse {
            at_ledger_state,
            encoded_receipt,
            instruction_resource_changes,
            logs,
            radix_engine_toolkit_receipt: None,
            receipt,
        }
    }
}

