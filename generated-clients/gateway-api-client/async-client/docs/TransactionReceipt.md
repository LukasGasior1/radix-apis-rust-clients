# TransactionReceipt

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**costing_parameters** | Option<[**serde_json::Value**](.md)> |  | [optional]
**error_message** | Option<**String**> | Error message (only present if status is `Failed` or `Rejected`) | [optional]
**events** | Option<[**Vec<models::EventsItem>**](EventsItem.md)> | Events emitted by a transaction. | [optional]
**fee_destination** | Option<[**serde_json::Value**](.md)> | This type is defined in the Core API as `FeeDestination`. See the Core API documentation for more details.  | [optional]
**fee_source** | Option<[**serde_json::Value**](.md)> | This type is defined in the Core API as `FeeSource`. See the Core API documentation for more details.  | [optional]
**fee_summary** | Option<[**serde_json::Value**](.md)> | This type is defined in the Core API as `FeeSummary`. See the Core API documentation for more details.  | [optional]
**next_epoch** | Option<[**serde_json::Value**](.md)> | Information (number and active validator list) about new epoch if occured. This type is defined in the Core API as `NextEpoch`. See the Core API documentation for more details.  | [optional]
**output** | Option<[**serde_json::Value**](.md)> | The manifest line-by-line engine return data (only present if `status` is `CommittedSuccess`). This type is defined in the Core API as `SborData`. See the Core API documentation for more details.  | [optional]
**state_updates** | Option<[**serde_json::Value**](.md)> | This type is defined in the Core API as `StateUpdates`. See the Core API documentation for more details.  | [optional]
**status** | Option<[**models::TransactionStatus**](TransactionStatus.md)> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


