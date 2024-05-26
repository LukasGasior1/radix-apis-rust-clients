# AccountDepositPreValidationResponse

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**ledger_state** | [**models::LedgerState**](LedgerState.md) |  | 
**allows_try_deposit_batch** | **bool** |  | 
**deciding_factors** | [**models::AccountDepositPreValidationDecidingFactors**](AccountDepositPreValidationDecidingFactors.md) |  | 
**resource_specific_behaviour** | Option<[**Vec<models::AccountDepositPreValidationResourceSpecificBehaviourItem>**](AccountDepositPreValidationResourceSpecificBehaviourItem.md)> | The fully resolved try_deposit_* ability of each resource (which takes all the inputs into account, including the authorized depositor badge, the default deposit rule and the resource-specific details). | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


