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

/// LtsCommittedTransactionOutcome : For the given transaction, contains the status, total fee summary and individual entity resource balance changes. The balance changes accounts for the fee payments as well. 
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct LtsCommittedTransactionOutcome {
    /// The hex-encoded transaction accumulator hash. This hash captures the order of all transactions on ledger. This hash is `ACC_{N+1} = combine(ACC_N, LEDGER_HASH_{N}))` (where `combine()` is an arbitrary deterministic function we use). 
    #[serde(rename = "accumulator_hash")]
    pub accumulator_hash: String,
    /// A list of all fungible balance updates which occurred in this transaction, aggregated by the global entity (such as account) which owns the vaults which were updated. 
    #[serde(rename = "fungible_entity_balance_changes")]
    pub fungible_entity_balance_changes: Vec<models::LtsEntityFungibleBalanceChanges>,
    /// Non fungible changes per entity and resource 
    #[serde(rename = "non_fungible_entity_balance_changes")]
    pub non_fungible_entity_balance_changes: Vec<models::LtsEntityNonFungibleBalanceChanges>,
    /// An integer between `0` and `10^14`, marking the proposer timestamp in ms.
    #[serde(rename = "proposer_timestamp_ms")]
    pub proposer_timestamp_ms: u64,
    /// A list of the resultant fungible account balances for any balances which changed in this transaction. Only balances for accounts are returned, not any other kind of entity. 
    #[serde(rename = "resultant_account_fungible_balances")]
    pub resultant_account_fungible_balances: Vec<models::LtsResultantAccountFungibleBalances>,
    #[serde(rename = "state_version")]
    pub state_version: u64,
    #[serde(rename = "status")]
    pub status: models::LtsCommittedTransactionStatus,
    /// The string-encoded decimal representing the total amount of XRD paid as fee (execution, validator tip and royalties). A decimal is formed of some signed integer `m` of attos (`10^(-18)`) units, where `-2^(192 - 1) <= m < 2^(192 - 1)`. 
    #[serde(rename = "total_fee")]
    pub total_fee: String,
    /// Only present if the transaction is a user-submitted notarized transaction.
    #[serde(rename = "user_transaction_identifiers", skip_serializing_if = "Option::is_none")]
    pub user_transaction_identifiers: Option<Box<models::TransactionIdentifiers>>,
}

impl LtsCommittedTransactionOutcome {
    /// For the given transaction, contains the status, total fee summary and individual entity resource balance changes. The balance changes accounts for the fee payments as well. 
    pub fn new(accumulator_hash: String, fungible_entity_balance_changes: Vec<models::LtsEntityFungibleBalanceChanges>, non_fungible_entity_balance_changes: Vec<models::LtsEntityNonFungibleBalanceChanges>, proposer_timestamp_ms: u64, resultant_account_fungible_balances: Vec<models::LtsResultantAccountFungibleBalances>, state_version: u64, status: models::LtsCommittedTransactionStatus, total_fee: String) -> LtsCommittedTransactionOutcome {
        LtsCommittedTransactionOutcome {
            accumulator_hash,
            fungible_entity_balance_changes,
            non_fungible_entity_balance_changes,
            proposer_timestamp_ms,
            resultant_account_fungible_balances,
            state_version,
            status,
            total_fee,
            user_transaction_identifiers: None,
        }
    }
}

