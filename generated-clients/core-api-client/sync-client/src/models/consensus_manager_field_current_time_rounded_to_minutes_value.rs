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
pub struct ConsensusManagerFieldCurrentTimeRoundedToMinutesValue {
    /// The latest round proposer's timestamp (rounded down to the current minute). An honest quorum of validators keeps this aligned with wall-clock time, and non-decreasing.  Note: in abnormal cases (e.g. Byzantine network quorum), this on-ledger field may be set to an arbitrary, extreme value allowed by 64-bit signed integer. The API will still clamp the timestamp to `0 <= ms <= 100000000000000 (== 10^14)`, which translates to `1970-01-01T00:00:00.000Z <= t <= 5138-11-16T09:46:40.000Z`. 
    #[serde(rename = "proposer_timestamp_rounded_down_to_minute")]
    pub proposer_timestamp_rounded_down_to_minute: models::InstantMs,
}

impl ConsensusManagerFieldCurrentTimeRoundedToMinutesValue {
    pub fn new(proposer_timestamp_rounded_down_to_minute: models::InstantMs) -> ConsensusManagerFieldCurrentTimeRoundedToMinutesValue {
        ConsensusManagerFieldCurrentTimeRoundedToMinutesValue {
            proposer_timestamp_rounded_down_to_minute,
        }
    }
}

