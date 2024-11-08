# IntentHeaderV2

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**end_epoch_exclusive** | **u64** | An integer between `0` and `10^10`, marking the epoch from which the transaction will no longer be valid, and be rejected. In the case of uncommitted transactions, a value of `10^10` indicates that the epoch was >= `10^10`.  | 
**intent_discriminator** | **String** | The string representation of an integer between `0` and `2^64 - 1`, which can be chosen to ensure that a unique intent can be created. It only needs to be unique for a particular intent content and epoch/timestamp, and can be safely selected randomly to very high probability.  This field was called `nonce` in the V1 models, but was a misleading name, as it got confused with a cryptographic nonce or an Ethereum-style nonce, and it is neither.  | 
**max_proposer_timestamp_exclusive** | Option<[**models::ScryptoInstant**](ScryptoInstant.md)> |  | [optional]
**min_proposer_timestamp_inclusive** | Option<[**models::ScryptoInstant**](ScryptoInstant.md)> |  | [optional]
**network_id** | **u32** | The logical id of the network | 
**start_epoch_inclusive** | **u64** | An integer between `0` and `10^10`, marking the epoch from which the transaction can be submitted. In the case of uncommitted transactions, a value of `10^10` indicates that the epoch was >= `10^10`.  | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


