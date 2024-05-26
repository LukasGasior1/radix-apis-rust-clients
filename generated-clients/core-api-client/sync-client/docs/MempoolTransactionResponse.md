# MempoolTransactionResponse

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**count** | **i32** | An integer giving the total count of payload hashes checked in the returned response. | 
**payloads** | [**Vec<models::MempoolTransactionResponsePayloadsInner>**](MempoolTransactionResponse_payloads_inner.md) | An array containing pairs of payload hash (query) and payload hex or error (response). Note that this response is bounded - this means it is not guaranteed all queries will be processed. Please query missing payload hashes again.  | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


