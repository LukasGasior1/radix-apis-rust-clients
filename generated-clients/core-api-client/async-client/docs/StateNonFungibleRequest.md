# StateNonFungibleRequest

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**network** | **String** | The logical name of the network | 
**non_fungible_id** | **String** | The simple string representation of the non-fungible id. * For string ids, this is `<the-string-id>` * For integer ids, this is `#the-integer-id#` * For bytes ids, this is `[the-lower-case-hex-representation]` * For RUID ids, this is `{...-...-...-...}` where `...` are each 16 hex characters. A given non-fungible resource has a fixed `NonFungibleIdType`, so this representation uniquely identifies this non-fungible under the given resource address.  | 
**resource_address** | **String** | The Bech32m-encoded human readable version of the resource's global address | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


