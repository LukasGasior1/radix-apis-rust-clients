# StateEntityDetailsResponsePackageDetails

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**r#type** | [**models::StateEntityDetailsResponseItemDetailsType**](StateEntityDetailsResponseItemDetailsType.md) |  | 
**blueprints** | Option<[**models::PackageBlueprintCollection**](PackageBlueprintCollection.md)> |  | [optional]
**code_hash_hex** | **String** | Hex-encoded binary blob. | 
**code_hex** | **String** | Hex-encoded binary blob. | 
**codes** | [**models::PackageCodeCollection**](PackageCodeCollection.md) |  | 
**role_assignments** | Option<[**models::ComponentEntityRoleAssignments**](ComponentEntityRoleAssignments.md)> |  | [optional]
**royalty_vault_balance** | Option<**String**> | String-encoded decimal representing the amount of a related fungible resource. | [optional]
**schemas** | Option<[**models::EntitySchemaCollection**](EntitySchemaCollection.md)> |  | [optional]
**two_way_linked_dapp_address** | Option<**String**> | Bech32m-encoded human readable version of the address. | [optional]
**vm_type** | [**models::PackageVmType**](PackageVmType.md) |  | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


