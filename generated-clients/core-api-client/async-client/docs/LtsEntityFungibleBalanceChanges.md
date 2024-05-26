# LtsEntityFungibleBalanceChanges

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**entity_address** | **String** | The Bech32m-encoded human readable version of the entity's address | 
**fee_balance_change** | Option<[**models::LtsFungibleResourceBalanceChange**](LtsFungibleResourceBalanceChange.md)> | If present, this field indicates the entity contributed to the payment of the fee. The change in balance will always be negative. NOTE: This property is deprecated but kept for backwards compatibility. This entry is duplicated in  `fee_balance_changes`.  | [optional]
**fee_balance_changes** | [**Vec<models::LtsFeeFungibleResourceBalanceChange>**](LtsFeeFungibleResourceBalanceChange.md) | If present, this field indicates fee-related balance changes, for example:  - Payment of the fee (including tip and royalty) - Distribution of royalties - Distribution of the fee and tip to the consensus-manager, for distributing to the relevant   validator/s at end of epoch  See https://www.radixdlt.com/blog/how-fees-work-in-babylon for further information on how fee payment works at Babylon.  | 
**non_fee_balance_changes** | [**Vec<models::LtsFungibleResourceBalanceChange>**](LtsFungibleResourceBalanceChange.md) |  | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


