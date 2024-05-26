# StreamProofsResponse

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**continuation_token** | Option<**String**> | A continuation token is returned if and only if there are further non-empty pages of items currently available. The token can be provided in a following request to fetch the next page of results. The filter and sort should not be changed when re-using the continuation token.  | [optional]
**page** | [**Vec<models::LedgerProof>**](LedgerProof.md) | A page of ledger proofs stored by this node. | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


