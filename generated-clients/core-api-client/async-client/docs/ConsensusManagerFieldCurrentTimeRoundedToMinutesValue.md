# ConsensusManagerFieldCurrentTimeRoundedToMinutesValue

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**proposer_timestamp_rounded_down_to_minute** | [**models::InstantMs**](InstantMs.md) | The latest round proposer's timestamp (rounded down to the current minute). An honest quorum of validators keeps this aligned with wall-clock time, and non-decreasing.  Note: in abnormal cases (e.g. Byzantine network quorum), this on-ledger field may be set to an arbitrary, extreme value allowed by 64-bit signed integer. The API will still clamp the timestamp to `0 <= ms <= 100000000000000 (== 10^14)`, which translates to `1970-01-01T00:00:00.000Z <= t <= 5138-11-16T09:46:40.000Z`.  | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


