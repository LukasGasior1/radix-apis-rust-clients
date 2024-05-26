# LtsStateAccountAllFungibleResourceBalancesResponse

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**account_address** | **String** | The Bech32m-encoded human readable version of the account's address | 
**fungible_resource_balances** | [**Vec<models::LtsFungibleResourceBalance>**](LtsFungibleResourceBalance.md) | A list containing all resource balances for the requested account. | 
**ledger_header_summary** | [**models::LedgerHeaderSummary**](LedgerHeaderSummary.md) | The excerpt from the ledger header committed at the `state_version`. | 
**state_version** | **u64** |  | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


