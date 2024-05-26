# BlueprintInterface

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**events** | [**std::collections::HashMap<String, models::BlueprintPayloadDef>**](BlueprintPayloadDef.md) | A map from the event name to the event payload type reference. | 
**features** | **Vec<String>** |  | 
**functions** | [**std::collections::HashMap<String, models::FunctionSchema>**](FunctionSchema.md) | A map from the function name to the FunctionSchema | 
**generic_type_parameters** | [**Vec<models::GenericTypeParameter>**](GenericTypeParameter.md) | Generic (SBOR) type parameters which need to be filled by a concrete instance of this blueprint.  | 
**is_transient** | **bool** | If true, an instantiation of this blueprint cannot be persisted. EG buckets and proofs are transient. | 
**outer_blueprint** | Option<**String**> |  | [optional]
**state** | [**models::IndexedStateSchema**](IndexedStateSchema.md) |  | 
**types** | [**std::collections::HashMap<String, models::ScopedTypeId>**](ScopedTypeId.md) | A map from the registered type name to the concrete type, resolved against a schema from the package's schema partition.  | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


