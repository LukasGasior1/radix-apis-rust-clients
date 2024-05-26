# StreamProofsErrorDetailsRequestedEpochOutOfBounds

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**r#type** | [**models::StreamProofsErrorDetailsType**](StreamProofsErrorDetailsType.md) |  | 
**max_ledger_epoch** | **u64** | The maximum completed epoch committed to this node's ledger. *Note on the bounds:* the requested `from_epoch` cannot be greater than `max_ledger_epoch + 1`. Any greater requested value triggers this error.  | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


