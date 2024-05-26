# ValidatorFieldStateValue

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**accepts_delegated_stake** | **bool** |  | 
**already_unlocked_owner_stake_unit_amount** | **String** | A string-encoded fixed-precision decimal to 18 decimal places. A decimal is formed of some signed integer `m` of attos (`10^(-18)`) units, where `-2^(192 - 1) <= m < 2^(192 - 1)`.  | 
**claim_token_resource_address** | **String** | The Bech32m-encoded human readable version of the resource address | 
**is_registered** | **bool** |  | 
**locked_owner_stake_unit_vault** | [**models::EntityReference**](EntityReference.md) |  | 
**pending_owner_stake_unit_unlock_vault** | [**models::EntityReference**](EntityReference.md) |  | 
**pending_owner_stake_unit_withdrawals** | [**Vec<models::PendingOwnerStakeWithdrawal>**](PendingOwnerStakeWithdrawal.md) |  | 
**pending_xrd_withdraw_vault** | [**models::EntityReference**](EntityReference.md) |  | 
**public_key** | [**models::EcdsaSecp256k1PublicKey**](EcdsaSecp256k1PublicKey.md) |  | 
**sorted_key** | Option<[**models::SubstateKey**](SubstateKey.md)> |  | [optional]
**stake_unit_resource_address** | **String** | The Bech32m-encoded human readable version of the resource address | 
**stake_xrd_vault** | [**models::EntityReference**](EntityReference.md) |  | 
**validator_fee_change_request** | Option<[**models::ValidatorFeeChangeRequest**](ValidatorFeeChangeRequest.md)> |  | [optional]
**validator_fee_factor** | **String** | A string-encoded fixed-precision decimal to 18 decimal places. A decimal is formed of some signed integer `m` of attos (`10^(-18)`) units, where `-2^(192 - 1) <= m < 2^(192 - 1)`.  | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


