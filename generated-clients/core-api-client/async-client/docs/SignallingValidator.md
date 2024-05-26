# SignallingValidator

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**active_stake_proportion** | **String** | A proportion (between 0 and 1) of the total active stake of an entire `current_validator_set` (i.e. an easily-computable convenience field). This is a string-encoded fixed-precision decimal to 18 decimal places. A decimal is formed of some signed integer `m` of attos (`10^(-18)`) units, where `-2^(192 - 1) <= m < 2^(192 - 1)`.  | 
**index** | [**models::ActiveValidatorIndex**](ActiveValidatorIndex.md) | Validator index within the `current_validator_set`. | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


