# ValidatorCollectionItem

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**active_in_epoch** | Option<[**models::ValidatorCollectionItemActiveInEpoch**](ValidatorCollectionItemActiveInEpoch.md)> |  | [optional]
**address** | **String** | Bech32m-encoded human readable version of the address. | 
**effective_fee_factor** | [**models::ValidatorCollectionItemEffectiveFeeFactor**](ValidatorCollectionItem_effective_fee_factor.md) |  | 
**locked_owner_stake_unit_vault** | [**models::ValidatorVaultItem**](ValidatorVaultItem.md) |  | 
**metadata** | [**models::EntityMetadataCollection**](EntityMetadataCollection.md) |  | 
**pending_owner_stake_unit_unlock_vault** | [**models::ValidatorVaultItem**](ValidatorVaultItem.md) |  | 
**pending_xrd_withdraw_vault** | [**models::ValidatorVaultItem**](ValidatorVaultItem.md) |  | 
**stake_vault** | [**models::ValidatorVaultItem**](ValidatorVaultItem.md) |  | 
**state** | Option<[**serde_json::Value**](.md)> | Validator inner state representation. This type is defined in the Core API as `ValidatorFieldStateValue`. See the Core API documentation for more details.  | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


