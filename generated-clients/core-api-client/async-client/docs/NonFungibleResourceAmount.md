# NonFungibleResourceAmount

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**resource_address** | **String** | The Bech32m-encoded human readable version of the resource address | 
**resource_type** | [**models::ResourceType**](ResourceType.md) |  | 
**amount** | **String** | The string-encoded decimal representing the amount of this resource (some decimal for fungible resources, a whole integer for non-fungible resources). A decimal is formed of some signed integer `m` of attos (`10^(-18)`) units, where `-2^(192 - 1) <= m < 2^(192 - 1)`.  | 
**non_fungible_ids** | [**Vec<models::NonFungibleLocalId>**](NonFungibleLocalId.md) |  | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


