/*
 * Radix Core API - Babylon (Bottlenose)
 *
 * This API is exposed by the Babylon Radix node to give clients access to the Radix Engine, Mempool and State in the node.  The default configuration is intended for use by node-runners on a private network, and is not intended to be exposed publicly. Very heavy load may impact the node's function. The node exposes a configuration flag which allows disabling certain endpoints which may be problematic, but monitoring is advised. This configuration parameter is `api.core.flags.enable_unbounded_endpoints` / `RADIXDLT_CORE_API_FLAGS_ENABLE_UNBOUNDED_ENDPOINTS`.  This API exposes queries against the node's current state (see `/lts/state/` or `/state/`), and streams of transaction history (under `/lts/stream/` or `/stream`).  If you require queries against snapshots of historical ledger state, you may also wish to consider using the [Gateway API](https://docs-babylon.radixdlt.com/).  ## Integration and forward compatibility guarantees  Integrators (such as exchanges) are recommended to use the `/lts/` endpoints - they have been designed to be clear and simple for integrators wishing to create and monitor transactions involving fungible transfers to/from accounts.  All endpoints under `/lts/` have high guarantees of forward compatibility in future node versions. We may add new fields, but existing fields will not be changed. Assuming the integrating code uses a permissive JSON parser which ignores unknown fields, any additions will not affect existing code.  Other endpoints may be changed with new node versions carrying protocol-updates, although any breaking changes will be flagged clearly in the corresponding release notes.  All responses may have additional fields added, so clients are advised to use JSON parsers which ignore unknown fields on JSON objects. 
 *
 * The version of the OpenAPI document: v1.2.1
 * 
 * Generated by: https://openapi-generator.tech
 */


use reqwest;
use serde::{Deserialize, Serialize};
use crate::{apis::ResponseContent, models};
use super::{Error, configuration};


/// struct for typed errors of method [`lts_state_account_all_fungible_resource_balances_post`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum LtsStateAccountAllFungibleResourceBalancesPostError {
    Status500(models::BasicErrorResponse),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`lts_state_account_deposit_behaviour_post`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum LtsStateAccountDepositBehaviourPostError {
    Status500(models::BasicErrorResponse),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`lts_state_account_fungible_resource_balance_post`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum LtsStateAccountFungibleResourceBalancePostError {
    Status500(models::BasicErrorResponse),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`lts_stream_account_transaction_outcomes_post`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum LtsStreamAccountTransactionOutcomesPostError {
    Status400(models::BasicErrorResponse),
    Status500(models::BasicErrorResponse),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`lts_stream_transaction_outcomes_post`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum LtsStreamTransactionOutcomesPostError {
    Status400(models::BasicErrorResponse),
    Status500(models::BasicErrorResponse),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`lts_transaction_construction_post`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum LtsTransactionConstructionPostError {
    Status500(models::BasicErrorResponse),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`lts_transaction_status_post`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum LtsTransactionStatusPostError {
    Status400(models::BasicErrorResponse),
    Status500(models::BasicErrorResponse),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`lts_transaction_submit_post`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum LtsTransactionSubmitPostError {
    Status400(models::LtsTransactionSubmitErrorResponse),
    Status500(models::LtsTransactionSubmitErrorResponse),
    UnknownValue(serde_json::Value),
}


/// Returns balances for all resources associated with an account
pub async fn lts_state_account_all_fungible_resource_balances_post(configuration: &configuration::Configuration, lts_state_account_all_fungible_resource_balances_request: models::LtsStateAccountAllFungibleResourceBalancesRequest) -> Result<models::LtsStateAccountAllFungibleResourceBalancesResponse, Error<LtsStateAccountAllFungibleResourceBalancesPostError>> {
    let local_var_configuration = configuration;

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!("{}/lts/state/account-all-fungible-resource-balances", local_var_configuration.base_path);
    let mut local_var_req_builder = local_var_client.request(reqwest::Method::POST, local_var_uri_str.as_str());

    if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
        local_var_req_builder = local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }
    local_var_req_builder = local_var_req_builder.json(&lts_state_account_all_fungible_resource_balances_request);

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        serde_json::from_str(&local_var_content).map_err(Error::from)
    } else {
        let local_var_entity: Option<LtsStateAccountAllFungibleResourceBalancesPostError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

/// Returns deposit behaviour of a single account for multiple resource addresses
pub async fn lts_state_account_deposit_behaviour_post(configuration: &configuration::Configuration, lts_state_account_deposit_behaviour_request: models::LtsStateAccountDepositBehaviourRequest) -> Result<models::LtsStateAccountDepositBehaviourResponse, Error<LtsStateAccountDepositBehaviourPostError>> {
    let local_var_configuration = configuration;

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!("{}/lts/state/account-deposit-behaviour", local_var_configuration.base_path);
    let mut local_var_req_builder = local_var_client.request(reqwest::Method::POST, local_var_uri_str.as_str());

    if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
        local_var_req_builder = local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }
    local_var_req_builder = local_var_req_builder.json(&lts_state_account_deposit_behaviour_request);

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        serde_json::from_str(&local_var_content).map_err(Error::from)
    } else {
        let local_var_entity: Option<LtsStateAccountDepositBehaviourPostError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

/// Returns balance of a single fungible resource in an account
pub async fn lts_state_account_fungible_resource_balance_post(configuration: &configuration::Configuration, lts_state_account_fungible_resource_balance_request: models::LtsStateAccountFungibleResourceBalanceRequest) -> Result<models::LtsStateAccountFungibleResourceBalanceResponse, Error<LtsStateAccountFungibleResourceBalancePostError>> {
    let local_var_configuration = configuration;

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!("{}/lts/state/account-fungible-resource-balance", local_var_configuration.base_path);
    let mut local_var_req_builder = local_var_client.request(reqwest::Method::POST, local_var_uri_str.as_str());

    if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
        local_var_req_builder = local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }
    local_var_req_builder = local_var_req_builder.json(&lts_state_account_fungible_resource_balance_request);

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        serde_json::from_str(&local_var_content).map_err(Error::from)
    } else {
        let local_var_entity: Option<LtsStateAccountFungibleResourceBalancePostError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

/// Returns a list of committed transaction outcomes (containing balance changes) from a given state version, filtered to only transactions which involved the given account. 
pub async fn lts_stream_account_transaction_outcomes_post(configuration: &configuration::Configuration, lts_stream_account_transaction_outcomes_request: models::LtsStreamAccountTransactionOutcomesRequest) -> Result<models::LtsStreamAccountTransactionOutcomesResponse, Error<LtsStreamAccountTransactionOutcomesPostError>> {
    let local_var_configuration = configuration;

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!("{}/lts/stream/account-transaction-outcomes", local_var_configuration.base_path);
    let mut local_var_req_builder = local_var_client.request(reqwest::Method::POST, local_var_uri_str.as_str());

    if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
        local_var_req_builder = local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }
    local_var_req_builder = local_var_req_builder.json(&lts_stream_account_transaction_outcomes_request);

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        serde_json::from_str(&local_var_content).map_err(Error::from)
    } else {
        let local_var_entity: Option<LtsStreamAccountTransactionOutcomesPostError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

/// Returns a list of committed transaction outcomes (containing balance changes) from a given state version. 
pub async fn lts_stream_transaction_outcomes_post(configuration: &configuration::Configuration, lts_stream_transaction_outcomes_request: models::LtsStreamTransactionOutcomesRequest) -> Result<models::LtsStreamTransactionOutcomesResponse, Error<LtsStreamTransactionOutcomesPostError>> {
    let local_var_configuration = configuration;

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!("{}/lts/stream/transaction-outcomes", local_var_configuration.base_path);
    let mut local_var_req_builder = local_var_client.request(reqwest::Method::POST, local_var_uri_str.as_str());

    if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
        local_var_req_builder = local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }
    local_var_req_builder = local_var_req_builder.json(&lts_stream_transaction_outcomes_request);

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        serde_json::from_str(&local_var_content).map_err(Error::from)
    } else {
        let local_var_entity: Option<LtsStreamTransactionOutcomesPostError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

/// Returns information necessary to build a transaction
pub async fn lts_transaction_construction_post(configuration: &configuration::Configuration, lts_transaction_construction_request: models::LtsTransactionConstructionRequest) -> Result<models::LtsTransactionConstructionResponse, Error<LtsTransactionConstructionPostError>> {
    let local_var_configuration = configuration;

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!("{}/lts/transaction/construction", local_var_configuration.base_path);
    let mut local_var_req_builder = local_var_client.request(reqwest::Method::POST, local_var_uri_str.as_str());

    if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
        local_var_req_builder = local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }
    local_var_req_builder = local_var_req_builder.json(&lts_transaction_construction_request);

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        serde_json::from_str(&local_var_content).map_err(Error::from)
    } else {
        let local_var_entity: Option<LtsTransactionConstructionPostError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

/// Shares the node's knowledge of any payloads associated with the given intent hash. Generally there will be a single payload for a given intent, but it's theoretically possible there may be multiple. This knowledge is summarised into a status for the intent. This summarised status in the response is likely sufficient for most clients. 
pub async fn lts_transaction_status_post(configuration: &configuration::Configuration, lts_transaction_status_request: models::LtsTransactionStatusRequest) -> Result<models::LtsTransactionStatusResponse, Error<LtsTransactionStatusPostError>> {
    let local_var_configuration = configuration;

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!("{}/lts/transaction/status", local_var_configuration.base_path);
    let mut local_var_req_builder = local_var_client.request(reqwest::Method::POST, local_var_uri_str.as_str());

    if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
        local_var_req_builder = local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }
    local_var_req_builder = local_var_req_builder.json(&lts_transaction_status_request);

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        serde_json::from_str(&local_var_content).map_err(Error::from)
    } else {
        let local_var_entity: Option<LtsTransactionStatusPostError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

/// Submits a notarized transaction to the network. Returns whether the transaction submission was already included in the node's mempool. 
pub async fn lts_transaction_submit_post(configuration: &configuration::Configuration, lts_transaction_submit_request: models::LtsTransactionSubmitRequest) -> Result<models::LtsTransactionSubmitResponse, Error<LtsTransactionSubmitPostError>> {
    let local_var_configuration = configuration;

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!("{}/lts/transaction/submit", local_var_configuration.base_path);
    let mut local_var_req_builder = local_var_client.request(reqwest::Method::POST, local_var_uri_str.as_str());

    if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
        local_var_req_builder = local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }
    local_var_req_builder = local_var_req_builder.json(&lts_transaction_submit_request);

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        serde_json::from_str(&local_var_content).map_err(Error::from)
    } else {
        let local_var_entity: Option<LtsTransactionSubmitPostError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

