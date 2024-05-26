# CommittedIntentMetadata

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**is_same_transaction** | **bool** | Whether the intent was committed in a transaction with the same payload. This is a convenience field, which can also be computed using `payload_hash` by a client knowing the payload of the submitted transaction.  | 
**payload_hash** | **String** | The hex-encoded notarized transaction hash for a user transaction. This hash identifies the full submittable notarized transaction - ie the signed intent, plus the notary signature.  | 
**payload_hash_bech32m** | **String** | The Bech32m-encoded human readable `NotarizedTransactionHash`. | 
**state_version** | **u64** |  | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


