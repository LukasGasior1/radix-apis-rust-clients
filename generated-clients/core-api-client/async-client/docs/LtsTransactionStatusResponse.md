# LtsTransactionStatusResponse

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**committed_state_version** | Option<**u64**> |  | [optional]
**intent_status** | [**models::LtsTransactionIntentStatus**](LtsTransactionIntentStatus.md) |  | 
**invalid_from_epoch** | Option<**u64**> | An integer between `0` and `10^10`, marking the epoch from which the transaction will no longer be valid, and be permanently rejected. Only present if the intent status is InMempool or Unknown and we know about a payload.  | [optional]
**known_payloads** | [**Vec<models::LtsTransactionPayloadDetails>**](LtsTransactionPayloadDetails.md) |  | 
**status_description** | **String** | An explanation as to why the intent status is resolved as it is.  | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


