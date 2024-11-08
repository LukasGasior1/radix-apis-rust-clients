# ExecutedScenarioTransaction

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**intent_hash** | **String** | The hex-encoded transaction intent hash for a user transaction, also known as the transaction id. This hash identifies the core \"intent\" of the transaction. Each transaction intent can only be committed once. This hash gets signed by any signatories on the transaction, to create the signed intent.  | 
**intent_hash_bech32m** | **String** | The Bech32m-encoded human readable `TransactionIntentHash`. | 
**logical_name** | **String** |  | 
**state_version** | **u64** |  | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


