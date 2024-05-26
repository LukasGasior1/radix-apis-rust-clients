# TransactionPreviewResponse

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**at_ledger_state** | [**models::LedgerStateSummary**](LedgerStateSummary.md) | A summarized state of the ledger on top of which the preview was performed. | 
**encoded_receipt** | **String** | The hex-sbor-encoded receipt | 
**instruction_resource_changes** | [**Vec<models::InstructionResourceChanges>**](InstructionResourceChanges.md) |  | 
**logs** | [**Vec<models::TransactionPreviewResponseLogsInner>**](TransactionPreviewResponse_logs_inner.md) |  | 
**receipt** | [**models::TransactionReceipt**](TransactionReceipt.md) |  | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


