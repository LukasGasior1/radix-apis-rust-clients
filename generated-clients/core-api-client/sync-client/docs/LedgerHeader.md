# LedgerHeader

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**consensus_parent_round_timestamp_ms** | **u64** | An integer between `0` and `10^14`, marking the consensus parent round timestamp in ms. | 
**epoch** | **u64** | An integer between `0` and `10^10`, marking the epoch. | 
**hashes** | [**models::LedgerHashes**](LedgerHashes.md) |  | 
**next_epoch** | Option<[**models::NextEpoch**](NextEpoch.md)> |  | [optional]
**next_protocol_version** | Option<**String**> | If present, indicates that this proof triggers the enactment of the given protocol version. | [optional]
**proposer_timestamp_ms** | **u64** | An integer between `0` and `10^14`, marking the proposer timestamp in ms. | 
**round** | **u64** | An integer between `0` and `10^10`, marking the current round in an epoch | 
**state_version** | **u64** |  | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


