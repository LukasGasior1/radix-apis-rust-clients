# \StatusApi

All URIs are relative to *https://mainnet.radixdlt.com*

Method | HTTP request | Description
------------- | ------------- | -------------
[**gateway_status**](StatusApi.md#gateway_status) | **POST** /status/gateway-status | Get Gateway Status
[**network_configuration**](StatusApi.md#network_configuration) | **POST** /status/network-configuration | Get Network Configuration



## gateway_status

> models::GatewayStatusResponse gateway_status()
Get Gateway Status

Returns the Gateway API version and current ledger state. 

### Parameters

This endpoint does not need any parameter.

### Return type

[**models::GatewayStatusResponse**](GatewayStatusResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## network_configuration

> models::NetworkConfigurationResponse network_configuration()
Get Network Configuration

Returns network identifier, network name and well-known network addresses. 

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

