# LeaderProposalHistory

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**current_leader** | [**models::ActiveValidatorIndex**](ActiveValidatorIndex.md) | The leader of the concluded round. | 
**gap_round_leaders** | [**Vec<models::ActiveValidatorIndex>**](ActiveValidatorIndex.md) | The validators which were leaders of the \"gap\" rounds (i.e. since the previous `RoundUpdateValidatorTransaction` - which means that this list will contain exactly `current.round - previous.round - 1` elements). The validators on this list should be penalized during emissions at the end of the epoch. | 
**is_fallback** | **bool** | Whether the concluded round was conducted in a \"fallback\" mode (i.e. indicating a fault of the current leader). When `true`, the `current_leader` should be penalized during emissions in the same way as `gap_round_leaders`. When `false`, the `current_leader` is considered to have made this round's proposal successfully. | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


