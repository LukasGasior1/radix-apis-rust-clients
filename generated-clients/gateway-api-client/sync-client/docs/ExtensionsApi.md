# \ExtensionsApi

All URIs are relative to *https://mainnet.radixdlt.com*

Method | HTTP request | Description
------------- | ------------- | -------------
[**resource_holders_page**](ExtensionsApi.md#resource_holders_page) | **POST** /extensions/resource-holders/page | Get Resource Holders Page



## resource_holders_page

> models::ResourceHoldersResponse resource_holders_page(resource_holders_request)
Get Resource Holders Page

A paginated endpoint to discover which global entities hold the most of a given resource. More specifically, it returns a page of global entities which hold the given resource, ordered descending by the total fungible balance / total count of non-fungibles stored in vaults in the state tree of that entity (excluding unclaimed royalty balances). This endpoint operates only at the **current state version**, it is not possible to browse historical data. Because of that, it is not possible to offer stable pagination as data constantly changes. Balances might change between pages being read, which might result in gaps or some entries being returned twice. Under default Gateway configuration, up to 100 entries are returned per response. This can be increased up to 1000 entries per page with the `limit_per_page` parameter. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**resource_holders_request** | [**ResourceHoldersRequest**](ResourceHoldersRequest.md) |  | [required] |

### Return type

[**models::ResourceHoldersResponse**](ResourceHoldersResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

