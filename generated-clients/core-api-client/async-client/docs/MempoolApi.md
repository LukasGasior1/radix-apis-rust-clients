# \MempoolApi

All URIs are relative to *http://localhost:3333/core*

Method | HTTP request | Description
------------- | ------------- | -------------
[**mempool_list_post**](MempoolApi.md#mempool_list_post) | **POST** /mempool/list | Get Mempool List
[**mempool_transaction_post**](MempoolApi.md#mempool_transaction_post) | **POST** /mempool/transaction | Get Mempool Transaction



## mempool_list_post

> models::MempoolListResponse mempool_list_post(mempool_list_request)
Get Mempool List

Returns the hashes of all the transactions currently in the mempool

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**mempool_list_request** | [**MempoolListRequest**](MempoolListRequest.md) |  | [required] |

### Return type

[**models::MempoolListResponse**](MempoolListResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## mempool_transaction_post

> models::MempoolTransactionResponse mempool_transaction_post(mempool_transaction_request)
Get Mempool Transaction

Returns the payload of a transaction currently in the mempool

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**mempool_transaction_request** | [**MempoolTransactionRequest**](MempoolTransactionRequest.md) |  | [required] |

### Return type

[**models::MempoolTransactionResponse**](MempoolTransactionResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

