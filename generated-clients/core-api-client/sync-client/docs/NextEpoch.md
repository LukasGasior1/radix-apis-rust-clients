# NextEpoch

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**epoch** | **u64** | An integer between `0` and `10^10`, marking the new epoch | 
**significant_protocol_update_readiness** | Option<[**Vec<models::SignificantProtocolUpdateReadinessEntry>**](SignificantProtocolUpdateReadinessEntry.md)> |  | [optional]
**validators** | [**Vec<models::ActiveValidator>**](ActiveValidator.md) | Active validator set for the new epoch, ordered by stake descending. | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


