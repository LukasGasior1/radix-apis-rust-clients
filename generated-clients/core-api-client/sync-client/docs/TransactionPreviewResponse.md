# TransactionPreviewResponse

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**at_ledger_state** | [**models::LedgerStateSummary**](LedgerStateSummary.md) | A summarized state of the ledger on top of which the preview was performed. | 
**encoded_receipt** | **String** | The hex-sbor-encoded receipt.  This field is deprecated and will be removed from the API with the release of the next  protocol update, cuttlefish. This field was provided primarily for use with the Radix  Engine Toolkit and its execution summary functionality. If you still wish to use this  functionality update your Radix Engine Toolkit and use the receipt provided in the  `radix_engine_toolkit_receipt` field of this response.  | 
**instruction_resource_changes** | [**Vec<models::InstructionResourceChanges>**](InstructionResourceChanges.md) |  | 
**logs** | [**Vec<models::TransactionPreviewResponseLogsInner>**](TransactionPreviewResponse_logs_inner.md) |  | 
**radix_engine_toolkit_receipt** | Option<[**serde_json::Value**](.md)> | An optional field which is only provided if the `request_radix_engine_toolkit_receipt` flag is set to true when requesting a transaction preview from the API.  This receipt is primarily intended for use with the toolkit and may contain information  that is already available in the receipt provided in the `receipt` field of this  response.  A typical client of this API is not expected to use this receipt. The primary clients  this receipt is intended for is the Radix wallet or any client that needs to perform  execution summaries on their transactions.  | [optional]
**receipt** | [**models::TransactionReceipt**](TransactionReceipt.md) |  | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


