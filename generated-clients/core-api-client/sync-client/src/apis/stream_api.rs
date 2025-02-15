/*
 * Radix Core API
 *
 * This API is exposed by the Babylon Radix node to give clients access to the Radix Engine, Mempool and State in the node.  The default configuration is intended for use by node-runners on a private network, and is not intended to be exposed publicly. Very heavy load may impact the node's function. The node exposes a configuration flag which allows disabling certain endpoints which may be problematic, but monitoring is advised. This configuration parameter is `api.core.flags.enable_unbounded_endpoints` / `RADIXDLT_CORE_API_FLAGS_ENABLE_UNBOUNDED_ENDPOINTS`.  This API exposes queries against the node's current state (see `/lts/state/` or `/state/`), and streams of transaction history (under `/lts/stream/` or `/stream`).  If you require queries against snapshots of historical ledger state, you may also wish to consider using the [Gateway API](https://docs-babylon.radixdlt.com/).  ## Integration and forward compatibility guarantees  Integrators (such as exchanges) are recommended to use the `/lts/` endpoints - they have been designed to be clear and simple for integrators wishing to create and monitor transactions involving fungible transfers to/from accounts.  All endpoints under `/lts/` have high guarantees of forward compatibility in future node versions. We may add new fields, but existing fields will not be changed. Assuming the integrating code uses a permissive JSON parser which ignores unknown fields, any additions will not affect existing code.  Other endpoints may be changed with new node versions carrying protocol-updates, although any breaking changes will be flagged clearly in the corresponding release notes.  All responses may have additional fields added, so clients are advised to use JSON parsers which ignore unknown fields on JSON objects. 
 *
 * The version of the OpenAPI document: v1.2.3
 * 
 * Generated by: https://openapi-generator.tech
 */


use reqwest;
use serde::{Deserialize, Serialize};
use crate::{apis::ResponseContent, models};
use super::{Error, configuration};


/// struct for typed errors of method [`stream_proofs_post`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum StreamProofsPostError {
    Status400(models::StreamProofsErrorResponse),
    Status500(models::StreamProofsErrorResponse),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`stream_transactions_post`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum StreamTransactionsPostError {
    Status400(models::StreamTransactionsErrorResponse),
    Status500(models::StreamTransactionsErrorResponse),
    UnknownValue(serde_json::Value),
}


/// Returns a stream of proofs committed to the node's ledger.  NOTE: This endpoint may return different results on different nodes: * Each node may persist different subset of signatures on a given proofs, as long as enough of the validator set has signed. * Inside an epoch, different nodes may receive and persist / keep different proofs, subject to constraints on gaps between proofs.  Proofs during an epoch can also be garbage collected by the node after the fact. Therefore proofs may disappear from this stream.  Some proofs (such as during genesis and protocol update enactment) are created on a node and don't include signatures.  This stream accepts four different options in the request: * All proofs forward (from state version) * All end-of-epoch proofs (from epoch number) * All end-of-epoch proofs triggering a protocol update * All node-injected proofs enacting genesis or a protocol update (for protocol update name, from state version)  The end-of-epoch proofs can be used to \"trustlessly\" verify the validator set for a given epoch. By tracking the fact that validators for epoch N sign the next validator set for epoch N + 1, this chain of proofs can be used to provide proof of the current validator set from a hardcoded start.  When a validator set is known for a given epoch, this can be used to verify the various transaction hash trees in the epoch, and to prove other data.  NOTE: This endpoint was built after agreeing the new Radix convention for paged APIs. Its models therefore follow the new convention, rather than attempting to align with existing loose Core API conventions. 
pub fn stream_proofs_post(configuration: &configuration::Configuration, stream_proofs_request: models::StreamProofsRequest) -> Result<models::StreamProofsResponse, Error<StreamProofsPostError>> {
    let local_var_configuration = configuration;

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!("{}/stream/proofs", local_var_configuration.base_path);
    let mut local_var_req_builder = local_var_client.request(reqwest::Method::POST, local_var_uri_str.as_str());

    if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
        local_var_req_builder = local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }
    local_var_req_builder = local_var_req_builder.json(&stream_proofs_request);

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req)?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text()?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        serde_json::from_str(&local_var_content).map_err(Error::from)
    } else {
        let local_var_entity: Option<StreamProofsPostError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

/// Returns the list of committed transactions. 
pub fn stream_transactions_post(configuration: &configuration::Configuration, stream_transactions_request: models::StreamTransactionsRequest) -> Result<models::StreamTransactionsResponse, Error<StreamTransactionsPostError>> {
    let local_var_configuration = configuration;

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!("{}/stream/transactions", local_var_configuration.base_path);
    let mut local_var_req_builder = local_var_client.request(reqwest::Method::POST, local_var_uri_str.as_str());

    if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
        local_var_req_builder = local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }
    local_var_req_builder = local_var_req_builder.json(&stream_transactions_request);

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req)?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text()?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        serde_json::from_str(&local_var_content).map_err(Error::from)
    } else {
        let local_var_entity: Option<StreamTransactionsPostError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

