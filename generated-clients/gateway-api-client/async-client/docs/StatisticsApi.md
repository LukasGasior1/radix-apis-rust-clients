# \StatisticsApi

All URIs are relative to *https://mainnet.radixdlt.com*

Method | HTTP request | Description
------------- | ------------- | -------------
[**validators_uptime**](StatisticsApi.md#validators_uptime) | **POST** /statistics/validators/uptime | Get Validators Uptime



## validators_uptime

> models::ValidatorsUptimeResponse validators_uptime(validators_uptime_request)
Get Validators Uptime

Returns validators uptime data for time range limited by `from_state_version` and `at_state_version`. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**validators_uptime_request** | [**ValidatorsUptimeRequest**](ValidatorsUptimeRequest.md) |  | [required] |

### Return type

[**models::ValidatorsUptimeResponse**](ValidatorsUptimeResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

