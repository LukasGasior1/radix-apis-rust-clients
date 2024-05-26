# \StateApi

All URIs are relative to *http://localhost:3333/core*

Method | HTTP request | Description
------------- | ------------- | -------------
[**state_access_controller_post**](StateApi.md#state_access_controller_post) | **POST** /state/access-controller | Get Access Controller Details
[**state_account_post**](StateApi.md#state_account_post) | **POST** /state/account | Get Account Details
[**state_component_post**](StateApi.md#state_component_post) | **POST** /state/component | Get Component Details
[**state_consensus_manager_post**](StateApi.md#state_consensus_manager_post) | **POST** /state/consensus-manager | Get Consensus Manager Details
[**state_non_fungible_post**](StateApi.md#state_non_fungible_post) | **POST** /state/non-fungible | Get Non-Fungible Details
[**state_package_post**](StateApi.md#state_package_post) | **POST** /state/package | Get Package Details
[**state_resource_post**](StateApi.md#state_resource_post) | **POST** /state/resource | Get Resource Details
[**state_validator_post**](StateApi.md#state_validator_post) | **POST** /state/validator | Get Validator Details



## state_access_controller_post

> models::StateAccessControllerResponse state_access_controller_post(state_access_controller_request)
Get Access Controller Details

Reads the access controller's substate/s from the top of the current ledger. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**state_access_controller_request** | [**StateAccessControllerRequest**](StateAccessControllerRequest.md) |  | [required] |

### Return type

[**models::StateAccessControllerResponse**](StateAccessControllerResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## state_account_post

> models::StateAccountResponse state_account_post(state_account_request)
Get Account Details

Reads the account's substate/s from the top of the current ledger. Also returns all vaults under the account. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**state_account_request** | [**StateAccountRequest**](StateAccountRequest.md) |  | [required] |

### Return type

[**models::StateAccountResponse**](StateAccountResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## state_component_post

> models::StateComponentResponse state_component_post(state_component_request)
Get Component Details

Reads the component's substate/s from the top of the current ledger. Also recursively extracts vault balance totals from the component's entity subtree. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**state_component_request** | [**StateComponentRequest**](StateComponentRequest.md) |  | [required] |

### Return type

[**models::StateComponentResponse**](StateComponentResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## state_consensus_manager_post

> models::StateConsensusManagerResponse state_consensus_manager_post(state_consensus_manager_request)
Get Consensus Manager Details

Reads the consensus manager's substate/s from the top of the current ledger. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**state_consensus_manager_request** | [**StateConsensusManagerRequest**](StateConsensusManagerRequest.md) |  | [required] |

### Return type

[**models::StateConsensusManagerResponse**](StateConsensusManagerResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## state_non_fungible_post

> models::StateNonFungibleResponse state_non_fungible_post(state_non_fungible_request)
Get Non-Fungible Details

Reads the data associated with a single Non-Fungible Unit under a Non-Fungible Resource. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**state_non_fungible_request** | [**StateNonFungibleRequest**](StateNonFungibleRequest.md) |  | [required] |

### Return type

[**models::StateNonFungibleResponse**](StateNonFungibleResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## state_package_post

> models::StatePackageResponse state_package_post(state_package_request)
Get Package Details

Reads the package's substate/s from the top of the current ledger. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**state_package_request** | [**StatePackageRequest**](StatePackageRequest.md) |  | [required] |

### Return type

[**models::StatePackageResponse**](StatePackageResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## state_resource_post

> models::StateResourceResponse state_resource_post(state_resource_request)
Get Resource Details

Reads the resource manager's substate/s from the top of the current ledger. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**state_resource_request** | [**StateResourceRequest**](StateResourceRequest.md) |  | [required] |

### Return type

[**models::StateResourceResponse**](StateResourceResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## state_validator_post

> models::StateValidatorResponse state_validator_post(state_validator_request)
Get Validator Details

Reads the validator's substate/s from the top of the current ledger. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**state_validator_request** | [**StateValidatorRequest**](StateValidatorRequest.md) |  | [required] |

### Return type

[**models::StateValidatorResponse**](StateValidatorResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

