# ResourceSpecificDepositBehaviour

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**allows_try_deposit** | **bool** | The fully resolved `try_deposit_*` ability of this resource (which takes all the inputs into account, including the authorized depositor badge, the default deposit rule and the above resource-specific circumstances).  | 
**is_xrd** | **bool** | Whether the resource represents the native XRD fungible. XRD is a special case which does not require `vault_exists = true` to satisfy the `AllowExisting` rule.  | 
**resource_preference** | Option<[**models::ResourcePreference**](ResourcePreference.md)> | Whether the resource is on the allow or the deny list (no preference when missing). | [optional]
**vault_exists** | **bool** | Whether the account contains a vault for the resource (even if 0 balance). This plays a role when `DefaultDepositRule` is `AllowExisting`.  | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


