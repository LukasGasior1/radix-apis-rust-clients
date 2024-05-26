# TransactionPreviewRequest

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**blobs_hex** | Option<**Vec<String>**> | An array of hex-encoded blob data (optional) | [optional]
**end_epoch_exclusive** | **u64** | An integer between `0` and `10^10`, marking the epoch at which the transaction is no longer valid | 
**flags** | [**models::TransactionPreviewRequestFlags**](TransactionPreviewRequest_flags.md) |  | 
**manifest** | **String** | A text-representation of a transaction manifest | 
**nonce** | **u64** | A decimal-string-encoded integer between `0` and `2^32 - 1`, used to ensure the transaction intent is unique. | 
**notary_is_signatory** | Option<**bool**> | Whether the notary should count as a signatory (optional, default false) | [optional]
**notary_public_key** | Option<[**models::PublicKey**](PublicKey.md)> |  | [optional]
**signer_public_keys** | [**Vec<models::PublicKey>**](PublicKey.md) | A list of public keys to be used as transaction signers | 
**start_epoch_inclusive** | **u64** | An integer between `0` and `10^10`, marking the epoch at which the transaction starts being valid | 
**tip_percentage** | **u32** | An integer between `0` and `65535`, giving the validator tip as a percentage amount. A value of `1` corresponds to 1% of the fee. | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


