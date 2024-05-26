/*
 * Radix Gateway API - Babylon
 *
 * This API is exposed by the Babylon Radix Gateway to enable clients to efficiently query current and historic state on the RadixDLT ledger, and intelligently handle transaction submission.  It is designed for use by wallets and explorers, and for light queries from front-end dApps. For exchange/asset integrations, back-end dApp integrations, or simple use cases, you should consider using the Core API on a Node. A Gateway is only needed for reading historic snapshots of ledger states or a more robust set-up.  The Gateway API is implemented by the [Network Gateway](https://github.com/radixdlt/babylon-gateway), which is configured to read from [full node(s)](https://github.com/radixdlt/babylon-node) to extract and index data from the network.  This document is an API reference documentation, visit [User Guide](https://docs.radixdlt.com/) to learn more about how to run a Gateway of your own.  ## Migration guide  Please see [the latest release notes](https://github.com/radixdlt/babylon-gateway/releases).  ## Integration and forward compatibility guarantees  All responses may have additional fields added at any release, so clients are advised to use JSON parsers which ignore unknown fields on JSON objects.  When the Radix protocol is updated, new functionality may be added, and so discriminated unions returned by the API may need to be updated to have new variants added, corresponding to the updated data. Clients may need to update in advance to be able to handle these new variants when a protocol update comes out.  On the very rare occasions we need to make breaking changes to the API, these will be warned in advance with deprecation notices on previous versions. These deprecation notices will include a safe migration path. Deprecation notes or breaking changes will be flagged clearly in release notes for new versions of the Gateway.  The Gateway DB schema is not subject to any compatibility guarantees, and may be changed at any release. DB changes will be flagged in the release notes so clients doing custom DB integrations can prepare. 
 *
 * The version of the OpenAPI document: v1.6.1
 * 
 * Generated by: https://openapi-generator.tech
 */

use crate::models;
use serde::{Deserialize, Serialize};

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct TransactionPreviewRequest {
    /// An array of hex-encoded blob data (optional)
    #[serde(rename = "blobs_hex", skip_serializing_if = "Option::is_none")]
    pub blobs_hex: Option<Vec<String>>,
    /// An integer between `0` and `10^10`, marking the epoch at which the transaction is no longer valid
    #[serde(rename = "end_epoch_exclusive")]
    pub end_epoch_exclusive: u64,
    #[serde(rename = "flags")]
    pub flags: Box<models::TransactionPreviewRequestFlags>,
    /// A text-representation of a transaction manifest
    #[serde(rename = "manifest")]
    pub manifest: String,
    /// A decimal-string-encoded integer between `0` and `2^32 - 1`, used to ensure the transaction intent is unique.
    #[serde(rename = "nonce")]
    pub nonce: u64,
    /// Whether the notary should count as a signatory (optional, default false)
    #[serde(rename = "notary_is_signatory", skip_serializing_if = "Option::is_none")]
    pub notary_is_signatory: Option<bool>,
    #[serde(rename = "notary_public_key", skip_serializing_if = "Option::is_none")]
    pub notary_public_key: Option<Box<models::PublicKey>>,
    /// A list of public keys to be used as transaction signers
    #[serde(rename = "signer_public_keys")]
    pub signer_public_keys: Vec<models::PublicKey>,
    /// An integer between `0` and `10^10`, marking the epoch at which the transaction starts being valid
    #[serde(rename = "start_epoch_inclusive")]
    pub start_epoch_inclusive: u64,
    /// An integer between `0` and `65535`, giving the validator tip as a percentage amount. A value of `1` corresponds to 1% of the fee.
    #[serde(rename = "tip_percentage")]
    pub tip_percentage: u32,
}

impl TransactionPreviewRequest {
    pub fn new(end_epoch_exclusive: u64, flags: models::TransactionPreviewRequestFlags, manifest: String, nonce: u64, signer_public_keys: Vec<models::PublicKey>, start_epoch_inclusive: u64, tip_percentage: u32) -> TransactionPreviewRequest {
        TransactionPreviewRequest {
            blobs_hex: None,
            end_epoch_exclusive,
            flags: Box::new(flags),
            manifest,
            nonce,
            notary_is_signatory: None,
            notary_public_key: None,
            signer_public_keys,
            start_epoch_inclusive,
            tip_percentage,
        }
    }
}

