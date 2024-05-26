# \TransactionApi

All URIs are relative to *https://mainnet.radixdlt.com*

Method | HTTP request | Description
------------- | ------------- | -------------
[**account_deposit_pre_validation**](TransactionApi.md#account_deposit_pre_validation) | **POST** /transaction/account-deposit-pre-validation | PreValidate deposit of resources to an account
[**transaction_committed_details**](TransactionApi.md#transaction_committed_details) | **POST** /transaction/committed-details | Get Committed Transaction Details
[**transaction_construction**](TransactionApi.md#transaction_construction) | **POST** /transaction/construction | Get Construction Metadata
[**transaction_preview**](TransactionApi.md#transaction_preview) | **POST** /transaction/preview | Preview Transaction
[**transaction_status**](TransactionApi.md#transaction_status) | **POST** /transaction/status | Get Transaction Status
[**transaction_submit**](TransactionApi.md#transaction_submit) | **POST** /transaction/submit | Submit Transaction



## account_deposit_pre_validation

> models::AccountDepositPreValidationResponse account_deposit_pre_validation(account_deposit_pre_validation_request)
PreValidate deposit of resources to an account

Helper endpoint that allows pre-validation if a deposit of certain resources to a given account can succeed or not. It is only meant for pre-validation usage, it does not guarantee that execution will succeed. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**account_deposit_pre_validation_request** | [**AccountDepositPreValidationRequest**](AccountDepositPreValidationRequest.md) |  | [required] |

### Return type

[**models::AccountDepositPreValidationResponse**](AccountDepositPreValidationResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## transaction_committed_details

> models::TransactionCommittedDetailsResponse transaction_committed_details(transaction_committed_details_request)
Get Committed Transaction Details

Returns the committed details and receipt of the transaction for a given transaction identifier. Transaction identifiers which don't correspond to a committed transaction will return a `TransactionNotFoundError`. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**transaction_committed_details_request** | [**TransactionCommittedDetailsRequest**](TransactionCommittedDetailsRequest.md) |  | [required] |

### Return type

[**models::TransactionCommittedDetailsResponse**](TransactionCommittedDetailsResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## transaction_construction

> models::TransactionConstructionResponse transaction_construction()
Get Construction Metadata

Returns information needed to construct a new transaction including current `epoch` number. 

### Parameters

This endpoint does not need any parameter.

### Return type

[**models::TransactionConstructionResponse**](TransactionConstructionResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## transaction_preview

> models::TransactionPreviewResponse transaction_preview(transaction_preview_request)
Preview Transaction

Previews transaction against the network. This endpoint is effectively a proxy towards the Core API `/v0/transaction/preview` endpoint. See the Core API documentation for more details. 

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


## transaction_status

> models::TransactionStatusResponse transaction_status(transaction_status_request)
Get Transaction Status

Returns overall transaction status and all of its known payloads based on supplied intent hash. 

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


## transaction_submit

> models::TransactionSubmitResponse transaction_submit(transaction_submit_request)
Submit Transaction

Submits a signed transaction payload to the network. 

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

