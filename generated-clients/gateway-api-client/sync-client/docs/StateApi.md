# \StateApi

All URIs are relative to *https://mainnet.radixdlt.com*

Method | HTTP request | Description
------------- | ------------- | -------------
[**account_authorized_depositors_page**](StateApi.md#account_authorized_depositors_page) | **POST** /state/account/page/authorized-depositors | Get Account authorized depositors
[**account_locker_vaults_page**](StateApi.md#account_locker_vaults_page) | **POST** /state/account-locker/page/vaults | Get Account Locker Vaults Page
[**account_lockers_touched_at**](StateApi.md#account_lockers_touched_at) | **POST** /state/account-lockers/touched-at | Get Most Recent Touch of Account Lockers
[**account_resource_preferences_page**](StateApi.md#account_resource_preferences_page) | **POST** /state/account/page/resource-preferences | Get Account resource preferences
[**entity_fungible_resource_vault_page**](StateApi.md#entity_fungible_resource_vault_page) | **POST** /state/entity/page/fungible-vaults/ | Get page of Global Entity Fungible Resource Vaults
[**entity_fungibles_page**](StateApi.md#entity_fungibles_page) | **POST** /state/entity/page/fungibles/ | Get page of Global Entity Fungible Resource Balances
[**entity_metadata_page**](StateApi.md#entity_metadata_page) | **POST** /state/entity/page/metadata | Get Entity Metadata Page
[**entity_non_fungible_ids_page**](StateApi.md#entity_non_fungible_ids_page) | **POST** /state/entity/page/non-fungible-vault/ids | Get page of Non-Fungibles in Vault
[**entity_non_fungible_resource_vault_page**](StateApi.md#entity_non_fungible_resource_vault_page) | **POST** /state/entity/page/non-fungible-vaults/ | Get page of Global Entity Non-Fungible Resource Vaults
[**entity_non_fungibles_page**](StateApi.md#entity_non_fungibles_page) | **POST** /state/entity/page/non-fungibles/ | Get page of Global Entity Non-Fungible Resource Balances
[**entity_schema_page**](StateApi.md#entity_schema_page) | **POST** /state/entity/page/schemas | Get Entity Schema Page
[**key_value_store_data**](StateApi.md#key_value_store_data) | **POST** /state/key-value-store/data | Get KeyValueStore Data
[**key_value_store_keys**](StateApi.md#key_value_store_keys) | **POST** /state/key-value-store/keys | Get KeyValueStore Keys
[**non_fungible_data**](StateApi.md#non_fungible_data) | **POST** /state/non-fungible/data | Get Non-Fungible Data
[**non_fungible_ids**](StateApi.md#non_fungible_ids) | **POST** /state/non-fungible/ids | Get page of Non-Fungible Ids in Resource Collection
[**non_fungible_location**](StateApi.md#non_fungible_location) | **POST** /state/non-fungible/location | Get Non-Fungible Location
[**package_blueprint_page**](StateApi.md#package_blueprint_page) | **POST** /state/package/page/blueprints | Get Package Blueprints Page
[**package_code_page**](StateApi.md#package_code_page) | **POST** /state/package/page/codes | Get Package Codes Page
[**state_entity_details**](StateApi.md#state_entity_details) | **POST** /state/entity/details | Get Entity Details
[**state_validators_list**](StateApi.md#state_validators_list) | **POST** /state/validators/list | Get Validators List



## account_authorized_depositors_page

> models::StateAccountAuthorizedDepositorsPageResponse account_authorized_depositors_page(state_account_authorized_depositors_page_request)
Get Account authorized depositors

Returns paginable collection of authorized depositors for given account. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**state_account_authorized_depositors_page_request** | [**StateAccountAuthorizedDepositorsPageRequest**](StateAccountAuthorizedDepositorsPageRequest.md) |  | [required] |

### Return type

[**models::StateAccountAuthorizedDepositorsPageResponse**](StateAccountAuthorizedDepositorsPageResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## account_locker_vaults_page

> models::StateAccountLockerPageVaultsResponse account_locker_vaults_page(state_account_locker_page_vaults_request)
Get Account Locker Vaults Page

Returns all the resource vaults associated with a given account locker. The returned response is in a paginated format, ordered by the most recent resource vault creation on the ledger. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**state_account_locker_page_vaults_request** | [**StateAccountLockerPageVaultsRequest**](StateAccountLockerPageVaultsRequest.md) |  | [required] |

### Return type

[**models::StateAccountLockerPageVaultsResponse**](StateAccountLockerPageVaultsResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## account_lockers_touched_at

> models::StateAccountLockersTouchedAtResponse account_lockers_touched_at(state_account_lockers_touched_at_request)
Get Most Recent Touch of Account Lockers

Returns most recent state version given account locker has been touched. Touch refers to the creation of the account locker itself as well as any modification to its contents, such as resource claim, airdrop or store. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**state_account_lockers_touched_at_request** | [**StateAccountLockersTouchedAtRequest**](StateAccountLockersTouchedAtRequest.md) |  | [required] |

### Return type

[**models::StateAccountLockersTouchedAtResponse**](StateAccountLockersTouchedAtResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## account_resource_preferences_page

> models::StateAccountResourcePreferencesPageResponse account_resource_preferences_page(state_account_resource_preferences_page_request)
Get Account resource preferences

Returns paginable collection of resource preference rules for given account. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**state_account_resource_preferences_page_request** | [**StateAccountResourcePreferencesPageRequest**](StateAccountResourcePreferencesPageRequest.md) |  | [required] |

### Return type

[**models::StateAccountResourcePreferencesPageResponse**](StateAccountResourcePreferencesPageResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## entity_fungible_resource_vault_page

> models::StateEntityFungibleResourceVaultsPageResponse entity_fungible_resource_vault_page(state_entity_fungible_resource_vaults_page_request)
Get page of Global Entity Fungible Resource Vaults

Returns vaults for fungible resource owned by a given global entity. The returned response is in a paginated format, ordered by the resource's first appearance on the ledger. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**state_entity_fungible_resource_vaults_page_request** | [**StateEntityFungibleResourceVaultsPageRequest**](StateEntityFungibleResourceVaultsPageRequest.md) |  | [required] |

### Return type

[**models::StateEntityFungibleResourceVaultsPageResponse**](StateEntityFungibleResourceVaultsPageResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## entity_fungibles_page

> models::StateEntityFungiblesPageResponse entity_fungibles_page(state_entity_fungibles_page_request)
Get page of Global Entity Fungible Resource Balances

Returns the total amount of each fungible resource owned by a given global entity. Result can be aggregated globally or per vault. The returned response is in a paginated format, ordered by the resource's first appearance on the ledger. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**state_entity_fungibles_page_request** | [**StateEntityFungiblesPageRequest**](StateEntityFungiblesPageRequest.md) |  | [required] |

### Return type

[**models::StateEntityFungiblesPageResponse**](StateEntityFungiblesPageResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## entity_metadata_page

> models::StateEntityMetadataPageResponse entity_metadata_page(state_entity_metadata_page_request)
Get Entity Metadata Page

Returns all the metadata properties associated with a given global entity. The returned response is in a paginated format, ordered by first appearance on the ledger. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**state_entity_metadata_page_request** | [**StateEntityMetadataPageRequest**](StateEntityMetadataPageRequest.md) |  | [required] |

### Return type

[**models::StateEntityMetadataPageResponse**](StateEntityMetadataPageResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## entity_non_fungible_ids_page

> models::StateEntityNonFungibleIdsPageResponse entity_non_fungible_ids_page(state_entity_non_fungible_ids_page_request)
Get page of Non-Fungibles in Vault

Returns all non-fungible IDs of a given non-fungible resource owned by a given entity. The returned response is in a paginated format, ordered by the resource's first appearence on the ledger. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**state_entity_non_fungible_ids_page_request** | [**StateEntityNonFungibleIdsPageRequest**](StateEntityNonFungibleIdsPageRequest.md) |  | [required] |

### Return type

[**models::StateEntityNonFungibleIdsPageResponse**](StateEntityNonFungibleIdsPageResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## entity_non_fungible_resource_vault_page

> models::StateEntityNonFungibleResourceVaultsPageResponse entity_non_fungible_resource_vault_page(state_entity_non_fungible_resource_vaults_page_request)
Get page of Global Entity Non-Fungible Resource Vaults

Returns vaults for non fungible resource owned by a given global entity. The returned response is in a paginated format, ordered by the resource's first appearance on the ledger. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**state_entity_non_fungible_resource_vaults_page_request** | [**StateEntityNonFungibleResourceVaultsPageRequest**](StateEntityNonFungibleResourceVaultsPageRequest.md) |  | [required] |

### Return type

[**models::StateEntityNonFungibleResourceVaultsPageResponse**](StateEntityNonFungibleResourceVaultsPageResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## entity_non_fungibles_page

> models::StateEntityNonFungiblesPageResponse entity_non_fungibles_page(state_entity_non_fungibles_page_request)
Get page of Global Entity Non-Fungible Resource Balances

Returns the total amount of each non-fungible resource owned by a given global entity. Result can be aggregated globally or per vault. The returned response is in a paginated format, ordered by the resource's first appearance on the ledger. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**state_entity_non_fungibles_page_request** | [**StateEntityNonFungiblesPageRequest**](StateEntityNonFungiblesPageRequest.md) |  | [required] |

### Return type

[**models::StateEntityNonFungiblesPageResponse**](StateEntityNonFungiblesPageResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## entity_schema_page

> models::StateEntitySchemaPageResponse entity_schema_page(state_entity_schema_page_request)
Get Entity Schema Page

Returns all the schemas associated with a given global entity. The returned response is in a paginated format, ordered by first appearance on the ledger. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**state_entity_schema_page_request** | [**StateEntitySchemaPageRequest**](StateEntitySchemaPageRequest.md) |  | [required] |

### Return type

[**models::StateEntitySchemaPageResponse**](StateEntitySchemaPageResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## key_value_store_data

> models::StateKeyValueStoreDataResponse key_value_store_data(state_key_value_store_data_request)
Get KeyValueStore Data

Returns data (value) associated with a given key of a given key-value store. [Check detailed documentation for explanation](#section/How-to-query-the-content-of-a-key-value-store-inside-a-component) 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**state_key_value_store_data_request** | [**StateKeyValueStoreDataRequest**](StateKeyValueStoreDataRequest.md) |  | [required] |

### Return type

[**models::StateKeyValueStoreDataResponse**](StateKeyValueStoreDataResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## key_value_store_keys

> models::StateKeyValueStoreKeysResponse key_value_store_keys(state_key_value_store_keys_request)
Get KeyValueStore Keys

Allows to iterate over key value store keys.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**state_key_value_store_keys_request** | [**StateKeyValueStoreKeysRequest**](StateKeyValueStoreKeysRequest.md) |  | [required] |

### Return type

[**models::StateKeyValueStoreKeysResponse**](StateKeyValueStoreKeysResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## non_fungible_data

> models::StateNonFungibleDataResponse non_fungible_data(state_non_fungible_data_request)
Get Non-Fungible Data

Returns data associated with a given non-fungible ID of a given non-fungible resource. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**state_non_fungible_data_request** | [**StateNonFungibleDataRequest**](StateNonFungibleDataRequest.md) |  | [required] |

### Return type

[**models::StateNonFungibleDataResponse**](StateNonFungibleDataResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## non_fungible_ids

> models::StateNonFungibleIdsResponse non_fungible_ids(state_non_fungible_ids_request)
Get page of Non-Fungible Ids in Resource Collection

Returns the non-fungible IDs of a given non-fungible resource. Returned response is in a paginated format, ordered by their first appearance on the ledger. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**state_non_fungible_ids_request** | [**StateNonFungibleIdsRequest**](StateNonFungibleIdsRequest.md) |  | [required] |

### Return type

[**models::StateNonFungibleIdsResponse**](StateNonFungibleIdsResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## non_fungible_location

> models::StateNonFungibleLocationResponse non_fungible_location(state_non_fungible_location_request)
Get Non-Fungible Location

Returns location of a given non-fungible ID. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**state_non_fungible_location_request** | [**StateNonFungibleLocationRequest**](StateNonFungibleLocationRequest.md) |  | [required] |

### Return type

[**models::StateNonFungibleLocationResponse**](StateNonFungibleLocationResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## package_blueprint_page

> models::StatePackageBlueprintPageResponse package_blueprint_page(state_package_blueprint_page_request)
Get Package Blueprints Page

Returns all the blueprints associated with a given package entity. The returned response is in a paginated format, ordered by first appearance on the ledger. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**state_package_blueprint_page_request** | [**StatePackageBlueprintPageRequest**](StatePackageBlueprintPageRequest.md) |  | [required] |

### Return type

[**models::StatePackageBlueprintPageResponse**](StatePackageBlueprintPageResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## package_code_page

> models::StatePackageCodePageResponse package_code_page(state_package_code_page_request)
Get Package Codes Page

Returns all the codes associated with a given package entity. The returned response is in a paginated format, ordered by first appearance on the ledger. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**state_package_code_page_request** | [**StatePackageCodePageRequest**](StatePackageCodePageRequest.md) |  | [required] |

### Return type

[**models::StatePackageCodePageResponse**](StatePackageCodePageResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## state_entity_details

> models::StateEntityDetailsResponse state_entity_details(state_entity_details_request)
Get Entity Details

Returns detailed information for collection of entities. Aggregate resources globally by default. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**state_entity_details_request** | [**StateEntityDetailsRequest**](StateEntityDetailsRequest.md) |  | [required] |

### Return type

[**models::StateEntityDetailsResponse**](StateEntityDetailsResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## state_validators_list

> models::StateValidatorsListResponse state_validators_list(state_validators_list_request)
Get Validators List

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**state_validators_list_request** | [**StateValidatorsListRequest**](StateValidatorsListRequest.md) |  | [required] |

### Return type

[**models::StateValidatorsListResponse**](StateValidatorsListResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

