# PlaintextTransactionMessage

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**r#type** | [**models::TransactionMessageType**](TransactionMessageType.md) |  | 
**content** | [**models::PlaintextMessageContent**](PlaintextMessageContent.md) |  | 
**mime_type** | **String** | Intended to represent the RFC 2046 MIME type of the `content`. A client cannot trust that this field is a valid mime type - in particular, the choice between `String` or `Binary` representation of the content is not enforced by this `mime_type`.  | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


