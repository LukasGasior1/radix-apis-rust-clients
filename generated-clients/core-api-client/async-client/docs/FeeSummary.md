# FeeSummary

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**execution_cost_units_consumed** | **u64** | An integer between `0` and `2^32 - 1`, representing the amount of cost units consumed by the transaction execution. | 
**finalization_cost_units_consumed** | **u64** | An integer between `0` and `2^32 - 1`, representing the amount of cost units consumed by the transaction finalization. | 
**xrd_total_execution_cost** | **String** | The string-encoded decimal representing the total amount of XRD burned in the transaction as part of execution costs. A decimal is formed of some signed integer `m` of attos (`10^(-18)`) units, where `-2^(192 - 1) <= m < 2^(192 - 1)`.  | 
**xrd_total_finalization_cost** | **String** | The string-encoded decimal representing the total amount of XRD burned in the transaction as part of finalization costs. A decimal is formed of some signed integer `m` of attos (`10^(-18)`) units, where `-2^(192 - 1) <= m < 2^(192 - 1)`.  | 
**xrd_total_royalty_cost** | **String** | The string-encoded decimal representing the total amount of XRD paid in royalties as part of the transaction. A decimal is formed of some signed integer `m` of attos (`10^(-18)`) units, where `-2^(192 - 1) <= m < 2^(192 - 1)`.  | 
**xrd_total_storage_cost** | **String** | The string-encoded decimal representing the total amount of XRD paid in state expansion costs as part of the transaction. A decimal is formed of some signed integer `m` of attos (`10^(-18)`) units, where `-2^(192 - 1) <= m < 2^(192 - 1)`.  | 
**xrd_total_tipping_cost** | **String** | The string-encoded decimal representing the total amount of XRD tipped to validators in the transaction. A decimal is formed of some signed integer `m` of attos (`10^(-18)`) units, where `-2^(192 - 1) <= m < 2^(192 - 1)`.  | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


