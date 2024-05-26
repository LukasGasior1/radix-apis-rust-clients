# StreamTransactionsResponse

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**count** | **u32** | An integer between `0` and `10000`, giving the total count of transactions in the returned response | 
**from_state_version** | **u64** |  | 
**max_ledger_state_version** | **u64** |  | 
**previous_state_identifiers** | Option<[**models::CommittedStateIdentifier**](CommittedStateIdentifier.md)> | Identifiers for the state on top of which the returned transactions were executed (ie `from_state_version - 1`). This should be used for sanity-checking that you're reading from the ledger history you're expecting. If this is field is missing, the previous state does not exists (`from_state_version` is 0).  | [optional]
**proofs** | Option<[**Vec<models::LedgerProof>**](LedgerProof.md)> | A ledger proof list starting from `from_state_version` (inclusive) stored by this node. | [optional]
**transactions** | [**Vec<models::CommittedTransaction>**](CommittedTransaction.md) | A committed transactions list starting from the `from_state_version` (inclusive). | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


