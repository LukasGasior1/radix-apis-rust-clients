# \StatusApi

All URIs are relative to *http://localhost:3333/core*

Method | HTTP request | Description
------------- | ------------- | -------------
[**status_network_configuration_post**](StatusApi.md#status_network_configuration_post) | **POST** /status/network-configuration | Get Network Configuration
[**status_network_status_post**](StatusApi.md#status_network_status_post) | **POST** /status/network-status | Get Network Status
[**status_scenarios_post**](StatusApi.md#status_scenarios_post) | **POST** /status/scenarios | Get Scenarios' results.



## status_network_configuration_post

> models::NetworkConfigurationResponse status_network_configuration_post()
Get Network Configuration

Returns the network configuration of the network the node is connected to.

### Parameters

This endpoint does not need any parameter.

### Return type

[**models::NetworkConfigurationResponse**](NetworkConfigurationResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## status_network_status_post

> models::NetworkStatusResponse status_network_status_post(network_status_request)
Get Network Status

Returns the current state and status of the node's copy of the ledger.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**network_status_request** | [**NetworkStatusRequest**](NetworkStatusRequest.md) |  | [required] |

### Return type

[**models::NetworkStatusResponse**](NetworkStatusResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## status_scenarios_post

> models::ScenariosResponse status_scenarios_post(scenarios_request)
Get Scenarios' results.

Get results of test \"Scenarios\" executed on this Network. Note: these Scenarios are meant to only be executed on test Networks; on a production Node, the response is expected to be empty. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**scenarios_request** | [**ScenariosRequest**](ScenariosRequest.md) |  | [required] |

### Return type

[**models::ScenariosResponse**](ScenariosResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

