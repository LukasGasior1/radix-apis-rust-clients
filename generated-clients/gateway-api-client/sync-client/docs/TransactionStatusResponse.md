# TransactionStatusResponse

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**ledger_state** | [**models::LedgerState**](LedgerState.md) |  | 
**committed_state_version** | Option<**i64**> | If the intent was committed, this gives the state version when this intent was committed.  | [optional]
**error_message** | Option<**String**> | The most relevant error message received, due to a rejection or commit as failure. Please note that presence of an error message doesn't imply that the intent will definitely reject or fail. This could represent a temporary error (such as out of fees), or an error with a payload which doesn't end up being committed.  | [optional]
**intent_status** | [**models::TransactionIntentStatus**](TransactionIntentStatus.md) |  | 
**intent_status_description** | **String** | An additional description to clarify the intent status.  | 
**known_payloads** | [**Vec<models::TransactionStatusResponseKnownPayloadItem>**](TransactionStatusResponseKnownPayloadItem.md) |  | 
**permanently_rejects_at_epoch** | Option<**u64**> | The epoch number at which the transaction is guaranteed to get permanently rejected by the Network due to exceeded epoch range defined when submitting transaction. | [optional]
**status** | [**models::TransactionStatus**](TransactionStatus.md) |  | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


