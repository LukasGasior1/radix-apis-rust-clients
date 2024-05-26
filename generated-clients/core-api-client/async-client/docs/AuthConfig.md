# AuthConfig

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**function_access_rules** | Option<[**std::collections::HashMap<String, models::AccessRule>**](AccessRule.md)> | A map from a function name to AccessRule. Only exists if `function_auth_type` is set to `FunctionAccessRules`.  | [optional]
**function_auth_type** | [**models::FunctionAuthType**](FunctionAuthType.md) |  | 
**method_auth_type** | [**models::MethodAuthType**](MethodAuthType.md) |  | 
**method_roles** | Option<[**models::StaticRoleDefinitionAuthTemplate**](StaticRoleDefinitionAuthTemplate.md)> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


