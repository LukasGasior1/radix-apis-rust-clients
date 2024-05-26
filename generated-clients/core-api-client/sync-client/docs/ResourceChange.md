# ResourceChange

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**amount** | **String** | The string-encoded decimal representing the XRD amount put or taken from the vault. A decimal is formed of some signed integer `m` of attos (`10^(-18)`) units, where `-2^(192 - 1) <= m < 2^(192 - 1)`.  | 
**component_entity** | [**models::EntityReference**](EntityReference.md) |  | 
**resource_address** | **String** | The Bech32m-encoded human readable version of the resource address | 
**vault_entity** | [**models::EntityReference**](EntityReference.md) |  | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


