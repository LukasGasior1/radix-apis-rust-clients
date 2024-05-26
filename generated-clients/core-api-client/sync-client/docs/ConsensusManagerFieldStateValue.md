# ConsensusManagerFieldStateValue

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**actual_epoch_start** | [**models::InstantMs**](InstantMs.md) | The actual time the epoch started. Not used by any logic, but the difference between this and the effective start gives a measure of the time it took for the end-of-epoch to be noticed.  Note: in abnormal cases (e.g. Byzantine network quorum), this on-ledger field may be set to an arbitrary, extreme value allowed by 64-bit signed integer. The API will still clamp the timestamp to `0 <= ms <= 100000000000000 (== 10^14)`, which translates to `1970-01-01T00:00:00.000Z <= t <= 5138-11-16T09:46:40.000Z`.  | 
**current_leader** | Option<[**models::ActiveValidatorIndex**](ActiveValidatorIndex.md)> |  | [optional]
**effective_epoch_start** | [**models::InstantMs**](InstantMs.md) | The effective time the epoch started. A drift-free measure, used to work out when the epoch should ideally end.   Note: in abnormal cases (e.g. Byzantine network quorum), this on-ledger field may be set to an arbitrary, extreme value allowed by 64-bit signed integer. The API will still clamp the timestamp to `0 <= ms <= 100000000000000 (== 10^14)`, which translates to `1970-01-01T00:00:00.000Z <= t <= 5138-11-16T09:46:40.000Z`.  | 
**epoch** | **u64** | An integer between `0` and `10^10`, marking the current epoch | 
**is_started** | **bool** |  | 
**round** | **u64** | An integer between `0` and `10^10`, marking the current round in an epoch | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


