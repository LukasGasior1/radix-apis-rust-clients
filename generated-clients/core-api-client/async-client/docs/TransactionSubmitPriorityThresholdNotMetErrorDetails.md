# TransactionSubmitPriorityThresholdNotMetErrorDetails

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**min_tip_percentage_required** | Option<**u32**> | A lower bound for tip percentage at current mempool state. Anything lower than this will very likely result in a mempool rejection. A missing value means there is no tip that can guarantee submission.  | [optional]
**tip_percentage** | **u32** | Tip percentage of the submitted (and rejected) transaction.  | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


