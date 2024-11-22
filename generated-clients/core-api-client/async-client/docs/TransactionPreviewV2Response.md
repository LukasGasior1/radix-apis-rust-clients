# TransactionPreviewV2Response

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**at_ledger_state** | [**models::LedgerStateSummary**](LedgerStateSummary.md) | A summarized state of the ledger on top of which the preview was performed. | 
**logs** | Option<[**Vec<models::TransactionPreviewResponseLogsInner>**](TransactionPreviewResponse_logs_inner.md)> | An optional field which is only provided if the `logs` flag is set to true in the `options` property of the request.  If present, it gives the emitted logs from the transaction execution.  | [optional]
**radix_engine_toolkit_receipt** | Option<[**serde_json::Value**](.md)> | An optional field which is only provided if the `radix_engine_toolkit_receipt` flag is set to true in the `options` property of the request.  This receipt is primarily intended for use with the toolkit and may contain information  that is already available in the receipt provided in the `receipt` field of this  response.  A typical client of this API is not expected to use this receipt. The primary clients  this receipt is intended for is the Radix wallet or any client that needs to perform  execution summaries on their transactions.  | [optional]
**receipt** | Option<[**models::TransactionReceipt**](TransactionReceipt.md)> | This is provided unless the `core_api_receipt` flag is set to false in the `options` property of the request.  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


