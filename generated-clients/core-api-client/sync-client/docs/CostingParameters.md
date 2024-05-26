# CostingParameters

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**execution_cost_unit_limit** | **u64** | An integer between `0` and `2^32 - 1`, representing the maximum amount of cost units available for the transaction execution. | 
**execution_cost_unit_loan** | **u64** | An integer between `0` and `2^32 - 1`, representing the maximum number of cost units which can be used before fee is locked from a vault. | 
**execution_cost_unit_price** | **String** | The string-encoded decimal representing the XRD price of a single cost unit of transaction execution. A decimal is formed of some signed integer `m` of attos (`10^(-18)`) units, where `-2^(192 - 1) <= m < 2^(192 - 1)`.  | 
**finalization_cost_unit_limit** | **u64** | An integer between `0` and `2^32 - 1`, representing the maximum amount of cost units available for the transaction finalization. | 
**finalization_cost_unit_price** | **String** | The string-encoded decimal representing the XRD price of a single cost unit of transaction finalization. A decimal is formed of some signed integer `m` of attos (`10^(-18)`) units, where `-2^(192 - 1) <= m < 2^(192 - 1)`.  | 
**tip_percentage** | **u32** | An integer between `0` and `65535`, giving the validator tip as a percentage amount. A value of `1` corresponds to 1% of the fee. | 
**xrd_archive_storage_price** | **String** | The string-encoded decimal representing the price of 1 byte of archive storage, expressed in XRD. A decimal is formed of some signed integer `m` of attos (`10^(-18)`) units, where `-2^(192 - 1) <= m < 2^(192 - 1)`.  | 
**xrd_storage_price** | **String** | The string-encoded decimal representing the price of 1 byte of state storage, expressed in XRD. A decimal is formed of some signed integer `m` of attos (`10^(-18)`) units, where `-2^(192 - 1) <= m < 2^(192 - 1)`.  | 
**xrd_usd_price** | **String** | The string-encoded decimal representing what amount of XRD is consumed by a Royalty of 1 USD. This is fixed for a given protocol version, so is not an accurate representation of the XRD price. A decimal is formed of some signed integer `m` of attos (`10^(-18)`) units, where `-2^(192 - 1) <= m < 2^(192 - 1)`.  | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


