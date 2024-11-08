# SubintentV2

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**hash** | **String** | The hex-encoded subintent hash for a subintent, also known as the partial transaction id. This hash identifies the subintent. Each subintent can only be *successfully* committed once, but unlike a transaction intent, could be committed as a failure zero or more times first. This hash gets signed by any signatories on subintent.  | 
**hash_bech32m** | **String** | The Bech32m-encoded human readable `SubintentHash`. | 
**intent_core** | [**models::IntentCoreV2**](IntentCoreV2.md) |  | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


