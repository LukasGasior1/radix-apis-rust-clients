# LtsStreamTransactionOutcomesResponse

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**committed_transaction_outcomes** | [**Vec<models::LtsCommittedTransactionOutcome>**](LtsCommittedTransactionOutcome.md) | A committed transaction outcomes list starting from the `from_state_version` (inclusive). | 
**count** | **u32** | An integer between `0` and `10000`, giving the total count of transactions in the returned response | 
**from_state_version** | **u64** |  | 
**max_ledger_state_version** | **u64** |  | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


