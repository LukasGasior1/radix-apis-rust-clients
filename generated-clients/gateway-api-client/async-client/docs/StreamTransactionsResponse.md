# StreamTransactionsResponse

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**ledger_state** | [**models::LedgerState**](LedgerState.md) |  | 
**items** | [**Vec<models::CommittedTransactionInfo>**](CommittedTransactionInfo.md) | The page of user transactions. | 
**next_cursor** | Option<**String**> | If specified, contains a cursor to query next page of the `items` collection. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


