# TransactionHeader

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**end_epoch_exclusive** | **u64** | An integer between `0` and `10^10`, marking the epoch from which the transaction will no longer be valid, and be rejected. In the case of uncommitted transactions, a value of `10^10` indicates that the epoch was >= `10^10`.  | 
**network_id** | **u32** | The logical id of the network | 
**nonce** | **u64** | An integer between `0` and `2^32 - 1`, chosen to allow a unique intent to be created (to enable submitting an otherwise identical/duplicate intent).  | 
**notary_is_signatory** | **bool** | Specifies whether the notary public key should be included in the transaction signers list | 
**notary_public_key** | [**models::PublicKey**](PublicKey.md) |  | 
**start_epoch_inclusive** | **u64** | An integer between `0` and `10^10`, marking the epoch from which the transaction can be submitted. In the case of uncommitted transactions, a value of `10^10` indicates that the epoch was >= `10^10`.  | 
**tip_percentage** | **u32** | An integer between `0` and `65535`, giving the validator tip as a percentage amount. A value of `1` corresponds to 1% of the fee. | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


