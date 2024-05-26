# ConsensusManagerFieldConfigValue

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**epoch_change_condition** | [**models::EpochChangeCondition**](EpochChangeCondition.md) |  | 
**max_validators** | **u64** | An integer between `0` and `10^10`, specifying the maximum number of validators in the active validator set.  | 
**min_validator_reliability** | **String** | A string-encoded fixed-precision decimal to 18 decimal places. A decimal is formed of some signed integer `m` of attos (`10^(-18)`) units, where `-2^(192 - 1) <= m < 2^(192 - 1)`.  | 
**num_fee_increase_delay_epochs** | **u64** | An integer between `0` and `10^10`, specifying the minimum number of epochs before a fee increase takes effect.  | 
**num_owner_stake_units_unlock_epochs** | **u64** | An integer between `0` and `10^10`, specifying the minimum number of epochs before an owner can take their stake units after attempting to withdraw them.  | 
**num_unstake_epochs** | **u64** | An integer between `0` and `10^10`, specifying the minimum number of epochs before an unstaker can withdraw their XRD.  | 
**total_emission_xrd_per_epoch** | **String** | A string-encoded fixed-precision decimal to 18 decimal places. A decimal is formed of some signed integer `m` of attos (`10^(-18)`) units, where `-2^(192 - 1) <= m < 2^(192 - 1)`.  | 
**validator_creation_usd_equivalent_cost** | **String** | The defining decimal cost of a validator in USD. This is turned into an XRD cost through the current protocol-based USD/XRD multiplier. A decimal is formed of some signed integer `m` of attos (`10^(-18)`) units, where `-2^(192 - 1) <= m < 2^(192 - 1)`.  | 
**validator_creation_xrd_cost** | **String** | The decimal amount of XRD required to be passed in a bucket to create a validator. A decimal is formed of some signed integer `m` of attos (`10^(-18)`) units, where `-2^(192 - 1) <= m < 2^(192 - 1)`.  | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


