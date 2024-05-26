# FeeDestination

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**to_burn** | **String** | The string-encoded decimal representing the amount of fee burnt, in XRD. A decimal is formed of some signed integer `m` of attos (`10^(-18)`) units, where `-2^(192 - 1) <= m < 2^(192 - 1)`.  | 
**to_proposer** | **String** | The string-encoded decimal representing the amount of fee in XRD paid to the proposer. A decimal is formed of some signed integer `m` of attos (`10^(-18)`) units, where `-2^(192 - 1) <= m < 2^(192 - 1)`.  | 
**to_royalty_recipients** | [**Vec<models::PaymentToRoyaltyRecipient>**](PaymentToRoyaltyRecipient.md) | A breakdown of where the royalties were paid to.  | 
**to_validator_set** | **String** | The string-encoded decimal representing the amount of fee in XRD paid to the validator set. A decimal is formed of some signed integer `m` of attos (`10^(-18)`) units, where `-2^(192 - 1) <= m < 2^(192 - 1)`.  | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


