# StateConsensusManagerResponse

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**at_ledger_state** | [**models::LedgerStateSummary**](LedgerStateSummary.md) | A summarized state of the ledger at which the query was performed. | 
**config** | [**models::Substate**](Substate.md) |  | 
**current_proposal_statistic** | [**models::Substate**](Substate.md) |  | 
**current_time** | [**models::Substate**](Substate.md) |  | 
**current_time_rounded_to_minutes** | [**models::Substate**](Substate.md) |  | 
**current_validator_readiness_signals** | Option<[**Vec<models::ProtocolVersionReadiness>**](ProtocolVersionReadiness.md)> | Protocol versions signalled by the current validator set. Every validator from `current_validator_set` will be referenced by exactly one of the items here. Only returned if enabled by `include_readiness_signals` on your request.  | [optional]
**current_validator_set** | [**models::Substate**](Substate.md) |  | 
**state** | [**models::Substate**](Substate.md) |  | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


