# MempoolTransactionResponsePayloadsInner

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**error** | Option<**String**> | Error message why `hex` field is missing: the transaction was not found in the mempool or the provided hash is invalid.  | [optional]
**hash** | **String** | The hex-encoded notarized transaction hash for a user transaction. This hash identifies the full submittable notarized transaction - ie the signed intent, plus the notary signature.  | 
**hash_bech32m** | **String** | The Bech32m-encoded human readable `NotarizedTransactionHash`. | 
**hex** | Option<**String**> | The hex-encoded full notarized transaction payload - returned only if found in mempool. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


