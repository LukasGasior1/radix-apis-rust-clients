# ProtocolVersionReadiness

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**signalled_protocol_version** | Option<**String**> | A name identifying a protocol version. May be absent to denote no readiness signalled by the `signalling_validators`.  | [optional]
**signalling_validators** | [**Vec<models::SignallingValidator>**](SignallingValidator.md) | References to some of the current validators (i.e. a subset of `current_validator_set`) which have signalled readiness for the `signalled_protocol_version`.  | 
**total_active_stake_proportion** | **String** | A sum of `active_stake_proportion` across `signalling_validators` (i.e. an easily-computable convenience field). This is a string-encoded fixed-precision decimal to 18 decimal places. A decimal is formed of some signed integer `m` of attos (`10^(-18)`) units, where `-2^(192 - 1) <= m < 2^(192 - 1)`.  | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


