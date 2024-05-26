# ValidatorFeeChangeRequest

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**epoch_effective** | **u64** | An integer between `0` and `10^10`, marking the epoch at which the fee change becomes effective.  | 
**new_fee_factor** | **String** | A string-encoded fixed-precision decimal to 18 decimal places. A decimal is formed of some signed integer `m` of attos (`10^(-18)`) units, where `-2^(192 - 1) <= m < 2^(192 - 1)`.  | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


