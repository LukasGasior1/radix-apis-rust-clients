# \StreamApi

All URIs are relative to *http://localhost:3333/core*

Method | HTTP request | Description
------------- | ------------- | -------------
[**stream_proofs_post**](StreamApi.md#stream_proofs_post) | **POST** /stream/proofs | Stream Proofs
[**stream_transactions_post**](StreamApi.md#stream_transactions_post) | **POST** /stream/transactions | Get Committed Transactions



## stream_proofs_post

> models::StreamProofsResponse stream_proofs_post(stream_proofs_request)
Stream Proofs

Returns a stream of proofs committed to the node's ledger.  NOTE: This endpoint may return different results on different nodes: * Each node may persist different subset of signatures on a given proofs, as long as enough of the validator set has signed. * Inside an epoch, different nodes may receive and persist / keep different proofs, subject to constraints on gaps between proofs.  Proofs during an epoch can also be garbage collected by the node after the fact. Therefore proofs may disappear from this stream.  Some proofs (such as during genesis and protocol update enactment) are created on a node and don't include signatures.  This stream accepts four different options in the request: * All proofs forward (from state version) * All end-of-epoch proofs (from epoch number) * All end-of-epoch proofs triggering a protocol update * All node-injected proofs enacting genesis or a protocol update (for protocol update name, from state version)  The end-of-epoch proofs can be used to \"trustlessly\" verify the validator set for a given epoch. By tracking the fact that validators for epoch N sign the next validator set for epoch N + 1, this chain of proofs can be used to provide proof of the current validator set from a hardcoded start.  When a validator set is known for a given epoch, this can be used to verify the various transaction hash trees in the epoch, and to prove other data.  NOTE: This endpoint was built after agreeing the new Radix convention for paged APIs. Its models therefore follow the new convention, rather than attempting to align with existing loose Core API conventions. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**stream_proofs_request** | [**StreamProofsRequest**](StreamProofsRequest.md) |  | [required] |

### Return type

[**models::StreamProofsResponse**](StreamProofsResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## stream_transactions_post

> models::StreamTransactionsResponse stream_transactions_post(stream_transactions_request)
Get Committed Transactions

Returns the list of committed transactions. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**stream_transactions_request** | [**StreamTransactionsRequest**](StreamTransactionsRequest.md) |  | [required] |

### Return type

[**models::StreamTransactionsResponse**](StreamTransactionsResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

