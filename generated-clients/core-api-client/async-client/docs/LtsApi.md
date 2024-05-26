# \LtsApi

All URIs are relative to *http://localhost:3333/core*

Method | HTTP request | Description
------------- | ------------- | -------------
[**lts_state_account_all_fungible_resource_balances_post**](LtsApi.md#lts_state_account_all_fungible_resource_balances_post) | **POST** /lts/state/account-all-fungible-resource-balances | Get All Account Balances
[**lts_state_account_deposit_behaviour_post**](LtsApi.md#lts_state_account_deposit_behaviour_post) | **POST** /lts/state/account-deposit-behaviour | Get Account Deposit Behaviour
[**lts_state_account_fungible_resource_balance_post**](LtsApi.md#lts_state_account_fungible_resource_balance_post) | **POST** /lts/state/account-fungible-resource-balance | Get Single Account Balance
[**lts_stream_account_transaction_outcomes_post**](LtsApi.md#lts_stream_account_transaction_outcomes_post) | **POST** /lts/stream/account-transaction-outcomes | Get Account Transaction Outcomes
[**lts_stream_transaction_outcomes_post**](LtsApi.md#lts_stream_transaction_outcomes_post) | **POST** /lts/stream/transaction-outcomes | Get Transaction Outcomes
[**lts_transaction_construction_post**](LtsApi.md#lts_transaction_construction_post) | **POST** /lts/transaction/construction | Get Construction Metadata
[**lts_transaction_status_post**](LtsApi.md#lts_transaction_status_post) | **POST** /lts/transaction/status | Get Transaction Status
[**lts_transaction_submit_post**](LtsApi.md#lts_transaction_submit_post) | **POST** /lts/transaction/submit | Submit Transaction



## lts_state_account_all_fungible_resource_balances_post

> models::LtsStateAccountAllFungibleResourceBalancesResponse lts_state_account_all_fungible_resource_balances_post(lts_state_account_all_fungible_resource_balances_request)
Get All Account Balances

Returns balances for all resources associated with an account

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**lts_state_account_all_fungible_resource_balances_request** | [**LtsStateAccountAllFungibleResourceBalancesRequest**](LtsStateAccountAllFungibleResourceBalancesRequest.md) |  | [required] |

### Return type

[**models::LtsStateAccountAllFungibleResourceBalancesResponse**](LtsStateAccountAllFungibleResourceBalancesResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## lts_state_account_deposit_behaviour_post

> models::LtsStateAccountDepositBehaviourResponse lts_state_account_deposit_behaviour_post(lts_state_account_deposit_behaviour_request)
Get Account Deposit Behaviour

Returns deposit behaviour of a single account for multiple resource addresses

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**lts_state_account_deposit_behaviour_request** | [**LtsStateAccountDepositBehaviourRequest**](LtsStateAccountDepositBehaviourRequest.md) |  | [required] |

### Return type

[**models::LtsStateAccountDepositBehaviourResponse**](LtsStateAccountDepositBehaviourResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## lts_state_account_fungible_resource_balance_post

> models::LtsStateAccountFungibleResourceBalanceResponse lts_state_account_fungible_resource_balance_post(lts_state_account_fungible_resource_balance_request)
Get Single Account Balance

Returns balance of a single fungible resource in an account

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**lts_state_account_fungible_resource_balance_request** | [**LtsStateAccountFungibleResourceBalanceRequest**](LtsStateAccountFungibleResourceBalanceRequest.md) |  | [required] |

### Return type

[**models::LtsStateAccountFungibleResourceBalanceResponse**](LtsStateAccountFungibleResourceBalanceResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## lts_stream_account_transaction_outcomes_post

> models::LtsStreamAccountTransactionOutcomesResponse lts_stream_account_transaction_outcomes_post(lts_stream_account_transaction_outcomes_request)
Get Account Transaction Outcomes

Returns a list of committed transaction outcomes (containing balance changes) from a given state version, filtered to only transactions which involved the given account. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**lts_stream_account_transaction_outcomes_request** | [**LtsStreamAccountTransactionOutcomesRequest**](LtsStreamAccountTransactionOutcomesRequest.md) |  | [required] |

### Return type

[**models::LtsStreamAccountTransactionOutcomesResponse**](LtsStreamAccountTransactionOutcomesResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## lts_stream_transaction_outcomes_post

> models::LtsStreamTransactionOutcomesResponse lts_stream_transaction_outcomes_post(lts_stream_transaction_outcomes_request)
Get Transaction Outcomes

Returns a list of committed transaction outcomes (containing balance changes) from a given state version. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**lts_stream_transaction_outcomes_request** | [**LtsStreamTransactionOutcomesRequest**](LtsStreamTransactionOutcomesRequest.md) |  | [required] |

### Return type

[**models::LtsStreamTransactionOutcomesResponse**](LtsStreamTransactionOutcomesResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## lts_transaction_construction_post

> models::LtsTransactionConstructionResponse lts_transaction_construction_post(lts_transaction_construction_request)
Get Construction Metadata

Returns information necessary to build a transaction

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**lts_transaction_construction_request** | [**LtsTransactionConstructionRequest**](LtsTransactionConstructionRequest.md) |  | [required] |

### Return type

[**models::LtsTransactionConstructionResponse**](LtsTransactionConstructionResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## lts_transaction_status_post

> models::LtsTransactionStatusResponse lts_transaction_status_post(lts_transaction_status_request)
Get Transaction Status

Shares the node's knowledge of any payloads associated with the given intent hash. Generally there will be a single payload for a given intent, but it's theoretically possible there may be multiple. This knowledge is summarised into a status for the intent. This summarised status in the response is likely sufficient for most clients. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**lts_transaction_status_request** | [**LtsTransactionStatusRequest**](LtsTransactionStatusRequest.md) |  | [required] |

### Return type

[**models::LtsTransactionStatusResponse**](LtsTransactionStatusResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## lts_transaction_submit_post

> models::LtsTransactionSubmitResponse lts_transaction_submit_post(lts_transaction_submit_request)
Submit Transaction

Submits a notarized transaction to the network. Returns whether the transaction submission was already included in the node's mempool. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**lts_transaction_submit_request** | [**LtsTransactionSubmitRequest**](LtsTransactionSubmitRequest.md) |  | [required] |

### Return type

[**models::LtsTransactionSubmitResponse**](LtsTransactionSubmitResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

