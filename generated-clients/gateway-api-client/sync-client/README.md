# Rust API client for gateway-api-client-sync

This API is exposed by the Babylon Radix Gateway to enable clients to efficiently query current and historic
state on the RadixDLT ledger, and intelligently handle transaction submission.

It is designed for use by wallets and explorers, and for light queries from front-end dApps.
For exchange/asset integrations, back-end dApp integrations, or simple use cases, you should consider using
the Core API on a Node. A Gateway is only needed for reading historic snapshots of ledger states or a more robust set-up.

The Gateway API is implemented by the [Network Gateway](https://github.com/radixdlt/babylon-gateway),
which is configured to read from [full node(s)](https://github.com/radixdlt/babylon-node) to extract
and index data from the network.

This document is an API reference documentation, visit [User Guide](https://docs.radixdlt.com/) to learn more
about how to run a Gateway of your own.

## Migration guide

Please see [the latest release notes](https://github.com/radixdlt/babylon-gateway/releases).

## Integration and forward compatibility guarantees

All responses may have additional fields added at any release, so clients are advised to use JSON parsers which
ignore unknown fields on JSON objects.

When the Radix protocol is updated, new functionality may be added, and so discriminated unions returned by the
API may need to be updated to have new variants added, corresponding to the updated data. Clients may need to update
in advance to be able to handle these new variants when a protocol update comes out.

On the very rare occasions we need to make breaking changes to the API, these will be warned in advance with
deprecation notices on previous versions. These deprecation notices will include a safe migration path.
Deprecation notes or breaking changes will be flagged clearly in release notes for new versions of the Gateway.

The Gateway DB schema is not subject to any compatibility guarantees, and may be changed at any release. DB changes
will be flagged in the release notes so clients doing custom DB integrations can prepare.



## Overview

This API client was generated by the [OpenAPI Generator](https://openapi-generator.tech) project.  By using the [openapi-spec](https://openapis.org) from a remote server, you can easily generate an API client.

- API version: v1.8.2
- Package version: 1.0.0
- Generator version: 7.6.0
- Build package: `org.openapitools.codegen.languages.RustClientCodegen`

## Installation

Put the package under your project folder in a directory named `gateway-api-client-sync` and add the following to `Cargo.toml` under `[dependencies]`:

```
gateway-api-client-sync = { path = "./gateway-api-client-sync" }
```

## Documentation for API Endpoints

All URIs are relative to *https://mainnet.radixdlt.com*

Class | Method | HTTP request | Description
------------ | ------------- | ------------- | -------------
*ExtensionsApi* | [**resource_holders_page**](docs/ExtensionsApi.md#resource_holders_page) | **POST** /extensions/resource-holders/page | Get Resource Holders Page
*StateApi* | [**account_authorized_depositors_page**](docs/StateApi.md#account_authorized_depositors_page) | **POST** /state/account/page/authorized-depositors | Get Account authorized depositors
*StateApi* | [**account_locker_vaults_page**](docs/StateApi.md#account_locker_vaults_page) | **POST** /state/account-locker/page/vaults | Get Account Locker Vaults Page
*StateApi* | [**account_lockers_touched_at**](docs/StateApi.md#account_lockers_touched_at) | **POST** /state/account-lockers/touched-at | Get Most Recent Touch of Account Lockers
*StateApi* | [**account_resource_preferences_page**](docs/StateApi.md#account_resource_preferences_page) | **POST** /state/account/page/resource-preferences | Get Account resource preferences
*StateApi* | [**entity_fungible_resource_vault_page**](docs/StateApi.md#entity_fungible_resource_vault_page) | **POST** /state/entity/page/fungible-vaults/ | Get page of Global Entity Fungible Resource Vaults
*StateApi* | [**entity_fungibles_page**](docs/StateApi.md#entity_fungibles_page) | **POST** /state/entity/page/fungibles/ | Get page of Global Entity Fungible Resource Balances
*StateApi* | [**entity_metadata_page**](docs/StateApi.md#entity_metadata_page) | **POST** /state/entity/page/metadata | Get Entity Metadata Page
*StateApi* | [**entity_non_fungible_ids_page**](docs/StateApi.md#entity_non_fungible_ids_page) | **POST** /state/entity/page/non-fungible-vault/ids | Get page of Non-Fungibles in Vault
*StateApi* | [**entity_non_fungible_resource_vault_page**](docs/StateApi.md#entity_non_fungible_resource_vault_page) | **POST** /state/entity/page/non-fungible-vaults/ | Get page of Global Entity Non-Fungible Resource Vaults
*StateApi* | [**entity_non_fungibles_page**](docs/StateApi.md#entity_non_fungibles_page) | **POST** /state/entity/page/non-fungibles/ | Get page of Global Entity Non-Fungible Resource Balances
*StateApi* | [**entity_schema_page**](docs/StateApi.md#entity_schema_page) | **POST** /state/entity/page/schemas | Get Entity Schema Page
*StateApi* | [**key_value_store_data**](docs/StateApi.md#key_value_store_data) | **POST** /state/key-value-store/data | Get KeyValueStore Data
*StateApi* | [**key_value_store_keys**](docs/StateApi.md#key_value_store_keys) | **POST** /state/key-value-store/keys | Get KeyValueStore Keys
*StateApi* | [**non_fungible_data**](docs/StateApi.md#non_fungible_data) | **POST** /state/non-fungible/data | Get Non-Fungible Data
*StateApi* | [**non_fungible_ids**](docs/StateApi.md#non_fungible_ids) | **POST** /state/non-fungible/ids | Get page of Non-Fungible Ids in Resource Collection
*StateApi* | [**non_fungible_location**](docs/StateApi.md#non_fungible_location) | **POST** /state/non-fungible/location | Get Non-Fungible Location
*StateApi* | [**package_blueprint_page**](docs/StateApi.md#package_blueprint_page) | **POST** /state/package/page/blueprints | Get Package Blueprints Page
*StateApi* | [**package_code_page**](docs/StateApi.md#package_code_page) | **POST** /state/package/page/codes | Get Package Codes Page
*StateApi* | [**state_entity_details**](docs/StateApi.md#state_entity_details) | **POST** /state/entity/details | Get Entity Details
*StateApi* | [**state_validators_list**](docs/StateApi.md#state_validators_list) | **POST** /state/validators/list | Get Validators List
*StatisticsApi* | [**validators_uptime**](docs/StatisticsApi.md#validators_uptime) | **POST** /statistics/validators/uptime | Get Validators Uptime
*StatusApi* | [**gateway_status**](docs/StatusApi.md#gateway_status) | **POST** /status/gateway-status | Get Gateway Status
*StatusApi* | [**network_configuration**](docs/StatusApi.md#network_configuration) | **POST** /status/network-configuration | Get Network Configuration
*StreamApi* | [**stream_transactions**](docs/StreamApi.md#stream_transactions) | **POST** /stream/transactions | Get Transactions Stream
*TransactionApi* | [**account_deposit_pre_validation**](docs/TransactionApi.md#account_deposit_pre_validation) | **POST** /transaction/account-deposit-pre-validation | PreValidate deposit of resources to an account
*TransactionApi* | [**transaction_committed_details**](docs/TransactionApi.md#transaction_committed_details) | **POST** /transaction/committed-details | Get Committed Transaction Details
*TransactionApi* | [**transaction_construction**](docs/TransactionApi.md#transaction_construction) | **POST** /transaction/construction | Get Construction Metadata
*TransactionApi* | [**transaction_preview**](docs/TransactionApi.md#transaction_preview) | **POST** /transaction/preview | Preview Transaction
*TransactionApi* | [**transaction_status**](docs/TransactionApi.md#transaction_status) | **POST** /transaction/status | Get Transaction Status
*TransactionApi* | [**transaction_submit**](docs/TransactionApi.md#transaction_submit) | **POST** /transaction/submit | Submit Transaction


## Documentation For Models

 - [AccountAuthorizedDepositorBadgeType](docs/AccountAuthorizedDepositorBadgeType.md)
 - [AccountAuthorizedDepositorsCollection](docs/AccountAuthorizedDepositorsCollection.md)
 - [AccountAuthorizedDepositorsNonFungibleBadge](docs/AccountAuthorizedDepositorsNonFungibleBadge.md)
 - [AccountAuthorizedDepositorsResourceBadge](docs/AccountAuthorizedDepositorsResourceBadge.md)
 - [AccountAuthorizedDepositorsResponseItem](docs/AccountAuthorizedDepositorsResponseItem.md)
 - [AccountDefaultDepositRule](docs/AccountDefaultDepositRule.md)
 - [AccountDepositPreValidationDecidingFactors](docs/AccountDepositPreValidationDecidingFactors.md)
 - [AccountDepositPreValidationDecidingFactorsResourceSpecificDetailsItem](docs/AccountDepositPreValidationDecidingFactorsResourceSpecificDetailsItem.md)
 - [AccountDepositPreValidationNonFungibleBadge](docs/AccountDepositPreValidationNonFungibleBadge.md)
 - [AccountDepositPreValidationRequest](docs/AccountDepositPreValidationRequest.md)
 - [AccountDepositPreValidationResourceBadge](docs/AccountDepositPreValidationResourceBadge.md)
 - [AccountDepositPreValidationResourceSpecificBehaviourItem](docs/AccountDepositPreValidationResourceSpecificBehaviourItem.md)
 - [AccountDepositPreValidationResponse](docs/AccountDepositPreValidationResponse.md)
 - [AccountLockerAddress](docs/AccountLockerAddress.md)
 - [AccountLockerVaultCollection](docs/AccountLockerVaultCollection.md)
 - [AccountLockerVaultCollectionItem](docs/AccountLockerVaultCollectionItem.md)
 - [AccountLockerVaultCollectionItemFungible](docs/AccountLockerVaultCollectionItemFungible.md)
 - [AccountLockerVaultCollectionItemNonFungible](docs/AccountLockerVaultCollectionItemNonFungible.md)
 - [AccountLockerVaultCollectionItemType](docs/AccountLockerVaultCollectionItemType.md)
 - [AccountResourcePreferenceRule](docs/AccountResourcePreferenceRule.md)
 - [AccountResourcePreferencesCollection](docs/AccountResourcePreferencesCollection.md)
 - [AccountResourcePreferencesResponseItem](docs/AccountResourcePreferencesResponseItem.md)
 - [AtLedgerStateMixin](docs/AtLedgerStateMixin.md)
 - [BlueprintMethodRoyalty](docs/BlueprintMethodRoyalty.md)
 - [BlueprintRoyaltyConfig](docs/BlueprintRoyaltyConfig.md)
 - [CommittedTransactionInfo](docs/CommittedTransactionInfo.md)
 - [ComponentEntityRoleAssignmentEntry](docs/ComponentEntityRoleAssignmentEntry.md)
 - [ComponentEntityRoleAssignmentEntryAssignment](docs/ComponentEntityRoleAssignmentEntryAssignment.md)
 - [ComponentEntityRoleAssignments](docs/ComponentEntityRoleAssignments.md)
 - [ComponentMethodRoyalty](docs/ComponentMethodRoyalty.md)
 - [ComponentRoyaltyConfig](docs/ComponentRoyaltyConfig.md)
 - [CursorLimitMixin](docs/CursorLimitMixin.md)
 - [EntityMetadataCollection](docs/EntityMetadataCollection.md)
 - [EntityMetadataItem](docs/EntityMetadataItem.md)
 - [EntityMetadataItemValue](docs/EntityMetadataItemValue.md)
 - [EntityNotFoundError](docs/EntityNotFoundError.md)
 - [EntitySchemaCollection](docs/EntitySchemaCollection.md)
 - [EntitySchemaCollectionItem](docs/EntitySchemaCollectionItem.md)
 - [ErrorResponse](docs/ErrorResponse.md)
 - [EventsItem](docs/EventsItem.md)
 - [FromLedgerStateMixin](docs/FromLedgerStateMixin.md)
 - [FungibleResourcesCollection](docs/FungibleResourcesCollection.md)
 - [FungibleResourcesCollectionItem](docs/FungibleResourcesCollectionItem.md)
 - [FungibleResourcesCollectionItemGloballyAggregated](docs/FungibleResourcesCollectionItemGloballyAggregated.md)
 - [FungibleResourcesCollectionItemVaultAggregated](docs/FungibleResourcesCollectionItemVaultAggregated.md)
 - [FungibleResourcesCollectionItemVaultAggregatedVault](docs/FungibleResourcesCollectionItemVaultAggregatedVault.md)
 - [FungibleResourcesCollectionItemVaultAggregatedVaultItem](docs/FungibleResourcesCollectionItemVaultAggregatedVaultItem.md)
 - [GatewayError](docs/GatewayError.md)
 - [GatewayInfoResponseKnownTarget](docs/GatewayInfoResponseKnownTarget.md)
 - [GatewayInfoResponseReleaseInfo](docs/GatewayInfoResponseReleaseInfo.md)
 - [GatewayStatusResponse](docs/GatewayStatusResponse.md)
 - [InternalServerError](docs/InternalServerError.md)
 - [InvalidEntityError](docs/InvalidEntityError.md)
 - [InvalidRequestError](docs/InvalidRequestError.md)
 - [LedgerState](docs/LedgerState.md)
 - [LedgerStateMixin](docs/LedgerStateMixin.md)
 - [LedgerStateSelector](docs/LedgerStateSelector.md)
 - [ManifestClass](docs/ManifestClass.md)
 - [MetadataBoolArrayValue](docs/MetadataBoolArrayValue.md)
 - [MetadataBoolValue](docs/MetadataBoolValue.md)
 - [MetadataDecimalArrayValue](docs/MetadataDecimalArrayValue.md)
 - [MetadataDecimalValue](docs/MetadataDecimalValue.md)
 - [MetadataGlobalAddressArrayValue](docs/MetadataGlobalAddressArrayValue.md)
 - [MetadataGlobalAddressValue](docs/MetadataGlobalAddressValue.md)
 - [MetadataI32ArrayValue](docs/MetadataI32ArrayValue.md)
 - [MetadataI32Value](docs/MetadataI32Value.md)
 - [MetadataI64ArrayValue](docs/MetadataI64ArrayValue.md)
 - [MetadataI64Value](docs/MetadataI64Value.md)
 - [MetadataInstantArrayValue](docs/MetadataInstantArrayValue.md)
 - [MetadataInstantValue](docs/MetadataInstantValue.md)
 - [MetadataNonFungibleGlobalIdArrayValue](docs/MetadataNonFungibleGlobalIdArrayValue.md)
 - [MetadataNonFungibleGlobalIdArrayValueValuesInner](docs/MetadataNonFungibleGlobalIdArrayValueValuesInner.md)
 - [MetadataNonFungibleGlobalIdValue](docs/MetadataNonFungibleGlobalIdValue.md)
 - [MetadataNonFungibleLocalIdArrayValue](docs/MetadataNonFungibleLocalIdArrayValue.md)
 - [MetadataNonFungibleLocalIdValue](docs/MetadataNonFungibleLocalIdValue.md)
 - [MetadataOriginArrayValue](docs/MetadataOriginArrayValue.md)
 - [MetadataOriginValue](docs/MetadataOriginValue.md)
 - [MetadataPublicKeyArrayValue](docs/MetadataPublicKeyArrayValue.md)
 - [MetadataPublicKeyHashArrayValue](docs/MetadataPublicKeyHashArrayValue.md)
 - [MetadataPublicKeyHashValue](docs/MetadataPublicKeyHashValue.md)
 - [MetadataPublicKeyValue](docs/MetadataPublicKeyValue.md)
 - [MetadataStringArrayValue](docs/MetadataStringArrayValue.md)
 - [MetadataStringValue](docs/MetadataStringValue.md)
 - [MetadataTypedValue](docs/MetadataTypedValue.md)
 - [MetadataU32ArrayValue](docs/MetadataU32ArrayValue.md)
 - [MetadataU32Value](docs/MetadataU32Value.md)
 - [MetadataU64ArrayValue](docs/MetadataU64ArrayValue.md)
 - [MetadataU64Value](docs/MetadataU64Value.md)
 - [MetadataU8ArrayValue](docs/MetadataU8ArrayValue.md)
 - [MetadataU8Value](docs/MetadataU8Value.md)
 - [MetadataUrlArrayValue](docs/MetadataUrlArrayValue.md)
 - [MetadataUrlValue](docs/MetadataUrlValue.md)
 - [MetadataValueType](docs/MetadataValueType.md)
 - [NativeResourceAccessControllerRecoveryBadgeValue](docs/NativeResourceAccessControllerRecoveryBadgeValue.md)
 - [NativeResourceDetails](docs/NativeResourceDetails.md)
 - [NativeResourceKind](docs/NativeResourceKind.md)
 - [NativeResourceMultiResourcePoolUnitValue](docs/NativeResourceMultiResourcePoolUnitValue.md)
 - [NativeResourceOneResourcePoolUnitValue](docs/NativeResourceOneResourcePoolUnitValue.md)
 - [NativeResourceRedemptionValueItem](docs/NativeResourceRedemptionValueItem.md)
 - [NativeResourceTwoResourcePoolUnitValue](docs/NativeResourceTwoResourcePoolUnitValue.md)
 - [NativeResourceValidatorClaimNftValue](docs/NativeResourceValidatorClaimNftValue.md)
 - [NativeResourceValidatorLiquidStakeUnitValue](docs/NativeResourceValidatorLiquidStakeUnitValue.md)
 - [NetworkConfigurationResponse](docs/NetworkConfigurationResponse.md)
 - [NetworkConfigurationResponseWellKnownAddresses](docs/NetworkConfigurationResponseWellKnownAddresses.md)
 - [NonFungibleIdType](docs/NonFungibleIdType.md)
 - [NonFungibleIdsCollection](docs/NonFungibleIdsCollection.md)
 - [NonFungibleResourcesCollection](docs/NonFungibleResourcesCollection.md)
 - [NonFungibleResourcesCollectionItem](docs/NonFungibleResourcesCollectionItem.md)
 - [NonFungibleResourcesCollectionItemGloballyAggregated](docs/NonFungibleResourcesCollectionItemGloballyAggregated.md)
 - [NonFungibleResourcesCollectionItemVaultAggregated](docs/NonFungibleResourcesCollectionItemVaultAggregated.md)
 - [NonFungibleResourcesCollectionItemVaultAggregatedVault](docs/NonFungibleResourcesCollectionItemVaultAggregatedVault.md)
 - [NonFungibleResourcesCollectionItemVaultAggregatedVaultItem](docs/NonFungibleResourcesCollectionItemVaultAggregatedVaultItem.md)
 - [NotSyncedUpError](docs/NotSyncedUpError.md)
 - [ObjectModuleId](docs/ObjectModuleId.md)
 - [OptionalNonFungibleIdsCollection](docs/OptionalNonFungibleIdsCollection.md)
 - [PackageBlueprintCollection](docs/PackageBlueprintCollection.md)
 - [PackageBlueprintCollectionItem](docs/PackageBlueprintCollectionItem.md)
 - [PackageCodeCollection](docs/PackageCodeCollection.md)
 - [PackageCodeCollectionItem](docs/PackageCodeCollectionItem.md)
 - [PackageVmType](docs/PackageVmType.md)
 - [ProgrammaticScryptoSborValue](docs/ProgrammaticScryptoSborValue.md)
 - [ProgrammaticScryptoSborValueArray](docs/ProgrammaticScryptoSborValueArray.md)
 - [ProgrammaticScryptoSborValueBool](docs/ProgrammaticScryptoSborValueBool.md)
 - [ProgrammaticScryptoSborValueBytes](docs/ProgrammaticScryptoSborValueBytes.md)
 - [ProgrammaticScryptoSborValueDecimal](docs/ProgrammaticScryptoSborValueDecimal.md)
 - [ProgrammaticScryptoSborValueEnum](docs/ProgrammaticScryptoSborValueEnum.md)
 - [ProgrammaticScryptoSborValueI128](docs/ProgrammaticScryptoSborValueI128.md)
 - [ProgrammaticScryptoSborValueI16](docs/ProgrammaticScryptoSborValueI16.md)
 - [ProgrammaticScryptoSborValueI32](docs/ProgrammaticScryptoSborValueI32.md)
 - [ProgrammaticScryptoSborValueI64](docs/ProgrammaticScryptoSborValueI64.md)
 - [ProgrammaticScryptoSborValueI8](docs/ProgrammaticScryptoSborValueI8.md)
 - [ProgrammaticScryptoSborValueKind](docs/ProgrammaticScryptoSborValueKind.md)
 - [ProgrammaticScryptoSborValueMap](docs/ProgrammaticScryptoSborValueMap.md)
 - [ProgrammaticScryptoSborValueMapEntry](docs/ProgrammaticScryptoSborValueMapEntry.md)
 - [ProgrammaticScryptoSborValueNonFungibleLocalId](docs/ProgrammaticScryptoSborValueNonFungibleLocalId.md)
 - [ProgrammaticScryptoSborValueOwn](docs/ProgrammaticScryptoSborValueOwn.md)
 - [ProgrammaticScryptoSborValuePreciseDecimal](docs/ProgrammaticScryptoSborValuePreciseDecimal.md)
 - [ProgrammaticScryptoSborValueReference](docs/ProgrammaticScryptoSborValueReference.md)
 - [ProgrammaticScryptoSborValueString](docs/ProgrammaticScryptoSborValueString.md)
 - [ProgrammaticScryptoSborValueTuple](docs/ProgrammaticScryptoSborValueTuple.md)
 - [ProgrammaticScryptoSborValueU128](docs/ProgrammaticScryptoSborValueU128.md)
 - [ProgrammaticScryptoSborValueU16](docs/ProgrammaticScryptoSborValueU16.md)
 - [ProgrammaticScryptoSborValueU32](docs/ProgrammaticScryptoSborValueU32.md)
 - [ProgrammaticScryptoSborValueU64](docs/ProgrammaticScryptoSborValueU64.md)
 - [ProgrammaticScryptoSborValueU8](docs/ProgrammaticScryptoSborValueU8.md)
 - [PublicKey](docs/PublicKey.md)
 - [PublicKeyEcdsaSecp256k1](docs/PublicKeyEcdsaSecp256k1.md)
 - [PublicKeyEddsaEd25519](docs/PublicKeyEddsaEd25519.md)
 - [PublicKeyHash](docs/PublicKeyHash.md)
 - [PublicKeyHashEcdsaSecp256k1](docs/PublicKeyHashEcdsaSecp256k1.md)
 - [PublicKeyHashEddsaEd25519](docs/PublicKeyHashEddsaEd25519.md)
 - [PublicKeyHashType](docs/PublicKeyHashType.md)
 - [PublicKeyType](docs/PublicKeyType.md)
 - [ResourceAggregationLevel](docs/ResourceAggregationLevel.md)
 - [ResourceHoldersCollection](docs/ResourceHoldersCollection.md)
 - [ResourceHoldersCollectionFungibleResourceItem](docs/ResourceHoldersCollectionFungibleResourceItem.md)
 - [ResourceHoldersCollectionItem](docs/ResourceHoldersCollectionItem.md)
 - [ResourceHoldersCollectionNonFungibleResourceItem](docs/ResourceHoldersCollectionNonFungibleResourceItem.md)
 - [ResourceHoldersRequest](docs/ResourceHoldersRequest.md)
 - [ResourceHoldersResourceType](docs/ResourceHoldersResourceType.md)
 - [ResourceHoldersResponse](docs/ResourceHoldersResponse.md)
 - [ResultSetCursorMixin](docs/ResultSetCursorMixin.md)
 - [RoleAssignmentResolution](docs/RoleAssignmentResolution.md)
 - [RoleKey](docs/RoleKey.md)
 - [RoyaltyAmount](docs/RoyaltyAmount.md)
 - [ScryptoSborValue](docs/ScryptoSborValue.md)
 - [StateAccountAuthorizedDepositorsPageRequest](docs/StateAccountAuthorizedDepositorsPageRequest.md)
 - [StateAccountAuthorizedDepositorsPageResponse](docs/StateAccountAuthorizedDepositorsPageResponse.md)
 - [StateAccountLockerPageVaultsRequest](docs/StateAccountLockerPageVaultsRequest.md)
 - [StateAccountLockerPageVaultsResponse](docs/StateAccountLockerPageVaultsResponse.md)
 - [StateAccountLockersTouchedAtRequest](docs/StateAccountLockersTouchedAtRequest.md)
 - [StateAccountLockersTouchedAtResponse](docs/StateAccountLockersTouchedAtResponse.md)
 - [StateAccountLockersTouchedAtResponseItem](docs/StateAccountLockersTouchedAtResponseItem.md)
 - [StateAccountResourcePreferencesPageRequest](docs/StateAccountResourcePreferencesPageRequest.md)
 - [StateAccountResourcePreferencesPageResponse](docs/StateAccountResourcePreferencesPageResponse.md)
 - [StateEntityDetailsOptIns](docs/StateEntityDetailsOptIns.md)
 - [StateEntityDetailsRequest](docs/StateEntityDetailsRequest.md)
 - [StateEntityDetailsResponse](docs/StateEntityDetailsResponse.md)
 - [StateEntityDetailsResponseComponentDetails](docs/StateEntityDetailsResponseComponentDetails.md)
 - [StateEntityDetailsResponseFungibleResourceDetails](docs/StateEntityDetailsResponseFungibleResourceDetails.md)
 - [StateEntityDetailsResponseFungibleVaultDetails](docs/StateEntityDetailsResponseFungibleVaultDetails.md)
 - [StateEntityDetailsResponseItem](docs/StateEntityDetailsResponseItem.md)
 - [StateEntityDetailsResponseItemAncestorIdentities](docs/StateEntityDetailsResponseItemAncestorIdentities.md)
 - [StateEntityDetailsResponseItemDetails](docs/StateEntityDetailsResponseItemDetails.md)
 - [StateEntityDetailsResponseItemDetailsType](docs/StateEntityDetailsResponseItemDetailsType.md)
 - [StateEntityDetailsResponseNonFungibleResourceDetails](docs/StateEntityDetailsResponseNonFungibleResourceDetails.md)
 - [StateEntityDetailsResponseNonFungibleVaultDetails](docs/StateEntityDetailsResponseNonFungibleVaultDetails.md)
 - [StateEntityDetailsResponsePackageDetails](docs/StateEntityDetailsResponsePackageDetails.md)
 - [StateEntityFungibleResourceVaultsPageRequest](docs/StateEntityFungibleResourceVaultsPageRequest.md)
 - [StateEntityFungibleResourceVaultsPageResponse](docs/StateEntityFungibleResourceVaultsPageResponse.md)
 - [StateEntityFungiblesPageRequest](docs/StateEntityFungiblesPageRequest.md)
 - [StateEntityFungiblesPageRequestOptIns](docs/StateEntityFungiblesPageRequestOptIns.md)
 - [StateEntityFungiblesPageResponse](docs/StateEntityFungiblesPageResponse.md)
 - [StateEntityMetadataPageRequest](docs/StateEntityMetadataPageRequest.md)
 - [StateEntityMetadataPageResponse](docs/StateEntityMetadataPageResponse.md)
 - [StateEntityNonFungibleIdsPageRequest](docs/StateEntityNonFungibleIdsPageRequest.md)
 - [StateEntityNonFungibleIdsPageResponse](docs/StateEntityNonFungibleIdsPageResponse.md)
 - [StateEntityNonFungibleResourceVaultsPageOptIns](docs/StateEntityNonFungibleResourceVaultsPageOptIns.md)
 - [StateEntityNonFungibleResourceVaultsPageRequest](docs/StateEntityNonFungibleResourceVaultsPageRequest.md)
 - [StateEntityNonFungibleResourceVaultsPageResponse](docs/StateEntityNonFungibleResourceVaultsPageResponse.md)
 - [StateEntityNonFungiblesPageRequest](docs/StateEntityNonFungiblesPageRequest.md)
 - [StateEntityNonFungiblesPageRequestOptIns](docs/StateEntityNonFungiblesPageRequestOptIns.md)
 - [StateEntityNonFungiblesPageResponse](docs/StateEntityNonFungiblesPageResponse.md)
 - [StateEntitySchemaPageRequest](docs/StateEntitySchemaPageRequest.md)
 - [StateEntitySchemaPageResponse](docs/StateEntitySchemaPageResponse.md)
 - [StateKeyValueStoreDataRequest](docs/StateKeyValueStoreDataRequest.md)
 - [StateKeyValueStoreDataRequestKeyItem](docs/StateKeyValueStoreDataRequestKeyItem.md)
 - [StateKeyValueStoreDataResponse](docs/StateKeyValueStoreDataResponse.md)
 - [StateKeyValueStoreDataResponseItem](docs/StateKeyValueStoreDataResponseItem.md)
 - [StateKeyValueStoreKeysCollection](docs/StateKeyValueStoreKeysCollection.md)
 - [StateKeyValueStoreKeysRequest](docs/StateKeyValueStoreKeysRequest.md)
 - [StateKeyValueStoreKeysResponse](docs/StateKeyValueStoreKeysResponse.md)
 - [StateKeyValueStoreKeysResponseItem](docs/StateKeyValueStoreKeysResponseItem.md)
 - [StateNonFungibleDataRequest](docs/StateNonFungibleDataRequest.md)
 - [StateNonFungibleDataResponse](docs/StateNonFungibleDataResponse.md)
 - [StateNonFungibleDetailsResponseItem](docs/StateNonFungibleDetailsResponseItem.md)
 - [StateNonFungibleIdsRequest](docs/StateNonFungibleIdsRequest.md)
 - [StateNonFungibleIdsResponse](docs/StateNonFungibleIdsResponse.md)
 - [StateNonFungibleLocationRequest](docs/StateNonFungibleLocationRequest.md)
 - [StateNonFungibleLocationResponse](docs/StateNonFungibleLocationResponse.md)
 - [StateNonFungibleLocationResponseItem](docs/StateNonFungibleLocationResponseItem.md)
 - [StatePackageBlueprintPageRequest](docs/StatePackageBlueprintPageRequest.md)
 - [StatePackageBlueprintPageResponse](docs/StatePackageBlueprintPageResponse.md)
 - [StatePackageCodePageRequest](docs/StatePackageCodePageRequest.md)
 - [StatePackageCodePageResponse](docs/StatePackageCodePageResponse.md)
 - [StateValidatorsListRequest](docs/StateValidatorsListRequest.md)
 - [StateValidatorsListResponse](docs/StateValidatorsListResponse.md)
 - [StreamTransactionsRequest](docs/StreamTransactionsRequest.md)
 - [StreamTransactionsRequestAllOfManifestClassFilter](docs/StreamTransactionsRequestAllOfManifestClassFilter.md)
 - [StreamTransactionsRequestEventFilterItem](docs/StreamTransactionsRequestEventFilterItem.md)
 - [StreamTransactionsResponse](docs/StreamTransactionsResponse.md)
 - [TransactionAccountDepositPreValidationAuthorizedDepositorBadge](docs/TransactionAccountDepositPreValidationAuthorizedDepositorBadge.md)
 - [TransactionBalanceChanges](docs/TransactionBalanceChanges.md)
 - [TransactionCommittedDetailsRequest](docs/TransactionCommittedDetailsRequest.md)
 - [TransactionCommittedDetailsResponse](docs/TransactionCommittedDetailsResponse.md)
 - [TransactionConstructionResponse](docs/TransactionConstructionResponse.md)
 - [TransactionDetailsOptIns](docs/TransactionDetailsOptIns.md)
 - [TransactionFungibleBalanceChanges](docs/TransactionFungibleBalanceChanges.md)
 - [TransactionFungibleFeeBalanceChangeType](docs/TransactionFungibleFeeBalanceChangeType.md)
 - [TransactionFungibleFeeBalanceChanges](docs/TransactionFungibleFeeBalanceChanges.md)
 - [TransactionIntentStatus](docs/TransactionIntentStatus.md)
 - [TransactionNonFungibleBalanceChanges](docs/TransactionNonFungibleBalanceChanges.md)
 - [TransactionNotFoundError](docs/TransactionNotFoundError.md)
 - [TransactionPayloadGatewayHandlingStatus](docs/TransactionPayloadGatewayHandlingStatus.md)
 - [TransactionPayloadStatus](docs/TransactionPayloadStatus.md)
 - [TransactionPreviewOptIns](docs/TransactionPreviewOptIns.md)
 - [TransactionPreviewRequest](docs/TransactionPreviewRequest.md)
 - [TransactionPreviewRequestFlags](docs/TransactionPreviewRequestFlags.md)
 - [TransactionPreviewResponse](docs/TransactionPreviewResponse.md)
 - [TransactionPreviewResponseLogsInner](docs/TransactionPreviewResponseLogsInner.md)
 - [TransactionReceipt](docs/TransactionReceipt.md)
 - [TransactionStatus](docs/TransactionStatus.md)
 - [TransactionStatusRequest](docs/TransactionStatusRequest.md)
 - [TransactionStatusResponse](docs/TransactionStatusResponse.md)
 - [TransactionStatusResponseKnownPayloadItem](docs/TransactionStatusResponseKnownPayloadItem.md)
 - [TransactionSubmitRequest](docs/TransactionSubmitRequest.md)
 - [TransactionSubmitResponse](docs/TransactionSubmitResponse.md)
 - [TwoWayLinkedDappOnLedgerDetails](docs/TwoWayLinkedDappOnLedgerDetails.md)
 - [TwoWayLinkedDappsCollection](docs/TwoWayLinkedDappsCollection.md)
 - [TwoWayLinkedDappsCollectionItem](docs/TwoWayLinkedDappsCollectionItem.md)
 - [TwoWayLinkedEntitiesCollection](docs/TwoWayLinkedEntitiesCollection.md)
 - [TwoWayLinkedEntitiesCollectionItem](docs/TwoWayLinkedEntitiesCollectionItem.md)
 - [ValidationErrorsAtPath](docs/ValidationErrorsAtPath.md)
 - [ValidatorCollection](docs/ValidatorCollection.md)
 - [ValidatorCollectionItem](docs/ValidatorCollectionItem.md)
 - [ValidatorCollectionItemActiveInEpoch](docs/ValidatorCollectionItemActiveInEpoch.md)
 - [ValidatorCollectionItemEffectiveFeeFactor](docs/ValidatorCollectionItemEffectiveFeeFactor.md)
 - [ValidatorCollectionItemEffectiveFeeFactorCurrent](docs/ValidatorCollectionItemEffectiveFeeFactorCurrent.md)
 - [ValidatorCollectionItemEffectiveFeeFactorPending](docs/ValidatorCollectionItemEffectiveFeeFactorPending.md)
 - [ValidatorUptimeCollection](docs/ValidatorUptimeCollection.md)
 - [ValidatorUptimeCollectionItem](docs/ValidatorUptimeCollectionItem.md)
 - [ValidatorVaultItem](docs/ValidatorVaultItem.md)
 - [ValidatorsUptimeRequest](docs/ValidatorsUptimeRequest.md)
 - [ValidatorsUptimeResponse](docs/ValidatorsUptimeResponse.md)


To get access to the crate's generated documentation, use:

```
cargo doc --open
```

## Author



