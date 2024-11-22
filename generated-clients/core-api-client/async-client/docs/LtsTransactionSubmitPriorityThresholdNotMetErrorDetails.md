# LtsTransactionSubmitPriorityThresholdNotMetErrorDetails

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**min_tip_percentage_required** | Option<**u32**> | NOTE: This is kept for backwards compatibility, but we recommend using `min_tip_proportion_required` instead.  A lower bound for tip percentage at current mempool state. Anything lower than this will very likely result in a mempool rejection. A missing value means there is no tip that can guarantee submission.  | [optional]
**min_tip_proportion_required** | Option<**String**> | A lower bound for tip proportion at current mempool state. Anything lower than this will very likely result in a mempool rejection. A missing value means there is no tip that can guarantee submission.  | [optional]
**tip_percentage** | **u32** | NOTE: This is kept for backwards compatibility, but we recommend using `tip_proportion` instead.  Tip percentage of the submitted (and rejected) transaction. For V2 transactions specifying basis point tips, the amount is rounded down.  | 
**tip_proportion** | Option<**String**> | The string-encoded decimal tip proportion of the submitted (and rejected) transaction.  This field will always be present on Cuttlefish nodes, but is marked as not-required for Cuttlefish launch, to avoid a dependency on clients to update after the node is updated.  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


