# SignedTransactionIntent

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**hash** | **String** | The hex-encoded signed intent hash for a user transaction. This hash identifies the transaction intent, plus additional signatures. This hash is signed by the notary, to create the submittable NotarizedTransaction.  | 
**hash_bech32m** | **String** | The Bech32m-encoded human readable `SignedIntentHash`. | 
**intent** | [**models::TransactionIntent**](TransactionIntent.md) |  | 
**intent_signatures** | [**Vec<models::SignatureWithPublicKey>**](SignatureWithPublicKey.md) |  | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


