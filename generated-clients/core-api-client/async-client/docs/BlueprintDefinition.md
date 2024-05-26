# BlueprintDefinition

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**function_exports** | [**std::collections::HashMap<String, models::PackageExport>**](PackageExport.md) | A map from the function name to its export | 
**hook_exports** | [**Vec<models::HookExport>**](HookExport.md) | A map from certain object lifecycle hooks to a callback \"package export\". There is at most one callback registered for each `ObjectHook`.  | 
**interface** | [**models::BlueprintInterface**](BlueprintInterface.md) |  | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


