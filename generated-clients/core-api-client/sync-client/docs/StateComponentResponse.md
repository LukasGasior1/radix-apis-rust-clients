# StateComponentResponse

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**at_ledger_state** | [**models::LedgerStateSummary**](LedgerStateSummary.md) | A summarized state of the ledger at which the query was performed. | 
**descendent_nodes** | [**Vec<models::StateComponentDescendentNode>**](StateComponentDescendentNode.md) | Any descendent nodes owned directly or indirectly by the component | 
**info** | [**models::Substate**](Substate.md) |  | 
**owner_role** | [**models::Substate**](Substate.md) |  | 
**royalty_accumulator** | Option<[**models::Substate**](Substate.md)> |  | [optional]
**state** | [**models::Substate**](Substate.md) |  | 
**vaults** | [**Vec<models::VaultBalance>**](VaultBalance.md) | Any vaults owned directly or indirectly by the component | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


