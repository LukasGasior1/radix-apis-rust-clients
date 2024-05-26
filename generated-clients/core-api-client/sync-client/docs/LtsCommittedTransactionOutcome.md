# LtsCommittedTransactionOutcome

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**accumulator_hash** | **String** | The hex-encoded transaction accumulator hash. This hash captures the order of all transactions on ledger. This hash is `ACC_{N+1} = combine(ACC_N, LEDGER_HASH_{N}))` (where `combine()` is an arbitrary deterministic function we use).  | 
**fungible_entity_balance_changes** | [**Vec<models::LtsEntityFungibleBalanceChanges>**](LtsEntityFungibleBalanceChanges.md) | A list of all fungible balance updates which occurred in this transaction, aggregated by the global entity (such as account) which owns the vaults which were updated.  | 
**non_fungible_entity_balance_changes** | [**Vec<models::LtsEntityNonFungibleBalanceChanges>**](LtsEntityNonFungibleBalanceChanges.md) | Non fungible changes per entity and resource  | 
**proposer_timestamp_ms** | **u64** | An integer between `0` and `10^14`, marking the proposer timestamp in ms. | 
**resultant_account_fungible_balances** | [**Vec<models::LtsResultantAccountFungibleBalances>**](LtsResultantAccountFungibleBalances.md) | A list of the resultant fungible account balances for any balances which changed in this transaction. Only balances for accounts are returned, not any other kind of entity.  | 
**state_version** | **u64** |  | 
**status** | [**models::LtsCommittedTransactionStatus**](LtsCommittedTransactionStatus.md) |  | 
**total_fee** | **String** | The string-encoded decimal representing the total amount of XRD paid as fee (execution, validator tip and royalties). A decimal is formed of some signed integer `m` of attos (`10^(-18)`) units, where `-2^(192 - 1) <= m < 2^(192 - 1)`.  | 
**user_transaction_identifiers** | Option<[**models::TransactionIdentifiers**](TransactionIdentifiers.md)> | Only present if the transaction is a user-submitted notarized transaction. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


