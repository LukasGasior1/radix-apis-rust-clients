# TransactionCallPreviewResponse

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**at_ledger_state** | [**models::LedgerStateSummary**](LedgerStateSummary.md) | A summarized state of the ledger on top of which the preview was performed. | 
**error_message** | Option<**String**> | Error message (only present if status is Failed or Rejected) | [optional]
**output** | Option<[**models::SborData**](SborData.md)> |  | [optional]
**status** | [**models::TransactionStatus**](TransactionStatus.md) |  | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


