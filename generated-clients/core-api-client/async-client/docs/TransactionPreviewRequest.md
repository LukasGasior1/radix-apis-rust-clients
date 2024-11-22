# TransactionPreviewRequest

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**at_ledger_state** | Option<[**models::LedgerStateSelector**](LedgerStateSelector.md)> |  | [optional]
**blobs_hex** | Option<**Vec<String>**> | An array of hex-encoded blob data, if referenced by the manifest. | [optional]
**end_epoch_exclusive** | Option<**u64**> | An integer between `0` and `10^10`, marking the epoch at which the transaction is no longer valid. If not provided, a maximum epoch (relative to the `start_epoch_inclusive`) will be used.  | [optional]
**flags** | Option<[**models::PreviewFlags**](PreviewFlags.md)> |  | [optional]
**manifest** | **String** | A text representation of a transaction manifest. | 
**message** | Option<[**models::TransactionMessage**](TransactionMessage.md)> | An optional transaction message. Only affects the costing. | [optional]
**network** | **String** | The logical name of the network | 
**nonce** | Option<**u64**> | An integer between `0` and `2^32 - 1`, chosen to allow a unique intent to be created (to enable submitting an otherwise identical/duplicate intent). If not provided, this defaults to 0.  | [optional]
**notary_is_signatory** | Option<**bool**> | Whether the notary should be used as a signer (optional). If not provided, this defaults to false.  | [optional]
**notary_public_key** | Option<[**models::PublicKey**](PublicKey.md)> | The notary public key to use. If not provided, this defaults to a fixed public key.  | [optional]
**options** | Option<[**models::TransactionPreviewResponseOptions**](TransactionPreviewResponseOptions.md)> |  | [optional]
**signer_public_keys** | Option<[**Vec<models::PublicKey>**](PublicKey.md)> | A list of public keys to be used as transaction signers. If not provided, this defaults to an empty array.  | [optional]
**start_epoch_inclusive** | Option<**u64**> | An integer between `0` and `10^10`, marking the epoch at which the transaction starts being valid. If not provided, the current epoch will be used (taking into account the `at_ledger_state`, if specified).  | [optional]
**tip_percentage** | Option<**u32**> | An integer between `0` and `65535`, giving the validator tip as a percentage amount. A value of `1` corresponds to a 1% fee. If not provided, this defaults to 0.  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


