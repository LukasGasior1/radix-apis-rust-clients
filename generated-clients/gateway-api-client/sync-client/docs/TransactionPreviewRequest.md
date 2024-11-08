# TransactionPreviewRequest

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**blobs_hex** | Option<**Vec<String>**> | An array of hex-encoded blob data, if referenced by the manifest. | [optional]
**end_epoch_exclusive** | **u64** | An integer between `0` and `10^10`, marking the epoch at which the transaction is no longer valid. If omitted, a maximum epoch (relative to the `start_epoch_inclusive`) will be used.  | 
**flags** | [**models::TransactionPreviewRequestFlags**](TransactionPreviewRequest_flags.md) |  | 
**manifest** | **String** | A text-representation of a transaction manifest | 
**message** | Option<[**serde_json::Value**](.md)> | An optional transaction message. Only affects the costing. This type is defined in the Core API as `TransactionMessage`. See the Core API documentation for more details.  | [optional]
**nonce** | **u64** | An integer between `0` and `2^32 - 1`, chosen to allow a unique intent to be created (to enable submitting an otherwise identical/duplicate intent).  | 
**notary_is_signatory** | Option<**bool**> | Whether the notary should count as a signatory (defaults to `false`). | [optional]
**notary_public_key** | Option<[**models::PublicKey**](PublicKey.md)> |  | [optional]
**opt_ins** | Option<[**models::TransactionPreviewOptIns**](TransactionPreviewOptIns.md)> |  | [optional]
**signer_public_keys** | [**Vec<models::PublicKey>**](PublicKey.md) | A list of public keys to be used as transaction signers | 
**start_epoch_inclusive** | **u64** | An integer between `0` and `10^10`, marking the epoch at which the transaction starts being valid. If omitted, the current epoch will be used (taking into account the `at_ledger_state`, if specified).  | 
**tip_percentage** | **u32** | An integer between `0` and `65535`, giving the validator tip as a percentage amount. A value of `1` corresponds to a 1% fee.  | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


