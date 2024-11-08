# NotarizedTransactionV2

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**hash** | **String** | The hex-encoded notarized transaction hash for a user transaction. This hash identifies the full submittable notarized transaction - ie the signed intent, plus the notary signature.  | 
**hash_bech32m** | **String** | The Bech32m-encoded human readable `NotarizedTransactionHash`. | 
**notary_signature** | [**models::Signature**](Signature.md) |  | 
**payload_hex** | Option<**String**> | The hex-encoded full notarized transaction payload. Returning this can be disabled in TransactionFormatOptions on your request (default true). | [optional]
**signed_transaction_intent** | [**models::SignedTransactionIntentV2**](SignedTransactionIntentV2.md) |  | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


