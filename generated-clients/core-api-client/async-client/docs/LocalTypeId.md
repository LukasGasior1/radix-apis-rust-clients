# LocalTypeId

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**as_sbor** | [**models::SborData**](SborData.md) | The SBOR encoding of the LocalTypeId. This is useful for passing to the Radix Engine toolkit along with a schema.  | 
**id** | **u64** | A reference to a type, interpreted according to `kind`: - If `WellKnown`, then it is a pointer to a well known scrypto type with that ID, - If `SchemaLocal`, then it is an index into the given schema.  | 
**kind** | **String** | The location against which to resolve this type reference. | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


