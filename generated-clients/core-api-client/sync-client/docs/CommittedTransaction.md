# CommittedTransaction

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**balance_changes** | Option<[**models::CommittedTransactionBalanceChanges**](CommittedTransactionBalanceChanges.md)> | All balance changes of a transaction (including those fee-related), aggregated by resource and global entity within which the change occurred. Only returned if available and enabled in `TransactionFormatOptions` on your request.  | [optional]
**ledger_transaction** | [**models::LedgerTransaction**](LedgerTransaction.md) |  | 
**proposer_timestamp_ms** | **u64** | An integer between `0` and `10^14`, marking the proposer timestamp in ms. | 
**receipt** | [**models::TransactionReceipt**](TransactionReceipt.md) |  | 
**resultant_state_identifiers** | [**models::CommittedStateIdentifier**](CommittedStateIdentifier.md) |  | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


