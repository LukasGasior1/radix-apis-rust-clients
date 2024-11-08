# TransactionIntentV2

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**hash** | **String** | The hex-encoded transaction intent hash for a user transaction, also known as the transaction id. This hash identifies the core \"intent\" of the transaction. Each transaction intent can only be committed once. This hash gets signed by any signatories on the transaction, to create the signed intent.  | 
**hash_bech32m** | **String** | The Bech32m-encoded human readable `TransactionIntentHash`. | 
**non_root_subintents** | [**Vec<models::SubintentV2>**](SubintentV2.md) |  | 
**root_intent_core** | [**models::IntentCoreV2**](IntentCoreV2.md) |  | 
**transaction_header** | [**models::TransactionHeaderV2**](TransactionHeaderV2.md) |  | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


