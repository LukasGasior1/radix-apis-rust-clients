# RoundUpdateTransaction

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**epoch** | **u64** | An integer between `0` and `10^10`, marking the epoch.  | 
**leader_proposal_history** | [**models::LeaderProposalHistory**](LeaderProposalHistory.md) | A recent history (i.e. since the previous `RoundUpdateValidatorTransaction`) of consensus round leaders' reliability. Used for validator emissions calculation. | 
**proposer_timestamp** | [**models::InstantMs**](InstantMs.md) | The round proposer's timestamp.  Note: in abnormal cases (e.g. Byzantine network quorum), this on-ledger field may be set to an arbitrary, extreme value allowed by 64-bit signed integer. The API will still clamp the timestamp to `0 <= ms <= 100000000000000 (== 10^14)`, which translates to `1970-01-01T00:00:00.000Z <= t <= 5138-11-16T09:46:40.000Z`.  | 
**round_in_epoch** | **u64** | An integer between `0` and `10^10`, marking the consensus round in the epoch | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


