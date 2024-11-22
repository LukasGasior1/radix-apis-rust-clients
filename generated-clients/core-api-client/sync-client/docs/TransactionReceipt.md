# TransactionReceipt

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**costing_parameters** | [**models::CostingParameters**](CostingParameters.md) |  | 
**error_message** | Option<**String**> | The error message. This property is only present if the status is `Failed` or `Rejected`.  | [optional]
**events** | Option<[**Vec<models::Event>**](Event.md)> |  | [optional]
**fee_destination** | Option<[**models::FeeDestination**](FeeDestination.md)> | Only present if the `status` is not `Rejected`. | [optional]
**fee_source** | Option<[**models::FeeSource**](FeeSource.md)> | Only present if the `status` is not `Rejected`. | [optional]
**fee_summary** | [**models::FeeSummary**](FeeSummary.md) |  | 
**next_epoch** | Option<[**models::NextEpoch**](NextEpoch.md)> |  | [optional]
**output** | Option<[**Vec<models::SborData>**](SborData.md)> | The return data for each line of the transaction intent's manifest. This property is only present if the `status` is `Succeeded`.  | [optional]
**state_updates** | [**models::StateUpdates**](StateUpdates.md) |  | 
**status** | [**models::TransactionStatus**](TransactionStatus.md) |  | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


