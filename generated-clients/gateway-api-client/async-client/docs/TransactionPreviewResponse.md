# TransactionPreviewResponse

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**encoded_receipt** | **String** | Hex-encoded binary blob. | 
**logs** | [**Vec<models::TransactionPreviewResponseLogsInner>**](TransactionPreviewResponse_logs_inner.md) |  | 
**radix_engine_toolkit_receipt** | Option<[**serde_json::Value**](.md)> | An optional field which is only provided if the `request_radix_engine_toolkit_receipt` flag is set to true when requesting a transaction preview from the API. This receipt is primarily intended for use with the toolkit and may contain information that is already available in the receipt provided in the `receipt` field of this response. A typical client of this API is not expected to use this receipt. The primary clients this receipt is intended for is the Radix wallet or any client that needs to perform execution summaries on their transactions.  | [optional]
**receipt** | [**serde_json::Value**](.md) | This type is defined in the Core API as `TransactionReceipt`. See the Core API documentation for more details.  | 
**resource_changes** | [**Vec<serde_json::Value>**](serde_json::Value.md) |  | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


