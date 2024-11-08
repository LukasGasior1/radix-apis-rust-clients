# SignedTransactionIntentV2

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**hash** | **String** | The hex-encoded signed intent hash for a user transaction. This hash identifies the transaction intent, plus additional signatures. This hash is signed by the notary, to create the submittable `NotarizedTransaction`.  | 
**hash_bech32m** | **String** | The Bech32m-encoded human readable `SignedTransactionIntentHash`. | 
**non_root_subintent_signatures** | [**Vec<models::IntentSignatures>**](IntentSignatures.md) | This gives the signatures for each subintent in `non_root_subintents` in `TransactionIntentV2`. For committed transactions, these arrays are of equal length and correspond one-to-one in order.  | 
**transaction_intent** | [**models::TransactionIntentV2**](TransactionIntentV2.md) |  | 
**transaction_intent_signatures** | [**models::IntentSignatures**](IntentSignatures.md) |  | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


