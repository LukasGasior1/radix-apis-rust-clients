# TransactionSubmitRejectedErrorDetails

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**error_message** | **String** | An explanation of the error | 
**invalid_from_epoch** | Option<**u64**> | An integer between `0` and `10^10`, marking the epoch from which the transaction will no longer be valid, and be permanently rejected. Only present if the rejection isn't permanent.  | [optional]
**is_fresh** | **bool** | Whether (true) this rejected status has just been calculated fresh, or (false) the status is from the pending transaction result cache.  | 
**is_intent_rejection_permanent** | **bool** | Whether the rejection of this intent is known to be permanent - this is a stronger statement than the payload rejection being permanent, as it implies any payloads containing the intent will also be permanently rejected.  | 
**is_payload_rejection_permanent** | **bool** | Whether the rejection of this payload is known to be permanent.  | 
**retry_from_epoch** | Option<**u64**> | An integer between `0` and `10^10`, marking the epoch after which the node will consider recalculating the validity of the transaction. Only present if the rejection is temporary due to a header specifying a \"from epoch\" in the future.  | [optional]
**retry_from_timestamp** | Option<[**models::InstantMs**](InstantMs.md)> | The time after which the node will consider recalculating the validity of the transaction. Only present if the rejection is temporary, and not due to the header specifying a \"from epoch\" in the future.  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


