# LedgerHeaderSummary

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**epoch_round** | [**models::EpochRound**](EpochRound.md) |  | 
**ledger_hashes** | [**models::LedgerHashes**](LedgerHashes.md) |  | 
**proposer_timestamp** | [**models::InstantMs**](InstantMs.md) | The time at which the consensus leader created the proposal for extending the ledger to the represented point.  Note: in abnormal cases (e.g. Byzantine network quorum), this on-ledger field may be set to an arbitrary, extreme value allowed by 64-bit signed integer. The API will still clamp the timestamp to `0 <= ms <= 100000000000000 (== 10^14)`, which translates to `1970-01-01T00:00:00.000Z <= t <= 5138-11-16T09:46:40.000Z`.  | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


