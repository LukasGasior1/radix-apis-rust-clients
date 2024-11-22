# \TransactionApi

All URIs are relative to *http://localhost:3333/core*

Method | HTTP request | Description
------------- | ------------- | -------------
[**transaction_call_preview_post**](TransactionApi.md#transaction_call_preview_post) | **POST** /transaction/call-preview | Scrypto Call Preview
[**transaction_parse_post**](TransactionApi.md#transaction_parse_post) | **POST** /transaction/parse | Parse Transaction Payload
[**transaction_preview_post**](TransactionApi.md#transaction_preview_post) | **POST** /transaction/preview | Transaction Preview V1
[**transaction_preview_v2_post**](TransactionApi.md#transaction_preview_v2_post) | **POST** /transaction/preview-v2 | Transaction Preview V2
[**transaction_receipt_post**](TransactionApi.md#transaction_receipt_post) | **POST** /transaction/receipt | Get Transaction Receipt
[**transaction_status_post**](TransactionApi.md#transaction_status_post) | **POST** /transaction/status | Get Transaction Status
[**transaction_submit_post**](TransactionApi.md#transaction_submit_post) | **POST** /transaction/submit | Transaction Submit



## transaction_call_preview_post

> models::TransactionCallPreviewResponse transaction_call_preview_post(transaction_call_preview_request)
Scrypto Call Preview

Preview a scrypto function or method call against the latest network state. Returns the result of the scrypto function or method call. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**transaction_call_preview_request** | [**TransactionCallPreviewRequest**](TransactionCallPreviewRequest.md) |  | [required] |

### Return type

[**models::TransactionCallPreviewResponse**](TransactionCallPreviewResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## transaction_parse_post

> models::TransactionParseResponse transaction_parse_post(transaction_parse_request)
Parse Transaction Payload

Extracts the contents and hashes of various types of transaction payloads, or sub-payloads.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**transaction_parse_request** | [**TransactionParseRequest**](TransactionParseRequest.md) |  | [required] |

### Return type

[**models::TransactionParseResponse**](TransactionParseResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## transaction_preview_post

> models::TransactionPreviewResponse transaction_preview_post(transaction_preview_request)
Transaction Preview V1

Preview a transaction against the latest network state, and returns the preview receipt. If the node has enabled it, you may be able to also preview against recent network state.  For V2 transactions (and beyond) the `/preview-v2` endpoint should be used instead. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**transaction_preview_request** | [**TransactionPreviewRequest**](TransactionPreviewRequest.md) |  | [required] |

### Return type

[**models::TransactionPreviewResponse**](TransactionPreviewResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## transaction_preview_v2_post

> models::TransactionPreviewV2Response transaction_preview_v2_post(transaction_preview_v2_request)
Transaction Preview V2

Previews a transaction against the latest network state, and returns the preview receipt. If the node has enabled it, you may be able to also preview against recent network state.  This endpoint supports V2 transactions (and beyond). If you still need to preview V1 transactions, you should use the `/preview` endpoint instead. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**transaction_preview_v2_request** | [**TransactionPreviewV2Request**](TransactionPreviewV2Request.md) |  | [required] |

### Return type

[**models::TransactionPreviewV2Response**](TransactionPreviewV2Response.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## transaction_receipt_post

> models::TransactionReceiptResponse transaction_receipt_post(transaction_receipt_request)
Get Transaction Receipt

Gets the transaction receipt for a committed transaction. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**transaction_receipt_request** | [**TransactionReceiptRequest**](TransactionReceiptRequest.md) |  | [required] |

### Return type

[**models::TransactionReceiptResponse**](TransactionReceiptResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## transaction_status_post

> models::TransactionStatusResponse transaction_status_post(transaction_status_request)
Get Transaction Status

Shares the node's knowledge of any payloads associated with the given intent hash. Generally there will be a single payload for a given intent, but it's theoretically possible there may be multiple. This knowledge is summarised into a status for the intent. This summarised status in the response is likely sufficient for most clients. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**transaction_status_request** | [**TransactionStatusRequest**](TransactionStatusRequest.md) |  | [required] |

### Return type

[**models::TransactionStatusResponse**](TransactionStatusResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## transaction_submit_post

> models::TransactionSubmitResponse transaction_submit_post(transaction_submit_request)
Transaction Submit

Submits a notarized transaction to the network. Returns whether the transaction submission was already included in the node's mempool. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**transaction_submit_request** | [**TransactionSubmitRequest**](TransactionSubmitRequest.md) |  | [required] |

### Return type

[**models::TransactionSubmitResponse**](TransactionSubmitResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

