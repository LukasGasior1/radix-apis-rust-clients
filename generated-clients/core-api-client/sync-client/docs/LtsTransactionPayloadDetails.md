# LtsTransactionPayloadDetails

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**error_message** | Option<**String**> | An explanation for the error, if failed or rejected | [optional]
**payload_hash** | **String** | The hex-encoded notarized transaction hash for a user transaction. This hash identifies the full submittable notarized transaction - ie the signed intent, plus the notary signature.  | 
**payload_hash_bech32m** | **String** | The Bech32m-encoded human readable `NotarizedTransactionHash`. | 
**state_version** | Option<**u64**> |  | [optional]
**status** | [**models::LtsTransactionPayloadStatus**](LtsTransactionPayloadStatus.md) |  | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


