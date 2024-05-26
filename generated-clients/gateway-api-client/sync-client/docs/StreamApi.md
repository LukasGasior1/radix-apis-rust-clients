# \StreamApi

All URIs are relative to *https://mainnet.radixdlt.com*

Method | HTTP request | Description
------------- | ------------- | -------------
[**stream_transactions**](StreamApi.md#stream_transactions) | **POST** /stream/transactions | Get Transactions Stream



## stream_transactions

> models::StreamTransactionsResponse stream_transactions(stream_transactions_request)
Get Transactions Stream

Returns transactions which have been committed to the ledger. [Check detailed documentation for brief explanation](#section/Using-the-streamtransactions-endpoint) 

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

