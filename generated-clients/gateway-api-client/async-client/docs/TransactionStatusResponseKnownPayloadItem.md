# TransactionStatusResponseKnownPayloadItem

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**error_message** | Option<**String**> | The initial error message received for a rejection or failure during transaction execution. This will typically be the useful error message, explaining the root cause of the issue. Please note that presence of an error message doesn't imply that this payload will definitely reject or fail. This could represent an error during a temporary rejection (such as out of fees) which then gets resolved (e.g. by depositing money to pay the fee), allowing the transaction to be committed.  | [optional]
**handling_status** | Option<[**models::TransactionPayloadGatewayHandlingStatus**](TransactionPayloadGatewayHandlingStatus.md)> |  | [optional]
**handling_status_reason** | Option<**String**> | Additional reason for why the Gateway has its current handling status.  | [optional]
**latest_error_message** | Option<**String**> | The latest error message received for a rejection or failure during transaction execution, this is only returned if it is different from the initial error message. This is more current than the initial error message, but may be less useful, as it could be a message regarding the expiry of the transaction at the end of its epoch validity window. Please note that presence of an error message doesn't imply that this payload will definitely reject or fail. This could represent an error during a temporary rejection (such as out of fees) which then gets resolved (e.g. by depositing money to pay the fee), allowing the transaction to be committed.  | [optional]
**payload_hash** | **String** | Bech32m-encoded hash. | 
**payload_status** | Option<[**models::TransactionPayloadStatus**](TransactionPayloadStatus.md)> |  | [optional]
**payload_status_description** | Option<**String**> | An additional description to clarify the payload status.  | [optional]
**status** | [**models::TransactionStatus**](TransactionStatus.md) |  | 
**submission_error** | Option<**String**> | The most recent error message received when submitting this transaction to the network. Please note that the presence of an error message doesn't imply that this transaction payload will definitely reject or fail. This could be a transient error.  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


