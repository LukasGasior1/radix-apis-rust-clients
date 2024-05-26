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
pub struct ConsensusManagerFieldStateValue {
    /// The actual time the epoch started. Not used by any logic, but the difference between this and the effective start gives a measure of the time it took for the end-of-epoch to be noticed.  Note: in abnormal cases (e.g. Byzantine network quorum), this on-ledger field may be set to an arbitrary, extreme value allowed by 64-bit signed integer. The API will still clamp the timestamp to `0 <= ms <= 100000000000000 (== 10^14)`, which translates to `1970-01-01T00:00:00.000Z <= t <= 5138-11-16T09:46:40.000Z`. 
    #[serde(rename = "actual_epoch_start")]
    pub actual_epoch_start: Box<models::InstantMs>,
    #[serde(rename = "current_leader", skip_serializing_if = "Option::is_none")]
    pub current_leader: Option<Box<models::ActiveValidatorIndex>>,
    /// The effective time the epoch started. A drift-free measure, used to work out when the epoch should ideally end.   Note: in abnormal cases (e.g. Byzantine network quorum), this on-ledger field may be set to an arbitrary, extreme value allowed by 64-bit signed integer. The API will still clamp the timestamp to `0 <= ms <= 100000000000000 (== 10^14)`, which translates to `1970-01-01T00:00:00.000Z <= t <= 5138-11-16T09:46:40.000Z`. 
    #[serde(rename = "effective_epoch_start")]
    pub effective_epoch_start: Box<models::InstantMs>,
    /// An integer between `0` and `10^10`, marking the current epoch
    #[serde(rename = "epoch")]
    pub epoch: u64,
    #[serde(rename = "is_started")]
    pub is_started: bool,
    /// An integer between `0` and `10^10`, marking the current round in an epoch
    #[serde(rename = "round")]
    pub round: u64,
}

impl ConsensusManagerFieldStateValue {
    pub fn new(actual_epoch_start: models::InstantMs, effective_epoch_start: models::InstantMs, epoch: u64, is_started: bool, round: u64) -> ConsensusManagerFieldStateValue {
        ConsensusManagerFieldStateValue {
            actual_epoch_start: Box::new(actual_epoch_start),
            current_leader: None,
            effective_epoch_start: Box::new(effective_epoch_start),
            epoch,
            is_started,
            round,
        }
    }
}

