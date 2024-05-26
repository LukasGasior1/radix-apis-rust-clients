# StateEntityDetailsResponseComponentDetails

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**r#type** | [**models::StateEntityDetailsResponseItemDetailsType**](StateEntityDetailsResponseItemDetailsType.md) |  | 
**blueprint_name** | **String** |  | 
**blueprint_version** | **String** |  | 
**package_address** | Option<**String**> | Bech32m-encoded human readable version of the address. | [optional]
**role_assignments** | Option<[**models::ComponentEntityRoleAssignments**](ComponentEntityRoleAssignments.md)> |  | [optional]
**royalty_config** | Option<[**models::ComponentRoyaltyConfig**](ComponentRoyaltyConfig.md)> |  | [optional]
**royalty_vault_balance** | Option<**String**> | String-encoded decimal representing the amount of a related fungible resource. | [optional]
**state** | Option<[**serde_json::Value**](.md)> | A representation of a component's inner state. If this entity is a `GenericComponent`, this field will be in a programmatic JSON structure (you can deserialize it as a `ProgrammaticScryptoSborValue`). Otherwise, for \"native\" components such as `Account`, `Validator`, `AccessController`, `OneResourcePool`, `TwoResourcePool`, and `MultiResourcePool`, this field will be a custom JSON model defined in the Core API schema.  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


