/*
 * Radix Gateway API - Babylon
 *
 * This API is exposed by the Babylon Radix Gateway to enable clients to efficiently query current and historic state on the RadixDLT ledger, and intelligently handle transaction submission.  It is designed for use by wallets and explorers, and for light queries from front-end dApps. For exchange/asset integrations, back-end dApp integrations, or simple use cases, you should consider using the Core API on a Node. A Gateway is only needed for reading historic snapshots of ledger states or a more robust set-up.  The Gateway API is implemented by the [Network Gateway](https://github.com/radixdlt/babylon-gateway), which is configured to read from [full node(s)](https://github.com/radixdlt/babylon-node) to extract and index data from the network.  This document is an API reference documentation, visit [User Guide](https://docs.radixdlt.com/) to learn more about how to run a Gateway of your own.  ## Migration guide  Please see [the latest release notes](https://github.com/radixdlt/babylon-gateway/releases).  ## Integration and forward compatibility guarantees  All responses may have additional fields added at any release, so clients are advised to use JSON parsers which ignore unknown fields on JSON objects.  When the Radix protocol is updated, new functionality may be added, and so discriminated unions returned by the API may need to be updated to have new variants added, corresponding to the updated data. Clients may need to update in advance to be able to handle these new variants when a protocol update comes out.  On the very rare occasions we need to make breaking changes to the API, these will be warned in advance with deprecation notices on previous versions. These deprecation notices will include a safe migration path. Deprecation notes or breaking changes will be flagged clearly in release notes for new versions of the Gateway.  The Gateway DB schema is not subject to any compatibility guarantees, and may be changed at any release. DB changes will be flagged in the release notes so clients doing custom DB integrations can prepare. 
 *
 * The version of the OpenAPI document: v1.8.2
 * 
 * Generated by: https://openapi-generator.tech
 */

use crate::models;
use serde::{Deserialize, Serialize};

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct StateEntityDetailsResponseComponentDetails {
    #[serde(rename = "blueprint_name")]
    pub blueprint_name: String,
    #[serde(rename = "blueprint_version")]
    pub blueprint_version: String,
    #[serde(rename = "native_resource_details", skip_serializing_if = "Option::is_none")]
    pub native_resource_details: Option<Box<models::NativeResourceDetails>>,
    /// Bech32m-encoded human readable version of the address.
    #[serde(rename = "package_address", skip_serializing_if = "Option::is_none")]
    pub package_address: Option<String>,
    #[serde(rename = "role_assignments", skip_serializing_if = "Option::is_none")]
    pub role_assignments: Option<Box<models::ComponentEntityRoleAssignments>>,
    #[serde(rename = "royalty_config", skip_serializing_if = "Option::is_none")]
    pub royalty_config: Option<Box<models::ComponentRoyaltyConfig>>,
    /// String-encoded decimal representing the amount of a related fungible resource.
    #[serde(rename = "royalty_vault_balance", skip_serializing_if = "Option::is_none")]
    pub royalty_vault_balance: Option<String>,
    /// A representation of a component's inner state. If this entity is a `GenericComponent`, this field will be in a programmatic JSON structure (you can deserialize it as a `ProgrammaticScryptoSborValue`). Otherwise, for \"native\" components such as `Account`, `Validator`, `AccessController`, `OneResourcePool`, `TwoResourcePool`, and `MultiResourcePool`, this field will be a custom JSON model defined in the Core API schema. 
    #[serde(rename = "state", skip_serializing_if = "Option::is_none")]
    pub state: Option<serde_json::Value>,
    /// Bech32m-encoded human readable version of the address.
    #[serde(rename = "two_way_linked_dapp_address", skip_serializing_if = "Option::is_none")]
    pub two_way_linked_dapp_address: Option<String>,
    #[serde(rename = "two_way_linked_dapp_details", skip_serializing_if = "Option::is_none")]
    pub two_way_linked_dapp_details: Option<Box<models::TwoWayLinkedDappOnLedgerDetails>>,
}

impl StateEntityDetailsResponseComponentDetails {
    pub fn new(blueprint_name: String, blueprint_version: String) -> StateEntityDetailsResponseComponentDetails {
        StateEntityDetailsResponseComponentDetails {
            blueprint_name,
            blueprint_version,
            native_resource_details: None,
            package_address: None,
            role_assignments: None,
            royalty_config: None,
            royalty_vault_balance: None,
            state: None,
            two_way_linked_dapp_address: None,
            two_way_linked_dapp_details: None,
        }
    }
}

