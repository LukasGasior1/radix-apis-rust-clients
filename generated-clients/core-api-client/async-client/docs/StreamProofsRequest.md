# StreamProofsRequest

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**continuation_token** | Option<**String**> | A continuation token is returned if and only if there are further non-empty pages of items currently available. The token can be provided in a following request to fetch the next page of results. The filter and sort should not be changed when re-using the continuation token.  | [optional]
**filter** | Option<[**models::StreamProofsFilter**](StreamProofsFilter.md)> |  | [optional]
**max_page_size** | Option<**i32**> | If specified, the maximum number of proofs that will be returned. | [optional]
**network** | **String** | The logical name of the network | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


